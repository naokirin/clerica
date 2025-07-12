use crate::database::{Database, DatabaseTrait, File};
use crate::ShelfManager;
use chrono::Utc;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, State};
use uuid::Uuid;

pub struct FileWatcher {
    watcher: notify::FsEventWatcher,
    watched_directories: HashMap<String, String>, // directory_id -> path
}

impl FileWatcher {
    pub fn new(
        pools: Arc<ShelfManager>,
        app_handle: Option<AppHandle>,
    ) -> Result<Self, notify::Error> {
        let (tx, rx) = mpsc::channel();
        let pools_clone = Arc::clone(&pools);

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let mut last_event_time = Instant::now();
            let mut event_queue: Vec<Event> = Vec::new();
            let debounce_duration = Duration::from_millis(300); // デバウンス時間を少し長く
            const MAX_QUEUE_SIZE: usize = 1000; // キューの最大サイズを制限

            loop {
                // イベントをキューにためる（最大サイズ制限）
                while event_queue.len() < MAX_QUEUE_SIZE {
                    if let Ok(event) = rx.try_recv() {
                        event_queue.push(event);
                        last_event_time = Instant::now();
                    } else {
                        break;
                    }
                }

                // キューが満杯の場合は強制的に処理
                let should_process = !event_queue.is_empty() && 
                    (last_event_time.elapsed() > debounce_duration || event_queue.len() >= MAX_QUEUE_SIZE);

                if should_process {
                    // イベントの重複を排除
                    let mut processed_paths = HashSet::new();
                    let mut unique_events = Vec::new();

                    // 最新のイベントから処理するため、逆順で重複チェック
                    for event in event_queue.drain(..).rev() {
                        if let Some(path) = event.paths.first() {
                            // 同じパスに対するイベントは最新のものだけを処理
                            if processed_paths.insert(path.clone()) {
                                unique_events.push(event);
                            }
                        }
                    }

                    // バッチサイズを制限（一度に処理するイベント数を制限）
                    const BATCH_SIZE: usize = 50;
                    let batches: Vec<_> = unique_events.chunks(BATCH_SIZE).collect();
                    let batch_count = batches.len();
                    
                    for batch in batches {
                        // 各バッチを元の順序に戻して処理
                        for event in batch.iter().rev() {
                            // データベースロック競合を避けるため、各イベント間に少し待機
                            if let Err(e) = rt.block_on(async {
                                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                                handle_file_event(&pools_clone, event.clone(), app_handle.clone()).await
                            }) {
                                eprintln!("ファイルイベント処理エラー: {e}");
                            }
                        }
                        
                        // バッチ間にも少し待機
                        if batch_count > 1 {
                            thread::sleep(Duration::from_millis(100));
                        }
                    }
                }

                // CPU負荷を下げるために少し待つ
                thread::sleep(Duration::from_millis(50));
            }
        });

        let watcher = notify::recommended_watcher(move |res| match res {
            Ok(event) => {
                if let Err(e) = tx.send(event) {
                    eprintln!("ファイルイベント送信エラー: {e}");
                }
            }
            Err(e) => eprintln!("ファイル監視エラー: {e}"),
        })?;

        Ok(FileWatcher {
            watcher,
            watched_directories: HashMap::new(),
        })
    }

    pub fn watch_directory(&mut self, directory_id: &str, path: &str) -> Result<(), notify::Error> {
        self.watcher
            .watch(Path::new(path), RecursiveMode::Recursive)?;
        self.watched_directories
            .insert(directory_id.to_string(), path.to_string());
        Ok(())
    }

    pub fn unwatch_directory(&mut self, directory_id: &str) -> Result<(), notify::Error> {
        if let Some(path) = self.watched_directories.remove(directory_id) {
            self.watcher.unwatch(Path::new(&path))?;
        }
        Ok(())
    }
}

#[tauri::command]
pub async fn start_watching(
    pools: State<'_, ShelfManager>,
    watcher: State<'_, Arc<Mutex<FileWatcher>>>,
    directory_id: String,
    path: String,
) -> Result<(), String> {
    // ディレクトリスキャンを実行
    crate::file_manager::scan_directory(&pools, &directory_id, &path)
        .await
        .map_err(|e| e.to_string())?;

    // ファイル監視を開始
    let mut watcher_guard = watcher.lock().map_err(|e| e.to_string())?;
    watcher_guard
        .watch_directory(&directory_id, &path)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn stop_watching(
    _pools: State<'_, ShelfManager>,
    watcher: State<'_, Arc<Mutex<FileWatcher>>>,
    directory_id: String,
) -> Result<(), String> {
    let mut watcher_guard = watcher.lock().map_err(|e| e.to_string())?;
    watcher_guard
        .unwatch_directory(&directory_id)
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn handle_file_event(
    pools: &ShelfManager,
    event: Event,
    app_handle: Option<AppHandle>,
) -> Result<(), String> {
    match event.kind {
        EventKind::Create(_) => {
            handle_create_event(pools, &event.paths, &app_handle).await
        }
        EventKind::Remove(_) => {
            handle_remove_event(pools, &event.paths, &app_handle).await
        }
        EventKind::Modify(_) => {
            handle_modify_event(pools, &event.paths, &app_handle).await
        }
        _ => {
            // 他のイベントは無視
            Ok(())
        }
    }
}

async fn handle_create_event(
    pools: &ShelfManager,
    paths: &[std::path::PathBuf],
    app_handle: &Option<AppHandle>,
) -> Result<(), String> {
    let db = Database;

    for path in paths {
        
        match fs::metadata(path) {
            Ok(metadata) => {
                let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
                match find_directory_id_for_path(&data_pool, path).await {
                    Ok(directory_id) => {
                        let file = create_file_from_metadata(path, &metadata, &directory_id);
                        
                        match db.add_file(&data_pool, &file).await {
                            Ok(()) => {
                                notify_ui(app_handle, "file_created", &file.path);
                            }
                            Err(e) => eprintln!("ファイル追加エラー: {e}"),
                        }
                    }
                    Err(e) => {
                        eprintln!("ディレクトリID特定エラー: {} (パス: {})", e, path.display());
                    }
                }
            }
            Err(e) => {
                eprintln!("メタデータ取得エラー: {} (パス: {})", e, path.display());
            }
        }
    }

    Ok(())
}

async fn handle_remove_event(
    pools: &ShelfManager,
    paths: &[std::path::PathBuf],
    app_handle: &Option<AppHandle>,
) -> Result<(), String> {
    let db = Database;

    for path in paths {
        let path_str = path.to_string_lossy().to_string();
        
        let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
        match db.remove_file_by_path(&data_pool, &path_str).await {
            Ok(()) => {
                notify_ui(app_handle, "file_deleted", &path_str);
            }
            Err(e) => eprintln!("ファイル削除エラー: {e}"),
        }
    }

    Ok(())
}

async fn handle_modify_event(
    pools: &ShelfManager,
    paths: &[std::path::PathBuf],
    app_handle: &Option<AppHandle>,
) -> Result<(), String> {
    for path in paths {
        
        match fs::metadata(path) {
            Ok(metadata) => {
                handle_modify_with_metadata(pools, path, &metadata, app_handle).await?;
            }
            Err(e) => {
                handle_modify_without_metadata(pools, path, e, app_handle).await?;
            }
        }
    }

    Ok(())
}

async fn handle_modify_with_metadata(
    pools: &ShelfManager,
    path: &Path,
    metadata: &fs::Metadata,
    app_handle: &Option<AppHandle>,
) -> Result<(), String> {
    use std::os::unix::fs::MetadataExt;
    
    let db = Database;
    let path_str = path.to_string_lossy().to_string();
    let inode = metadata.ino() as i64;
    let device_id = Some(metadata.dev() as i64);

    let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
    // パスによる存在確認
    match db.file_exists_by_path(&data_pool, &path_str).await {
        Ok(exists) => {
            if exists {
                handle_existing_file_update(pools, &path_str, metadata, app_handle).await
            } else {
                handle_non_existing_file(pools, path, &path_str, inode, device_id, metadata, app_handle).await
            }
        }
        Err(e) => {
            eprintln!("ファイル存在確認エラー: {e} (パス: {path_str})");
            handle_file_update_fallback(pools, &path_str, metadata, app_handle).await
        }
    }
}

async fn handle_existing_file_update(
    pools: &ShelfManager,
    path_str: &str,
    metadata: &fs::Metadata,
    app_handle: &Option<AppHandle>,
) -> Result<(), String> {
    let db = Database;
    let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
    
    match db.update_file_metadata(&data_pool, path_str, metadata).await {
        Ok(()) => {
            notify_ui(app_handle, "file_modified", path_str);
        }
        Err(e) => eprintln!("ファイル更新エラー: {e}"),
    }

    Ok(())
}

async fn handle_non_existing_file(
    pools: &ShelfManager,
    path: &Path,
    path_str: &str,
    inode: i64,
    device_id: Option<i64>,
    metadata: &fs::Metadata,
    app_handle: &Option<AppHandle>,
) -> Result<(), String> {
    let db = Database;

    let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
    // inode番号で検索（ファイル名変更の可能性）
    match db.find_file_by_inode(&data_pool, inode, device_id).await {
        Ok(Some(existing_file)) => {
            handle_file_rename(pools, path, path_str, &existing_file, app_handle).await
        }
        Ok(None) => {
            handle_new_file_from_move(pools, path, metadata, app_handle).await
        }
        Err(e) => {
            eprintln!("inode検索エラー: {} (パス: {})", e, path.display());
            handle_new_file_from_move(pools, path, metadata, app_handle).await
        }
    }
}

async fn handle_file_rename(
    pools: &ShelfManager,
    path: &Path,
    path_str: &str,
    existing_file: &File,
    app_handle: &Option<AppHandle>,
) -> Result<(), String> {
    let db = Database;
    
    let new_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
    match db.update_file_path(&data_pool, &existing_file.id, path_str, &new_name).await {
        Ok(()) => {
            notify_ui(app_handle, "file_renamed", path_str);
        }
        Err(e) => {
            eprintln!("ファイル名変更更新エラー: {e}")
        }
    }

    Ok(())
}

async fn handle_new_file_from_move(
    pools: &ShelfManager,
    path: &Path,
    metadata: &fs::Metadata,
    app_handle: &Option<AppHandle>,
) -> Result<(), String> {
    let db = Database;

    let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
    match find_directory_id_for_path(&data_pool, path).await {
        Ok(directory_id) => {
            let file = create_file_from_metadata(path, metadata, &directory_id);
            
            match db.add_file(&data_pool, &file).await {
                Ok(()) => {
                    notify_ui(app_handle, "file_created", &file.path);
                }
                Err(e) => {
                    eprintln!("移動ファイル追加エラー: {e}")
                }
            }
        }
        Err(e) => {
            eprintln!("移動ファイルのディレクトリID特定エラー: {} (パス: {})", e, path.display());
        }
    }

    Ok(())
}

async fn handle_file_update_fallback(
    pools: &ShelfManager,
    path_str: &str,
    metadata: &fs::Metadata,
    app_handle: &Option<AppHandle>,
) -> Result<(), String> {
    let db = Database;
    
    let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
    // エラーの場合は更新を試行
    match db.update_file_metadata(&data_pool, path_str, metadata).await {
        Ok(()) => {
            notify_ui(app_handle, "file_modified", path_str);
        }
        Err(e) => {
            eprintln!("ファイル更新エラー（存在確認失敗後）: {e}")
        }
    }

    Ok(())
}

async fn handle_modify_without_metadata(
    pools: &ShelfManager,
    path: &Path,
    _error: std::io::Error,
    app_handle: &Option<AppHandle>,
) -> Result<(), String> {
    let db = Database;
    let path_str = path.to_string_lossy().to_string();
    

    let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
    // データベースから該当ファイルを削除
    match db.remove_file_by_path(&data_pool, &path_str).await {
        Ok(()) => {
            notify_ui(app_handle, "file_deleted", &path_str);
        }
        Err(e) => eprintln!("移動ファイル削除エラー: {e}"),
    }

    Ok(())
}

fn create_file_from_metadata(path: &Path, metadata: &fs::Metadata, directory_id: &str) -> File {
    use std::os::unix::fs::MetadataExt;

    File {
        id: Uuid::new_v4().to_string(),
        path: path.to_string_lossy().to_string(),
        name: path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string(),
        directory_id: directory_id.to_string(),
        size: metadata.len() as i64,
        file_type: path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string()),
        created_at: metadata.created().ok().map(chrono::DateTime::from),
        modified_at: metadata.modified().ok().map(chrono::DateTime::from),
        birth_time: None,
        inode: Some(metadata.ino() as i64),
        is_directory: metadata.is_dir(),
        created_at_db: Utc::now(),
        updated_at_db: Utc::now(),
        file_size: Some(metadata.len() as i64),
        mime_type: infer_mime_type_from_path(path),
        permissions: Some(format!("{:o}", metadata.mode() & 0o777)),
        owner_uid: Some(metadata.uid() as i64),
        group_gid: Some(metadata.gid() as i64),
        hard_links: Some(metadata.nlink() as i64),
        device_id: Some(metadata.dev() as i64),
        last_accessed: metadata.accessed().ok().map(chrono::DateTime::from),
        metadata: None,
    }
}

fn infer_mime_type_from_path(path: &Path) -> Option<String> {
    path.extension().and_then(|ext| ext.to_str()).map(|ext| {
        match ext.to_lowercase().as_str() {
            "txt" => "text/plain",
            "md" => "text/markdown",
            "html" | "htm" => "text/html",
            "css" => "text/css",
            "js" => "text/javascript",
            "json" => "application/json",
            "xml" => "text/xml",
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "pdf" => "application/pdf",
            _ => "application/octet-stream",
        }
        .to_string()
    })
}

fn notify_ui(app_handle: &Option<AppHandle>, event_type: &str, file_path: &str) {
    if let Some(app) = app_handle {
        let payload = serde_json::json!({
            "type": event_type,
            "path": file_path,
            "timestamp": chrono::Utc::now().timestamp()
        });

        if let Err(e) = app.emit("file_system_change", payload) {
            eprintln!("UI通知エラー: {e}");
        }
    }
}

async fn find_directory_id_for_path(pool: &SqlitePool, path: &Path) -> Result<String, String> {
    let db = Database;
    let directories = db.get_directories(pool).await.map_err(|e| e.to_string())?;

    for directory in directories {
        if path.starts_with(&directory.path) {
            return Ok(directory.id);
        }
    }

    Err("対応するディレクトリが見つかりません".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::{event::CreateKind, Event, EventKind};
    use sqlx::SqlitePool;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_file_watcher_creation() {
        let settings_pool = SqlitePool::connect(":memory:").await.unwrap();
        let data_pool = SqlitePool::connect(":memory:").await.unwrap();
        let pools = std::sync::Arc::new(crate::ShelfManager::new(settings_pool).await.unwrap());
        let result = FileWatcher::new(pools, None);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_file_watcher_watch_directory() {
        let settings_pool = SqlitePool::connect(":memory:").await.unwrap();
        let data_pool = SqlitePool::connect(":memory:").await.unwrap();
        let pools = std::sync::Arc::new(crate::ShelfManager::new(settings_pool).await.unwrap());
        let mut watcher = FileWatcher::new(pools, None).unwrap();

        // 存在しないディレクトリをwatch
        let result = watcher.watch_directory("test_id", "/nonexistent/path");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_file_watcher_unwatch_directory() {
        let settings_pool = SqlitePool::connect(":memory:").await.unwrap();
        let data_pool = SqlitePool::connect(":memory:").await.unwrap();
        let pools = std::sync::Arc::new(crate::ShelfManager::new(settings_pool).await.unwrap());
        let mut watcher = FileWatcher::new(pools, None).unwrap();

        // 存在しないディレクトリをunwatch
        let result = watcher.unwatch_directory("test_id");
        assert!(result.is_ok()); // unwatchは存在しないディレクトリでもエラーにならない
    }

    // Note: Tauriコマンドのテストは実際のTauri環境でのみ可能

    #[tokio::test]
    async fn test_handle_file_event_create() {
        let settings_pool = SqlitePool::connect(":memory:").await.unwrap();
        let data_pool = SqlitePool::connect(":memory:").await.unwrap();
        let pools = crate::ShelfManager::new(settings_pool).await.unwrap();

        let event = Event {
            kind: EventKind::Create(CreateKind::File),
            paths: vec![PathBuf::from("/test/file.txt")],
            attrs: Default::default(),
        };

        let result = handle_file_event(&pools, event, None).await;
        // メタデータ取得に失敗するためエラーは発生しない（pathが存在しないため）
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_file_event_remove() {
        let settings_pool = SqlitePool::connect(":memory:").await.unwrap();
        let data_pool = SqlitePool::connect(":memory:").await.unwrap();
        let pools = crate::ShelfManager::new(settings_pool).await.unwrap();

        let event = Event {
            kind: EventKind::Remove(notify::event::RemoveKind::File),
            paths: vec![PathBuf::from("/test/file.txt")],
            attrs: Default::default(),
        };

        let result = handle_file_event(&pools, event, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_file_event_modify() {
        let settings_pool = SqlitePool::connect(":memory:").await.unwrap();
        let data_pool = SqlitePool::connect(":memory:").await.unwrap();
        let pools = crate::ShelfManager::new(settings_pool).await.unwrap();

        let event = Event {
            kind: EventKind::Modify(notify::event::ModifyKind::Data(
                notify::event::DataChange::Content,
            )),
            paths: vec![PathBuf::from("/test/file.txt")],
            attrs: Default::default(),
        };

        let result = handle_file_event(&pools, event, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_file_event_other() {
        let settings_pool = SqlitePool::connect(":memory:").await.unwrap();
        let data_pool = SqlitePool::connect(":memory:").await.unwrap();
        let pools = crate::ShelfManager::new(settings_pool).await.unwrap();

        let event = Event {
            kind: EventKind::Access(notify::event::AccessKind::Read),
            paths: vec![PathBuf::from("/test/file.txt")],
            attrs: Default::default(),
        };

        let result = handle_file_event(&pools, event, None).await;
        assert!(result.is_ok());
    }
}

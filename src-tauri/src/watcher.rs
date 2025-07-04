use crate::database::{Database, DatabaseTrait, File};
use sqlx::SqlitePool;
use notify::{Watcher, RecursiveMode, Event, EventKind};
use tauri::{State, AppHandle, Emitter};
use uuid::Uuid;
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use serde_json;

pub struct FileWatcher {
    watcher: notify::FsEventWatcher,
    watched_directories: HashMap<String, String>, // directory_id -> path
}

impl FileWatcher {
    pub fn new(pool: Arc<SqlitePool>, app_handle: Option<AppHandle>) -> Result<Self, notify::Error> {
        let (tx, rx) = mpsc::channel();
        let pool_clone = Arc::clone(&pool);
        
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            while let Ok(event) = rx.recv() {
                if let Err(e) = rt.block_on(handle_file_event(&pool_clone, event, app_handle.clone())) {
                    eprintln!("ファイルイベント処理エラー: {}", e);
                }
            }
        });
        
        let watcher = notify::recommended_watcher(move |res| {
            match res {
                Ok(event) => {
                    if let Err(e) = tx.send(event) {
                        eprintln!("ファイルイベント送信エラー: {}", e);
                    }
                }
                Err(e) => eprintln!("ファイル監視エラー: {}", e),
            }
        })?;
        
        Ok(FileWatcher {
            watcher,
            watched_directories: HashMap::new(),
        })
    }
    
    pub fn watch_directory(&mut self, directory_id: &str, path: &str) -> Result<(), notify::Error> {
        self.watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
        self.watched_directories.insert(directory_id.to_string(), path.to_string());
        Ok(())
    }
    
    pub fn unwatch_directory(&mut self, directory_id: &str) -> Result<(), notify::Error> {
        if let Some(path) = self.watched_directories.remove(directory_id) {
            self.watcher.unwatch(Path::new(&path))?;
        }
        Ok(())
    }
    
    pub fn get_directory_id_for_path(&self, path: &Path) -> Option<String> {
        for (dir_id, dir_path) in &self.watched_directories {
            if path.starts_with(dir_path) {
                return Some(dir_id.clone());
            }
        }
        None
    }
}

#[tauri::command]
pub async fn start_watching(
    pool: State<'_, SqlitePool>,
    watcher: State<'_, Arc<Mutex<FileWatcher>>>,
    directory_id: String,
    path: String,
) -> Result<(), String> {
    // ディレクトリスキャンを実行
    crate::file_manager::scan_directory(&pool, &directory_id, &path)
        .await
        .map_err(|e| e.to_string())?;
    
    // ファイル監視を開始
    let mut watcher_guard = watcher.lock().map_err(|e| e.to_string())?;
    watcher_guard.watch_directory(&directory_id, &path).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn stop_watching(
    _pool: State<'_, SqlitePool>,
    watcher: State<'_, Arc<Mutex<FileWatcher>>>,
    directory_id: String,
) -> Result<(), String> {
    let mut watcher_guard = watcher.lock().map_err(|e| e.to_string())?;
    watcher_guard.unwatch_directory(&directory_id).map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn handle_file_event(
    pool: &SqlitePool,
    event: Event,
    app_handle: Option<AppHandle>,
) -> Result<(), String> {
    let db = Database;
    
    match event.kind {
        EventKind::Create(_) => {
            for path in &event.paths {
                println!("ファイル作成検知: {}", path.display());
                match fs::metadata(&path) {
                    Ok(metadata) => {
                        match find_directory_id_for_path(pool, &path).await {
                            Ok(directory_id) => {
                                let file = create_file_from_metadata(&path, &metadata, &directory_id);
                                
                                match db.add_file(pool, &file).await {
                                    Ok(()) => {
                                        notify_ui(&app_handle, "file_created", &file.path);
                                    },
                                    Err(e) => eprintln!("ファイル追加エラー: {}", e),
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
        },
        EventKind::Remove(_) => {
            for path in &event.paths {
                let path_str = path.to_string_lossy().to_string();
                println!("ファイル削除検知: {}", path_str);
                match db.remove_file_by_path(pool, &path_str).await {
                    Ok(()) => {
                        notify_ui(&app_handle, "file_deleted", &path_str);
                    },
                    Err(e) => eprintln!("ファイル削除エラー: {}", e),
                }
            }
        },
        EventKind::Modify(_) => {
            for path in &event.paths {
                println!("ファイル変更検知: {}", path.display());
                match fs::metadata(&path) {
                    Ok(metadata) => {
                        use std::os::unix::fs::MetadataExt;
                        let path_str = path.to_string_lossy().to_string();
                        let inode = metadata.ino() as i64;
                        let device_id = Some(metadata.dev() as i64);
                        
                        // パスによる存在確認
                        match db.file_exists_by_path(pool, &path_str).await {
                            Ok(exists) => {
                                if exists {
                                    // 既存ファイルの更新
                                    match db.update_file_metadata(pool, &path_str, &metadata).await {
                                        Ok(()) => {
                                            notify_ui(&app_handle, "file_modified", &path_str);
                                        },
                                        Err(e) => eprintln!("ファイル更新エラー: {}", e),
                                    }
                                } else {
                                    // パスに存在しない場合、inode番号で検索（ファイル名変更の可能性）
                                    match db.find_file_by_inode(pool, inode, device_id).await {
                                        Ok(Some(existing_file)) => {
                                            // ファイル名変更として処理
                                            let new_name = path.file_name()
                                                .and_then(|n| n.to_str())
                                                .unwrap_or("unknown")
                                                .to_string();
                                            
                                            match db.update_file_path(pool, &existing_file.id, &path_str, &new_name).await {
                                                Ok(()) => {
                                                    println!("ファイル名変更検知: {} -> {}", existing_file.path, path_str);
                                                    notify_ui(&app_handle, "file_renamed", &path_str);
                                                },
                                                Err(e) => eprintln!("ファイル名変更更新エラー: {}", e),
                                            }
                                        }
                                        Ok(None) => {
                                            // ディレクトリ外からの移動による新規ファイル
                                            match find_directory_id_for_path(pool, &path).await {
                                                Ok(directory_id) => {
                                                    let file = create_file_from_metadata(&path, &metadata, &directory_id);
                                                    match db.add_file(pool, &file).await {
                                                        Ok(()) => {
                                                            notify_ui(&app_handle, "file_created", &file.path);
                                                        },
                                                        Err(e) => eprintln!("移動ファイル追加エラー: {}", e),
                                                    }
                                                }
                                                Err(e) => {
                                                    eprintln!("移動ファイルのディレクトリID特定エラー: {} (パス: {})", e, path.display());
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("inode検索エラー: {} (パス: {})", e, path.display());
                                            // エラーの場合は新規ファイルとして処理
                                            match find_directory_id_for_path(pool, &path).await {
                                                Ok(directory_id) => {
                                                    let file = create_file_from_metadata(&path, &metadata, &directory_id);
                                                    match db.add_file(pool, &file).await {
                                                        Ok(()) => {
                                                            notify_ui(&app_handle, "file_created", &file.path);
                                                        },
                                                        Err(e) => eprintln!("移動ファイル追加エラー（inode検索失敗後）: {}", e),
                                                    }
                                                }
                                                Err(e) => {
                                                    eprintln!("移動ファイルのディレクトリID特定エラー（inode検索失敗後）: {} (パス: {})", e, path.display());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("ファイル存在確認エラー: {} (パス: {})", e, path_str);
                                // エラーの場合は更新を試行
                                match db.update_file_metadata(pool, &path_str, &metadata).await {
                                    Ok(()) => {
                                        notify_ui(&app_handle, "file_modified", &path_str);
                                    },
                                    Err(e) => eprintln!("ファイル更新エラー（存在確認失敗後）: {}", e),
                                }
                            }
                        }
                    }
                    Err(e) => {
                        // ファイルが存在しない場合（ディレクトリ外への移動）
                        let path_str = path.to_string_lossy().to_string();
                        println!("ファイル不存在によるメタデータ取得エラー: {} (パス: {})", e, path.display());
                        
                        // データベースから該当ファイルを削除
                        match db.remove_file_by_path(pool, &path_str).await {
                            Ok(()) => {
                                println!("ディレクトリ外移動によりファイルレコード削除: {}", path_str);
                                notify_ui(&app_handle, "file_deleted", &path_str);
                            },
                            Err(e) => eprintln!("移動ファイル削除エラー: {}", e),
                        }
                    }
                }
            }
        },
        _ => {
            // 他のイベントは無視
        },
    }
    
    Ok(())
}

fn create_file_from_metadata(path: &PathBuf, metadata: &fs::Metadata, directory_id: &str) -> File {
    use std::os::unix::fs::MetadataExt;
    
    File {
        id: Uuid::new_v4().to_string(),
        path: path.to_string_lossy().to_string(),
        name: path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string(),
        directory_id: directory_id.to_string(),
        size: metadata.len() as i64,
        file_type: path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string()),
        created_at: metadata.created()
            .ok()
            .map(chrono::DateTime::from),
        modified_at: metadata.modified()
            .ok()
            .map(chrono::DateTime::from),
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
        last_accessed: metadata.accessed()
            .ok()
            .map(chrono::DateTime::from),
        metadata: None,
    }
}

fn infer_mime_type_from_path(path: &PathBuf) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
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
            eprintln!("UI通知エラー: {}", e);
        }
    }
}

async fn find_directory_id_for_path(pool: &SqlitePool, path: &PathBuf) -> Result<String, String> {
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
    use notify::{Event, EventKind, event::CreateKind};
    use std::path::PathBuf;
    use sqlx::SqlitePool;
    

    #[test]
    fn test_file_watcher_creation() {
        let pool = std::sync::Arc::new(sqlx::SqlitePool::connect_lazy(":memory:").unwrap());
        let result = FileWatcher::new(pool);
        assert!(result.is_ok());
    }

    #[test]
    fn test_file_watcher_watch_directory() {
        let pool = std::sync::Arc::new(sqlx::SqlitePool::connect_lazy(":memory:").unwrap());
        let mut watcher = FileWatcher::new(pool).unwrap();
        
        // 存在しないディレクトリをwatch
        let result = watcher.watch_directory("test_id", "/nonexistent/path");
        assert!(result.is_err());
    }

    #[test]
    fn test_file_watcher_unwatch_directory() {
        let pool = std::sync::Arc::new(sqlx::SqlitePool::connect_lazy(":memory:").unwrap());
        let mut watcher = FileWatcher::new(pool).unwrap();
        
        // 存在しないディレクトリをunwatch
        let result = watcher.unwatch_directory("test_id");
        assert!(result.is_ok()); // unwatchは存在しないディレクトリでもエラーにならない
    }

    // Note: Tauriコマンドのテストは実際のTauri環境でのみ可能

    #[tokio::test]
    async fn test_handle_file_event_create() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        
        let event = Event {
            kind: EventKind::Create(CreateKind::File),
            paths: vec![PathBuf::from("/test/file.txt")],
            attrs: Default::default(),
        };
        
        let result = handle_file_event(&pool, event).await;
        // メタデータ取得に失敗するためエラーは発生しない（pathが存在しないため）
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_file_event_remove() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        
        let event = Event {
            kind: EventKind::Remove(notify::event::RemoveKind::File),
            paths: vec![PathBuf::from("/test/file.txt")],
            attrs: Default::default(),
        };
        
        let result = handle_file_event(&pool, event).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_file_event_modify() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        
        let event = Event {
            kind: EventKind::Modify(notify::event::ModifyKind::Data(notify::event::DataChange::Content)),
            paths: vec![PathBuf::from("/test/file.txt")],
            attrs: Default::default(),
        };
        
        let result = handle_file_event(&pool, event).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_file_event_other() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        
        let event = Event {
            kind: EventKind::Access(notify::event::AccessKind::Read),
            paths: vec![PathBuf::from("/test/file.txt")],
            attrs: Default::default(),
        };
        
        let result = handle_file_event(&pool, event).await;
        assert!(result.is_ok());
    }
} 
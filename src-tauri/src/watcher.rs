use crate::database::{Database, DatabaseTrait, File};
use sqlx::SqlitePool;
use notify::{Watcher, RecursiveMode, Event, EventKind};
use tauri::State;
use uuid::Uuid;
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::Arc;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub struct FileWatcher {
    watcher: notify::FsEventWatcher,
    watched_directories: HashMap<String, String>, // directory_id -> path
}

impl FileWatcher {
    pub fn new(pool: Arc<SqlitePool>) -> Result<Self, notify::Error> {
        let (tx, rx) = mpsc::channel();
        let pool_clone = Arc::clone(&pool);
        
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            while let Ok(event) = rx.recv() {
                if let Err(e) = rt.block_on(handle_file_event(&pool_clone, event)) {
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
    directory_id: String,
    path: String,
) -> Result<(), String> {
    // 実装予定: ファイル監視の開始
    // 現在はディレクトリスキャンのみ実装
    crate::file_manager::scan_directory(&pool, &directory_id, &path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_watching(
    _pool: State<'_, SqlitePool>,
    _directory_id: String,
) -> Result<(), String> {
    // 実装予定: ファイル監視の停止
    Ok(())
}

pub async fn handle_file_event(
    pool: &SqlitePool,
    event: Event,
) -> Result<(), String> {
    let db = Database;
    
    match event.kind {
        EventKind::Create(_) => {
            for path in event.paths {
                if let Ok(metadata) = fs::metadata(&path) {
                    let directory_id = find_directory_id_for_path(pool, &path).await?;
                    
                    let file = File {
                        id: Uuid::new_v4().to_string(),
                        path: path.to_string_lossy().to_string(),
                        name: path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string(),
                        directory_id,
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
                        inode: None,
                        is_directory: metadata.is_dir(),
                        created_at_db: Utc::now(),
                        updated_at_db: Utc::now(),
                        file_size: Some(metadata.len() as i64),
                        mime_type: None,
                        permissions: None,
                        owner_uid: None,
                        group_gid: None,
                        hard_links: None,
                        device_id: None,
                        last_accessed: metadata.accessed()
                            .ok()
                            .map(chrono::DateTime::from),
                        metadata: None,
                    };
                    
                    if let Err(e) = db.add_file(pool, &file).await {
                        eprintln!("ファイル追加エラー: {}", e);
                    }
                }
            }
        },
        EventKind::Remove(_) => {
            for path in event.paths {
                let path_str = path.to_string_lossy().to_string();
                if let Err(e) = db.remove_file_by_path(pool, &path_str).await {
                    eprintln!("ファイル削除エラー: {}", e);
                }
            }
        },
        EventKind::Modify(_) => {
            for path in event.paths {
                if let Ok(metadata) = fs::metadata(&path) {
                    let path_str = path.to_string_lossy().to_string();
                    if let Err(e) = db.update_file_metadata(pool, &path_str, &metadata).await {
                        eprintln!("ファイル更新エラー: {}", e);
                    }
                }
            }
        },
        _ => {},
    }
    
    Ok(())
}

async fn find_directory_id_for_path(pool: &SqlitePool, path: &PathBuf) -> Result<String, String> {
    let db = Database;
    let directories = db.get_all_directories(pool).await.map_err(|e| e.to_string())?;
    
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
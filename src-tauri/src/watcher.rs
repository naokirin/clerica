use crate::database::{Database, DatabaseTrait, File};
use sqlx::SqlitePool;
use notify::{Watcher, RecursiveMode, Event, EventKind};
use tauri::State;
use uuid::Uuid;
use chrono::Utc;
use std::fs;
use std::path::Path;

pub struct FileWatcher {
    watcher: notify::FsEventWatcher,
}

impl FileWatcher {
    pub fn new() -> Result<Self, notify::Error> {
        let watcher = notify::recommended_watcher(|_res| {
            // イベント処理は別途実装
        })?;
        
        Ok(FileWatcher {
            watcher,
        })
    }
    
    pub fn watch_directory(&mut self, path: &str) -> Result<(), notify::Error> {
        self.watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
        Ok(())
    }
    
    pub fn unwatch_directory(&mut self, path: &str) -> Result<(), notify::Error> {
        self.watcher.unwatch(Path::new(path))?;
        Ok(())
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
    match event.kind {
        EventKind::Create(_) => {
            for path in event.paths {
                if let Ok(metadata) = fs::metadata(&path) {
                    let file = File {
                        id: Uuid::new_v4().to_string(),
                        path: path.to_string_lossy().to_string(),
                        name: path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string(),
                        directory_id: "".to_string(), // ディレクトリIDの特定が必要
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
                        inode: None, // macOS固有の実装が必要
                        is_directory: metadata.is_dir(),
                        created_at_db: Utc::now(),
                        updated_at_db: Utc::now(),
                        file_size: Some(metadata.len() as i64),
                        mime_type: None, // TODO: MIME type inference
                        permissions: None, // TODO: Add permissions
                        owner_uid: None,
                        group_gid: None,
                        hard_links: None,
                        device_id: None,
                        last_accessed: metadata.accessed()
                            .ok()
                            .map(chrono::DateTime::from),
                        metadata: None,
                    };
                    
                    let db = Database;
                    db.add_file(pool, &file)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
        },
        EventKind::Remove(_) => {
            // ファイル削除の処理（実装予定）
        },
        EventKind::Modify(_) => {
            // ファイル変更の処理（実装予定）
        },
        _ => {},
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::{Event, EventKind, event::CreateKind};
    use std::path::PathBuf;
    use sqlx::SqlitePool;
    

    #[test]
    fn test_file_watcher_creation() {
        let result = FileWatcher::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_file_watcher_watch_directory() {
        let mut watcher = FileWatcher::new().unwrap();
        
        // 存在しないディレクトリをwatch
        let result = watcher.watch_directory("/nonexistent/path");
        assert!(result.is_err());
    }

    #[test]
    fn test_file_watcher_unwatch_directory() {
        let mut watcher = FileWatcher::new().unwrap();
        
        // 存在しないディレクトリをunwatch
        let result = watcher.unwatch_directory("/nonexistent/path");
        assert!(result.is_err());
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
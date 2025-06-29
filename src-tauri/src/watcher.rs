use crate::database::{self, File};
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
                            .map(|t| chrono::DateTime::from(t)),
                        modified_at: metadata.modified()
                            .ok()
                            .map(|t| chrono::DateTime::from(t)),
                        birth_time: None,
                        inode: None, // macOS固有の実装が必要
                        is_directory: metadata.is_dir(),
                        created_at_db: Utc::now(),
                        updated_at_db: Utc::now(),
                    };
                    
                    database::add_file(pool, &file)
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
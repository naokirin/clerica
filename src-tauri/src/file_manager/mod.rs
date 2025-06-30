use crate::database::{Database, DatabaseTrait, Directory, File};
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use walkdir::WalkDir;
use std::fs;
use std::os::unix::fs::MetadataExt;

#[cfg(test)]
mod tests;

#[tauri::command]
pub async fn add_directory(
    pool: State<'_, SqlitePool>,
    path: String,
    name: String,
) -> Result<Directory, String> {
    let db = Database;
    let directory = db.add_directory(&pool, &path, &name)
        .await
        .map_err(|e| e.to_string())?;
    
    // ディレクトリ追加後、ファイルスキャンを実行
    if let Err(e) = scan_directory(&pool, &directory.id, &path).await {
        eprintln!("ファイルスキャンエラー: {e}");
    }
    
    Ok(directory)
}

#[tauri::command]
pub async fn remove_directory(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    let db = Database;
    db.remove_directory(&pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_files(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<File>, String> {
    let db = Database;
    db.get_all_files(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_file_info(
    _pool: State<'_, SqlitePool>,
    _file_id: String,
) -> Result<File, String> {
    // 実装予定
    Err("Not implemented".to_string())
}

#[tauri::command]
pub async fn update_file_tags(
    pool: State<'_, SqlitePool>,
    file_id: String,
    tag_ids: Vec<String>,
) -> Result<(), String> {
    let db = Database;
    // 既存のタグを削除
    let current_tags = db.get_file_tags(&pool, &file_id)
        .await
        .map_err(|e| e.to_string())?;
    
    for tag in current_tags {
        db.remove_file_tag(&pool, &file_id, &tag.id)
            .await
            .map_err(|e| e.to_string())?;
    }
    
    // 新しいタグを追加
    for tag_id in tag_ids {
        db.add_file_tag(&pool, &file_id, &tag_id)
            .await
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn delete_file(
    _pool: State<'_, SqlitePool>,
    _file_id: String,
) -> Result<(), String> {
    // 実装予定
    Err("Not implemented".to_string())
}

#[tauri::command]
pub async fn move_file(
    _pool: State<'_, SqlitePool>,
    _file_id: String,
    _new_path: String,
) -> Result<(), String> {
    // 実装予定
    Err("Not implemented".to_string())
}

pub async fn scan_directory(pool: &SqlitePool, directory_id: &str, path: &str) -> Result<(), String> {
    let walker = WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok());
    
    for entry in walker {
        let path = entry.path();
        if let Ok(metadata) = fs::metadata(path) {
            let file = File {
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
                    .map(DateTime::from),
                modified_at: metadata.modified()
                    .ok()
                    .map(DateTime::from),
                birth_time: None, // macOS固有の実装が必要
                inode: Some(metadata.ino() as i64),
                is_directory: metadata.is_dir(),
                created_at_db: Utc::now(),
                updated_at_db: Utc::now(),
            };
            
            let db = Database;
            db.add_file(pool, &file)
                .await
                .map_err(|e| e.to_string())?;
        }
    }
    
    Ok(())
}

#[tauri::command]
pub async fn get_directories(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Directory>, String> {
    let db = Database;
    db.get_directories(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_files_by_directory(
    pool: State<'_, SqlitePool>,
    directory_id: String,
) -> Result<Vec<File>, String> {
    let db = Database;
    db.get_files_by_directory(&pool, &directory_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn rescan_directory(
    pool: State<'_, SqlitePool>,
    directory_id: String,
) -> Result<(), String> {
    let db = Database;
    
    // ディレクトリ情報を取得
    let directories = db.get_directories(&pool)
        .await
        .map_err(|e| e.to_string())?;
    
    let directory = directories.iter()
        .find(|d| d.id == directory_id)
        .ok_or("ディレクトリが見つかりません")?;
    
    // 既存のファイル情報を削除
    sqlx::query("DELETE FROM files WHERE directory_id = ?")
        .bind(&directory_id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    // ディレクトリを再スキャン
    scan_directory(&pool, &directory_id, &directory.path).await
}
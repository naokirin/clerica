use crate::database::{Database, DatabaseTrait, Directory, File};
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use walkdir::WalkDir;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

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
    pool: State<'_, SqlitePool>,
    file_path: String,
) -> Result<(), String> {
    // ファイルパスの存在確認
    if !std::path::Path::new(&file_path).exists() {
        return Err("ファイルが見つかりません".to_string());
    }

    // macOSでファイルをゴミ箱に移動
    let result = Command::new("osascript")
        .arg("-e")
        .arg(format!(
            "tell application \"Finder\" to move POSIX file \"{}\" to trash",
            file_path
        ))
        .output();
    
    match result {
        Ok(output) => {
            if output.status.success() {
                // データベースからファイル情報を削除
                let db = Database;
                sqlx::query("DELETE FROM files WHERE path = ?")
                    .bind(&file_path)
                    .execute(pool.inner())
                    .await
                    .map_err(|e| format!("データベース更新エラー: {e}"))?;
                
                Ok(())
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr);
                Err(format!("ファイルをゴミ箱に移動できませんでした: {error_message}"))
            }
        }
        Err(e) => Err(format!("コマンド実行エラー: {e}")),
    }
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

fn infer_mime_type(path: &std::path::Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            match ext.to_lowercase().as_str() {
                // 画像
                "jpg" | "jpeg" => "image/jpeg",
                "png" => "image/png",
                "gif" => "image/gif",
                "bmp" => "image/bmp",
                "webp" => "image/webp",
                "svg" => "image/svg+xml",
                "ico" => "image/x-icon",
                
                // 動画
                "mp4" => "video/mp4",
                "avi" => "video/x-msvideo",
                "mov" => "video/quicktime",
                "wmv" => "video/x-ms-wmv",
                "flv" => "video/x-flv",
                "webm" => "video/webm",
                "mkv" => "video/x-matroska",
                
                // 音声
                "mp3" => "audio/mpeg",
                "wav" => "audio/wav",
                "ogg" => "audio/ogg",
                "flac" => "audio/flac",
                "aac" => "audio/aac",
                "m4a" => "audio/mp4",
                
                // ドキュメント
                "pdf" => "application/pdf",
                "doc" => "application/msword",
                "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                "xls" => "application/vnd.ms-excel",
                "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                "ppt" => "application/vnd.ms-powerpoint",
                "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
                
                // テキスト
                "txt" => "text/plain",
                "md" => "text/markdown",
                "html" | "htm" => "text/html",
                "css" => "text/css",
                "js" => "text/javascript",
                "json" => "application/json",
                "xml" => "text/xml",
                "csv" => "text/csv",
                
                // プログラミング
                "rs" => "text/x-rust",
                "py" => "text/x-python",
                "java" => "text/x-java-source",
                "c" => "text/x-c",
                "cpp" | "cc" | "cxx" => "text/x-c++",
                "h" | "hpp" => "text/x-c-header",
                "go" => "text/x-go",
                "php" => "text/x-php",
                "rb" => "text/x-ruby",
                "swift" => "text/x-swift",
                "kt" => "text/x-kotlin",
                "scala" => "text/x-scala",
                
                // アーカイブ
                "zip" => "application/zip",
                "rar" => "application/x-rar-compressed",
                "7z" => "application/x-7z-compressed",
                "tar" => "application/x-tar",
                "gz" => "application/gzip",
                "bz2" => "application/x-bzip2",
                
                // その他
                "exe" => "application/x-msdownload",
                "dmg" => "application/x-apple-diskimage",
                "deb" => "application/vnd.debian.binary-package",
                "rpm" => "application/x-rpm",
                
                _ => "application/octet-stream",
            }
        })
        .map(String::from)
}

pub async fn scan_directory(pool: &SqlitePool, directory_id: &str, path: &str) -> Result<(), String> {
    let walker = WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok());
    
    for entry in walker {
        let path = entry.path();
        if let Ok(metadata) = fs::metadata(path) {
            // MIMEタイプの推定
            let mime_type = infer_mime_type(path);
            
            // ファイルパーミッション（8進数）
            let permissions = format!("{:o}", metadata.permissions().mode() & 0o777);
            
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
                file_size: Some(metadata.len() as i64),
                mime_type,
                permissions: Some(permissions),
                owner_uid: Some(metadata.uid() as i64),
                group_gid: Some(metadata.gid() as i64),
                hard_links: Some(metadata.nlink() as i64),
                device_id: Some(metadata.dev() as i64),
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

#[tauri::command]
pub async fn open_file(file_path: String) -> Result<(), String> {
    // ファイルパスの存在確認
    if !std::path::Path::new(&file_path).exists() {
        return Err("ファイルが見つかりません".to_string());
    }

    // macOSでファイルを開く
    let result = Command::new("open")
        .arg(&file_path)
        .output();
    
    match result {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr);
                Err(format!("ファイルを開けませんでした: {error_message}"))
            }
        }
        Err(e) => Err(format!("コマンド実行エラー: {e}")),
    }
}

#[tauri::command]
pub async fn reveal_in_finder(file_path: String) -> Result<(), String> {
    // ファイルパスの存在確認
    if !std::path::Path::new(&file_path).exists() {
        return Err("ファイルが見つかりません".to_string());
    }

    // macOSでFinderでファイルを表示
    let result = Command::new("open")
        .arg("-R")  // Reveal in Finder
        .arg(&file_path)
        .output();
    
    match result {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr);
                Err(format!("Finderで表示できませんでした: {error_message}"))
            }
        }
        Err(e) => Err(format!("コマンド実行エラー: {e}")),
    }
}
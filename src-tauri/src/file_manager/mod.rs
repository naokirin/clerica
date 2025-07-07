use crate::database::{Database, DatabaseTrait, Directory, File, Tag};
use crate::watcher::FileWatcher;
use crate::settings;
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use walkdir::WalkDir;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FileWithTags {
    pub file: File,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone)]
pub enum FileCategory {
    Image,
    Video,
    Audio,
    Document,
    Text,
    Programming,
    Archive,
    Other,
}

impl FileCategory {
    pub fn from_extension(extension: &str) -> Self {
        match extension.to_lowercase().as_str() {
            // 画像
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "ico" => FileCategory::Image,
            // 動画
            "mp4" | "avi" | "mov" | "wmv" | "flv" | "webm" | "mkv" => FileCategory::Video,
            // 音声
            "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a" => FileCategory::Audio,
            // ドキュメント
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" => FileCategory::Document,
            // テキスト
            "txt" | "md" | "html" | "htm" | "css" | "json" | "xml" | "csv" => FileCategory::Text,
            // プログラミング
            "rs" | "py" | "java" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "go" | "php" | "rb" | "swift" | "kt" | "scala" | "js" => FileCategory::Programming,
            // アーカイブ
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" => FileCategory::Archive,
            // その他
            _ => FileCategory::Other,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            FileCategory::Image => "Image".to_string(),
            FileCategory::Video => "Video".to_string(),
            FileCategory::Audio => "Audio".to_string(),
            FileCategory::Document => "Document".to_string(),
            FileCategory::Text => "Text".to_string(),
            FileCategory::Programming => "Programming".to_string(),
            FileCategory::Archive => "Archive".to_string(),
            FileCategory::Other => "Other".to_string(),
        }
    }
}

#[tauri::command]
pub async fn add_directory(
    pool: State<'_, SqlitePool>,
    watcher: State<'_, Arc<Mutex<FileWatcher>>>,
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
    
    // 設定を確認して自動タグ付けが有効な場合のみ実行
    let settings = settings::get_all_settings(&pool).await.unwrap_or_default();
    if settings.auto_tag_directories {
        if let Err(e) = analyze_and_auto_tag_directory(&pool, &directory.id, &path, settings.auto_tag_threshold).await {
            eprintln!("ファイルカテゴリ分析エラー: {e}");
        }
    }
    
    // ファイル監視を開始
    let mut watcher_guard = watcher.lock().map_err(|e| e.to_string())?;
    if let Err(e) = watcher_guard.watch_directory(&directory.id, &path) {
        eprintln!("ファイル監視開始エラー: {e}");
    } else {
        println!("ディレクトリの監視を開始しました: {path}");
    }
    
    Ok(directory)
}

#[derive(Debug, serde::Serialize)]
pub struct DirectoryRemovalResult {
    pub success: bool,
    pub deleted_tags_count: usize,
    pub deleted_tag_ids: Vec<String>,
}

#[tauri::command]
pub async fn remove_directory(
    pool: State<'_, SqlitePool>,
    watcher: State<'_, Arc<Mutex<FileWatcher>>>,
    id: String,
) -> Result<DirectoryRemovalResult, String> {
    let _db = Database;
    
    // ファイル監視を停止
    {
        let mut watcher_guard = watcher.lock().map_err(|e| e.to_string())?;
        if let Err(e) = watcher_guard.unwatch_directory(&id) {
            eprintln!("ファイル監視停止エラー: {e}");
        }
    }
    
    // トランザクション開始
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    
    // ディレクトリを削除（ON DELETE CASCADEにより関連ファイルも自動削除）
    sqlx::query("DELETE FROM directories WHERE id = ?")
        .bind(&id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    
    // 孤児タグを削除
    let orphaned_tag_ids: Vec<String> = sqlx::query_scalar(
        "SELECT t.id FROM tags t 
         LEFT JOIN file_tags ft ON t.id = ft.tag_id 
         WHERE ft.tag_id IS NULL"
    )
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    // 孤児タグを削除
    let deleted_tags_count = orphaned_tag_ids.len();
    if !orphaned_tag_ids.is_empty() {
        for tag_id in &orphaned_tag_ids {
            sqlx::query("DELETE FROM tags WHERE id = ?")
                .bind(tag_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
        }
        println!("削除されたタグ数: {}", deleted_tags_count);
    }
    
    // トランザクションをコミット
    tx.commit().await.map_err(|e| e.to_string())?;
    
    Ok(DirectoryRemovalResult {
        success: true,
        deleted_tags_count,
        deleted_tag_ids: orphaned_tag_ids,
    })
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
pub async fn get_files(
    pool: State<'_, SqlitePool>,
    sort_field: Option<String>,
    sort_order: Option<String>,
) -> Result<Vec<File>, String> {
    let db = Database;
    let mut files = db.get_all_files_sorted(&pool, sort_field, sort_order)
        .await
        .map_err(|e| e.to_string())?;
    
    // 設定を取得して隠しファイルを除外するかどうかを決定
    let settings = settings::get_all_settings(&pool).await
        .map_err(|e| e.to_string())?;
    
    if !settings.show_hidden_files {
        files.retain(|file| !settings::is_hidden_file(&file.path));
    }
    
    Ok(files)
}

#[tauri::command]
pub async fn get_files_paginated(
    pool: State<'_, SqlitePool>,
    sort_field: Option<String>,
    sort_order: Option<String>,
    limit: u32,
    offset: u32,
) -> Result<Vec<File>, String> {
    let db = Database;
    let mut files = db.get_all_files_paginated(&pool, sort_field, sort_order, limit, offset)
        .await
        .map_err(|e| e.to_string())?;
    
    // 設定を取得して隠しファイルを除外するかどうかを決定
    let settings = settings::get_all_settings(&pool).await
        .map_err(|e| e.to_string())?;
    
    if !settings.show_hidden_files {
        files.retain(|file| !settings::is_hidden_file(&file.path));
    }
    
    Ok(files)
}

#[tauri::command]
pub async fn get_files_with_tags(
    pool: State<'_, SqlitePool>,
    sort_field: Option<String>,
    sort_order: Option<String>,
) -> Result<Vec<FileWithTags>, String> {
    let db = Database;
    let mut files = db.get_all_files_sorted(&pool, sort_field, sort_order)
        .await
        .map_err(|e| e.to_string())?;
    
    // 設定を取得して隠しファイルを除外するかどうかを決定
    let settings = settings::get_all_settings(&pool).await
        .map_err(|e| e.to_string())?;
    
    if !settings.show_hidden_files {
        files.retain(|file| !settings::is_hidden_file(&file.path));
    }
    
    // 各ファイルのタグ情報を取得
    let mut results = Vec::new();
    for file in files {
        let tags = db.get_file_tags(&pool, &file.id)
            .await
            .map_err(|e| e.to_string())?;
        
        results.push(FileWithTags { file, tags });
    }
    
    Ok(results)
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
    
    // 未参照タグを削除
    db.delete_orphaned_tags(&pool)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_file_tags(
    pool: State<'_, SqlitePool>,
    file_id: String,
) -> Result<Vec<Tag>, String> {
    let db = Database;
    db.get_file_tags(&pool, &file_id)
        .await
        .map_err(|e| e.to_string())
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
            "tell application \"Finder\" to move POSIX file \"{file_path}\" to trash"
        ))
        .output();
    
    match result {
        Ok(output) => {
            if output.status.success() {
                // データベースからファイル情報を削除
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
                last_accessed: metadata.accessed()
                    .ok()
                    .map(DateTime::from),
                metadata: extract_metadata(path),
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
pub async fn get_files_by_directory(
    pool: State<'_, SqlitePool>,
    directory_id: String,
    sort_field: Option<String>,
    sort_order: Option<String>,
) -> Result<Vec<File>, String> {
    let db = Database;
    let mut files = db.get_files_by_directory_sorted(&pool, &directory_id, sort_field, sort_order)
        .await
        .map_err(|e| e.to_string())?;
    
    // 設定を取得して隠しファイルを除外するかどうかを決定
    let settings = settings::get_all_settings(&pool).await
        .map_err(|e| e.to_string())?;
    
    if !settings.show_hidden_files {
        files.retain(|file| !settings::is_hidden_file(&file.path));
    }
    
    Ok(files)
}

#[tauri::command]
pub async fn get_files_by_directory_paginated(
    pool: State<'_, SqlitePool>,
    directory_id: String,
    sort_field: Option<String>,
    sort_order: Option<String>,
    limit: u32,
    offset: u32,
) -> Result<Vec<File>, String> {
    let db = Database;
    let mut files = db.get_files_by_directory_paginated(&pool, &directory_id, sort_field, sort_order, limit, offset)
        .await
        .map_err(|e| e.to_string())?;
    
    // 設定を取得して隠しファイルを除外するかどうかを決定
    let settings = settings::get_all_settings(&pool).await
        .map_err(|e| e.to_string())?;
    
    if !settings.show_hidden_files {
        files.retain(|file| !settings::is_hidden_file(&file.path));
    }
    
    Ok(files)
}

#[tauri::command]
pub async fn count_files(
    pool: State<'_, SqlitePool>,
) -> Result<u32, String> {
    let db = Database;
    db.count_all_files(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn count_files_by_directory(
    pool: State<'_, SqlitePool>,
    directory_id: String,
) -> Result<u32, String> {
    let db = Database;
    db.count_files_by_directory(&pool, &directory_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn count_files_by_category(
    pool: State<'_, SqlitePool>,
    directory_id: String,
) -> Result<std::collections::HashMap<String, u32>, String> {
    use std::collections::HashMap;

    let query = if directory_id == "all" {
        "SELECT path FROM files WHERE is_directory = false".to_string()
    } else {
        "SELECT path FROM files WHERE is_directory = false AND directory_id = ?".to_string()
    };

    let files: Vec<String> = if directory_id == "all" {
        sqlx::query_scalar(&query)
            .fetch_all(pool.inner())
            .await
            .map_err(|e| e.to_string())?
    } else {
        sqlx::query_scalar(&query)
            .bind(&directory_id)
            .fetch_all(pool.inner())
            .await
            .map_err(|e| e.to_string())?
    };

    let mut counts = HashMap::new();
    counts.insert("all".to_string(), files.len() as u32);
    counts.insert("image".to_string(), 0);
    counts.insert("audio".to_string(), 0);
    counts.insert("video".to_string(), 0);
    counts.insert("document".to_string(), 0);
    counts.insert("archive".to_string(), 0);
    counts.insert("other".to_string(), 0);

    for file_path in &files {
        let category = get_file_category_from_path(file_path);
        let count = counts.get(&category).unwrap_or(&0) + 1;
        counts.insert(category, count);
    }

    Ok(counts)
}

#[tauri::command]
pub async fn get_files_paginated_with_category(
    pool: State<'_, SqlitePool>,
    category: String,
    sort_field: Option<String>,
    sort_order: Option<String>,
    limit: u32,
    offset: u32,
) -> Result<Vec<File>, String> {
    let db = Database;
    db.get_files_paginated_with_category(&pool, &category, sort_field, sort_order, limit, offset)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_files_by_directory_paginated_with_category(
    pool: State<'_, SqlitePool>,
    directory_id: String,
    category: String,
    sort_field: Option<String>,
    sort_order: Option<String>,
    limit: u32,
    offset: u32,
) -> Result<Vec<File>, String> {
    let db = Database;
    db.get_files_by_directory_paginated_with_category(&pool, &directory_id, &category, sort_field, sort_order, limit, offset)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn count_files_with_category(
    pool: State<'_, SqlitePool>,
    category: String,
) -> Result<u32, String> {
    let db = Database;
    db.count_files_with_category(&pool, &category)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn count_files_by_directory_with_category(
    pool: State<'_, SqlitePool>,
    directory_id: String,
    category: String,
) -> Result<u32, String> {
    let db = Database;
    db.count_files_by_directory_with_category(&pool, &directory_id, &category)
        .await
        .map_err(|e| e.to_string())
}

// ファイルパスからカテゴリを判定する関数
fn get_file_category_from_path(file_path: &str) -> String {
    let path = std::path::Path::new(file_path);
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        // 画像
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "ico" | "tiff" | "raw" => "image".to_string(),
        // 音声
        "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a" | "wma" | "opus" => "audio".to_string(),
        // 動画
        "mp4" | "avi" | "mov" | "wmv" | "flv" | "webm" | "mkv" | "m4v" | "3gp" => "video".to_string(),
        // ドキュメント
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "md" | "html" | "htm" | "css" | "js" | "json" | "xml" | "csv" | "rtf" => "document".to_string(),
        // アーカイブ
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "lzma" => "archive".to_string(),
        // その他
        _ => "other".to_string(),
    }
}

// カテゴリ条件をSQL WHERE句に変換する関数
fn build_category_where_clause(category: &str) -> String {
    if category == "all" {
        return String::new();
    }
    
    let extensions = match category {
        "image" => vec!["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg", "ico", "tiff", "raw"],
        "audio" => vec!["mp3", "wav", "ogg", "flac", "aac", "m4a", "wma", "opus"],
        "video" => vec!["mp4", "avi", "mov", "wmv", "flv", "webm", "mkv", "m4v", "3gp"],
        "document" => vec!["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "md", "html", "htm", "css", "js", "json", "xml", "csv", "rtf"],
        "archive" => vec!["zip", "rar", "7z", "tar", "gz", "bz2", "xz", "lzma"],
        "other" => return " AND (path NOT LIKE '%.jpg' AND path NOT LIKE '%.jpeg' AND path NOT LIKE '%.png' AND path NOT LIKE '%.gif' AND path NOT LIKE '%.bmp' AND path NOT LIKE '%.webp' AND path NOT LIKE '%.svg' AND path NOT LIKE '%.ico' AND path NOT LIKE '%.tiff' AND path NOT LIKE '%.raw' AND path NOT LIKE '%.mp3' AND path NOT LIKE '%.wav' AND path NOT LIKE '%.ogg' AND path NOT LIKE '%.flac' AND path NOT LIKE '%.aac' AND path NOT LIKE '%.m4a' AND path NOT LIKE '%.wma' AND path NOT LIKE '%.opus' AND path NOT LIKE '%.mp4' AND path NOT LIKE '%.avi' AND path NOT LIKE '%.mov' AND path NOT LIKE '%.wmv' AND path NOT LIKE '%.flv' AND path NOT LIKE '%.webm' AND path NOT LIKE '%.mkv' AND path NOT LIKE '%.m4v' AND path NOT LIKE '%.3gp' AND path NOT LIKE '%.pdf' AND path NOT LIKE '%.doc' AND path NOT LIKE '%.docx' AND path NOT LIKE '%.xls' AND path NOT LIKE '%.xlsx' AND path NOT LIKE '%.ppt' AND path NOT LIKE '%.pptx' AND path NOT LIKE '%.txt' AND path NOT LIKE '%.md' AND path NOT LIKE '%.html' AND path NOT LIKE '%.htm' AND path NOT LIKE '%.css' AND path NOT LIKE '%.js' AND path NOT LIKE '%.json' AND path NOT LIKE '%.xml' AND path NOT LIKE '%.csv' AND path NOT LIKE '%.rtf' AND path NOT LIKE '%.zip' AND path NOT LIKE '%.rar' AND path NOT LIKE '%.7z' AND path NOT LIKE '%.tar' AND path NOT LIKE '%.gz' AND path NOT LIKE '%.bz2' AND path NOT LIKE '%.xz' AND path NOT LIKE '%.lzma')".to_string(),
        _ => return String::new(),
    };
    
    let conditions: Vec<String> = extensions.iter()
        .map(|ext| format!("path LIKE '%.{}' OR path LIKE '%.{}'", ext, ext.to_uppercase()))
        .collect();
    
    format!(" AND ({})", conditions.join(" OR "))
}

#[tauri::command]
pub async fn get_files_by_directory_with_tags(
    pool: State<'_, SqlitePool>,
    directory_id: String,
    sort_field: Option<String>,
    sort_order: Option<String>,
) -> Result<Vec<FileWithTags>, String> {
    let db = Database;
    let mut files = db.get_files_by_directory_sorted(&pool, &directory_id, sort_field, sort_order)
        .await
        .map_err(|e| e.to_string())?;
    
    // 設定を取得して隠しファイルを除外するかどうかを決定
    let settings = settings::get_all_settings(&pool).await
        .map_err(|e| e.to_string())?;
    
    if !settings.show_hidden_files {
        files.retain(|file| !settings::is_hidden_file(&file.path));
    }
    
    // 各ファイルのタグ情報を取得
    let mut results = Vec::new();
    for file in files {
        let tags = db.get_file_tags(&pool, &file.id)
            .await
            .map_err(|e| e.to_string())?;
        
        results.push(FileWithTags { file, tags });
    }
    
    Ok(results)
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
pub async fn open_file(
    pool: State<'_, SqlitePool>,
    file_path: String,
) -> Result<(), String> {
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
                // ファイルのアクセス日時を更新
                update_file_last_accessed(&pool, &file_path).await?;
                Ok(())
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr);
                Err(format!("ファイルを開けませんでした: {error_message}"))
            }
        }
        Err(e) => Err(format!("コマンド実行エラー: {e}")),
    }
}

async fn update_file_last_accessed(pool: &SqlitePool, file_path: &str) -> Result<(), String> {
    let now = Utc::now();
    sqlx::query("UPDATE files SET last_accessed = ? WHERE path = ?")
        .bind(now)
        .bind(file_path)
        .execute(pool)
        .await
        .map_err(|e| format!("アクセス日時更新エラー: {e}"))?;
    
    Ok(())
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

fn extract_metadata(file_path: &std::path::Path) -> Option<String> {
    use serde_json::json;
    
    let mut metadata = json!({});
    
    if let Some(mime_type) = infer_mime_type(file_path) {
        // 画像ファイルのEXIFデータ抽出
        if mime_type.starts_with("image/") {
            if let Ok(exif_data) = extract_exif_metadata(file_path) {
                metadata["exif"] = exif_data;
            }
        }
        
        // 音声ファイルのメタデータ抽出
        if mime_type.starts_with("audio/") {
            if let Ok(audio_data) = extract_audio_metadata(file_path) {
                metadata["audio"] = audio_data;
            }
        }
    }
    
    if metadata == json!({}) {
        None
    } else {
        Some(metadata.to_string())
    }
}

fn extract_exif_metadata(file_path: &std::path::Path) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufReader;
    use exif::{Reader, Value};
    use serde_json::json;
    
    let file = File::open(file_path)?;
    let mut bufreader = BufReader::new(&file);
    let exifreader = Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    
    let mut metadata = json!({});
    
    for field in exif.fields() {
        let tag_name = get_tag_name(field.tag);
        let value = match &field.value {
            Value::Ascii(ref vec) if !vec.is_empty() => {
                if let Ok(s) = std::str::from_utf8(&vec[0]) {
                    json!(s)
                } else {
                    json!(format!("{:?}", vec))
                }
            }
            Value::Byte(ref vec) => json!(vec),
            Value::Short(ref vec) => json!(vec),
            Value::Long(ref vec) => json!(vec),
            Value::Rational(ref vec) => {
                let rationals: Vec<f64> = vec.iter()
                    .map(|r| r.num as f64 / r.denom as f64)
                    .collect();
                json!(rationals)
            }
            Value::SByte(ref vec) => json!(vec),
            Value::Undefined(ref vec, _) => json!(format!("{:02x?}", vec)),
            Value::SShort(ref vec) => json!(vec),
            Value::SLong(ref vec) => json!(vec),
            Value::SRational(ref vec) => {
                let rationals: Vec<f64> = vec.iter()
                    .map(|r| r.num as f64 / r.denom as f64)
                    .collect();
                json!(rationals)
            }
            Value::Float(ref vec) => json!(vec),
            Value::Double(ref vec) => json!(vec),
            _ => json!(field.display_value().to_string()),
        };
        
        metadata[tag_name] = value;
    }
    
    Ok(metadata)
}

fn extract_audio_metadata(file_path: &std::path::Path) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    use lofty::{probe::Probe, prelude::AudioFile, file::TaggedFileExt, tag::Accessor};
    use serde_json::json;
    
    let tagged_file = Probe::open(file_path)?.read()?;
    let mut metadata = json!({});
    
    // 基本的なオーディオプロパティ
    let properties = tagged_file.properties();
    metadata["duration"] = json!(properties.duration().as_secs());
    metadata["bitrate"] = json!(properties.overall_bitrate().unwrap_or(0));
    metadata["sample_rate"] = json!(properties.sample_rate().unwrap_or(0));
    metadata["channels"] = json!(properties.channels().unwrap_or(0));
    
    // タグ情報
    let primary_tag = tagged_file.primary_tag();
    let mut tags = json!({});
    
    if let Some(tag) = primary_tag {
        if let Some(title) = tag.title() {
            tags["title"] = json!(title);
        }
        if let Some(artist) = tag.artist() {
            tags["artist"] = json!(artist);
        }
        if let Some(album) = tag.album() {
            tags["album"] = json!(album);
        }
        if let Some(date) = tag.year() {
            tags["year"] = json!(date);
        }
        if let Some(genre) = tag.genre() {
            tags["genre"] = json!(genre);
        }
        if let Some(track) = tag.track() {
            tags["track"] = json!(track);
        }
    }
    
    if !tags.as_object().unwrap().is_empty() {
        metadata["tags"] = tags;
    }
    
    Ok(metadata)
}

fn get_tag_name(tag: exif::Tag) -> String {
    use exif::Tag;
    
    match tag {
        Tag::ImageWidth => "ImageWidth".to_string(),
        Tag::ImageLength => "ImageLength".to_string(),
        Tag::BitsPerSample => "BitsPerSample".to_string(),
        Tag::Compression => "Compression".to_string(),
        Tag::PhotometricInterpretation => "PhotometricInterpretation".to_string(),
        Tag::ImageDescription => "ImageDescription".to_string(),
        Tag::Make => "Make".to_string(),
        Tag::Model => "Model".to_string(),
        Tag::StripOffsets => "StripOffsets".to_string(),
        Tag::Orientation => "Orientation".to_string(),
        Tag::SamplesPerPixel => "SamplesPerPixel".to_string(),
        Tag::RowsPerStrip => "RowsPerStrip".to_string(),
        Tag::StripByteCounts => "StripByteCounts".to_string(),
        Tag::XResolution => "XResolution".to_string(),
        Tag::YResolution => "YResolution".to_string(),
        Tag::PlanarConfiguration => "PlanarConfiguration".to_string(),
        Tag::ResolutionUnit => "ResolutionUnit".to_string(),
        Tag::TransferFunction => "TransferFunction".to_string(),
        Tag::Software => "Software".to_string(),
        Tag::DateTime => "DateTime".to_string(),
        Tag::Artist => "Artist".to_string(),
        Tag::WhitePoint => "WhitePoint".to_string(),
        Tag::PrimaryChromaticities => "PrimaryChromaticities".to_string(),
        Tag::JPEGInterchangeFormat => "JpegInterchangeFormat".to_string(),
        Tag::JPEGInterchangeFormatLength => "JpegInterchangeFormatLength".to_string(),
        Tag::YCbCrCoefficients => "YCbCrCoefficients".to_string(),
        Tag::YCbCrSubSampling => "YCbCrSubSampling".to_string(),
        Tag::YCbCrPositioning => "YCbCrPositioning".to_string(),
        Tag::ReferenceBlackWhite => "ReferenceBlackWhite".to_string(),
        Tag::Copyright => "Copyright".to_string(),
        Tag::ExifIFDPointer => "ExifIfdPointer".to_string(),
        Tag::GPSInfoIFDPointer => "GpsInfoIfdPointer".to_string(),
        Tag::ExposureTime => "ExposureTime".to_string(),
        Tag::FNumber => "FNumber".to_string(),
        Tag::ExposureProgram => "ExposureProgram".to_string(),
        Tag::SpectralSensitivity => "SpectralSensitivity".to_string(),
        Tag::PhotographicSensitivity => "PhotographicSensitivity".to_string(),
        Tag::SensitivityType => "SensitivityType".to_string(),
        Tag::StandardOutputSensitivity => "StandardOutputSensitivity".to_string(),
        Tag::RecommendedExposureIndex => "RecommendedExposureIndex".to_string(),
        Tag::ISOSpeed => "IsoSpeed".to_string(),
        Tag::ISOSpeedLatitudeyyy => "IsoSpeedLatitudeyyy".to_string(),
        Tag::ISOSpeedLatitudezzz => "IsoSpeedLatitudezzz".to_string(),
        Tag::ExifVersion => "ExifVersion".to_string(),
        Tag::DateTimeOriginal => "DateTimeOriginal".to_string(),
        Tag::DateTimeDigitized => "DateTimeDigitized".to_string(),
        Tag::OffsetTime => "OffsetTime".to_string(),
        Tag::OffsetTimeOriginal => "OffsetTimeOriginal".to_string(),
        Tag::OffsetTimeDigitized => "OffsetTimeDigitized".to_string(),
        Tag::ComponentsConfiguration => "ComponentsConfiguration".to_string(),
        Tag::CompressedBitsPerPixel => "CompressedBitsPerPixel".to_string(),
        Tag::ShutterSpeedValue => "ShutterSpeedValue".to_string(),
        Tag::ApertureValue => "ApertureValue".to_string(),
        Tag::BrightnessValue => "BrightnessValue".to_string(),
        Tag::ExposureBiasValue => "ExposureBiasValue".to_string(),
        Tag::MaxApertureValue => "MaxApertureValue".to_string(),
        Tag::SubjectDistance => "SubjectDistance".to_string(),
        Tag::MeteringMode => "MeteringMode".to_string(),
        Tag::LightSource => "LightSource".to_string(),
        Tag::Flash => "Flash".to_string(),
        Tag::FocalLength => "FocalLength".to_string(),
        Tag::SubjectArea => "SubjectArea".to_string(),
        Tag::MakerNote => "MakerNote".to_string(),
        Tag::UserComment => "UserComment".to_string(),
        Tag::SubSecTime => "SubSecTime".to_string(),
        Tag::SubSecTimeOriginal => "SubSecTimeOriginal".to_string(),
        Tag::SubSecTimeDigitized => "SubSecTimeDigitized".to_string(),
        Tag::Temperature => "Temperature".to_string(),
        Tag::Humidity => "Humidity".to_string(),
        Tag::Pressure => "Pressure".to_string(),
        Tag::WaterDepth => "WaterDepth".to_string(),
        Tag::Acceleration => "Acceleration".to_string(),
        Tag::CameraElevationAngle => "CameraElevationAngle".to_string(),
        Tag::FlashpixVersion => "FlashPixVersion".to_string(),
        Tag::ColorSpace => "ColorSpace".to_string(),
        Tag::PixelXDimension => "PixelXDimension".to_string(),
        Tag::PixelYDimension => "PixelYDimension".to_string(),
        Tag::RelatedSoundFile => "RelatedSoundFile".to_string(),
        Tag::InteroperabilityIndex => "InteroperabilityIfdPointer".to_string(),
        Tag::FlashEnergy => "FlashEnergy".to_string(),
        Tag::SpatialFrequencyResponse => "SpatialFrequencyResponse".to_string(),
        Tag::FocalPlaneXResolution => "FocalPlaneXResolution".to_string(),
        Tag::FocalPlaneYResolution => "FocalPlaneYResolution".to_string(),
        Tag::FocalPlaneResolutionUnit => "FocalPlaneResolutionUnit".to_string(),
        Tag::SubjectLocation => "SubjectLocation".to_string(),
        Tag::ExposureIndex => "ExposureIndex".to_string(),
        Tag::SensingMethod => "SensingMethod".to_string(),
        Tag::FileSource => "FileSource".to_string(),
        Tag::SceneType => "SceneType".to_string(),
        Tag::CFAPattern => "CfaPattern".to_string(),
        Tag::CustomRendered => "CustomRendered".to_string(),
        Tag::ExposureMode => "ExposureMode".to_string(),
        Tag::WhiteBalance => "WhiteBalance".to_string(),
        Tag::DigitalZoomRatio => "DigitalZoomRatio".to_string(),
        Tag::FocalLengthIn35mmFilm => "FocalLengthIn35mmFilm".to_string(),
        Tag::SceneCaptureType => "SceneCaptureType".to_string(),
        Tag::GainControl => "GainControl".to_string(),
        Tag::Contrast => "Contrast".to_string(),
        Tag::Saturation => "Saturation".to_string(),
        Tag::Sharpness => "Sharpness".to_string(),
        Tag::DeviceSettingDescription => "DeviceSettingDescription".to_string(),
        Tag::SubjectDistanceRange => "SubjectDistanceRange".to_string(),
        Tag::ImageUniqueID => "ImageUniqueId".to_string(),
        Tag::CameraOwnerName => "CameraOwnerName".to_string(),
        Tag::BodySerialNumber => "BodySerialNumber".to_string(),
        Tag::LensSpecification => "LensSpecification".to_string(),
        Tag::LensMake => "LensMake".to_string(),
        Tag::LensModel => "LensModel".to_string(),
        Tag::LensSerialNumber => "LensSerialNumber".to_string(),
        _ => {
            // 不明なタグの場合は数値で表示
            format!("Tag_{}", tag.number())
        }
    }
}

/// ディレクトリツリー全体のファイル分析と自動タグ付けを実行する関数
async fn analyze_and_auto_tag_directory(
    pool: &SqlitePool,
    _directory_id: &str,
    directory_path: &str,
    threshold: f64,
) -> Result<(), String> {
    println!("ディレクトリツリー '{}' の自動タグ付けを開始", directory_path);
    
    // ディレクトリツリー内のすべてのディレクトリを取得
    let walker = WalkDir::new(directory_path)
        .follow_links(false)
        .max_depth(100)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir()); // ディレクトリのみを対象
    
    let mut processed_directories = 0;
    
    // 各ディレクトリを個別に分析
    for dir_entry in walker {
        let dir_path = dir_entry.path();
        
        if let Err(e) = analyze_and_auto_tag_single_directory(pool, dir_path, threshold).await {
            eprintln!("ディレクトリ '{}' の分析エラー: {}", dir_path.display(), e);
        }
        
        processed_directories += 1;
    }
    
    println!("自動タグ付け完了: {} 個のディレクトリを処理しました", processed_directories);
    Ok(())
}

/// 単一ディレクトリのファイル分析と自動タグ付けを実行する関数
async fn analyze_and_auto_tag_single_directory(
    pool: &SqlitePool,
    directory_path: &std::path::Path,
    threshold: f64,
) -> Result<(), String> {
    // 直下のファイルのみを取得（サブディレクトリは含まない）
    let entries = match std::fs::read_dir(directory_path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("ディレクトリ読み取りエラー: {}: {}", directory_path.display(), e);
            return Ok(());
        }
    };
    
    let mut category_counts: HashMap<String, usize> = HashMap::new();
    let mut total_files = 0;
    let mut has_git_directory = false;
    
    // ディレクトリ直下のファイルをスキャンしてカテゴリを分析
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            
            // .gitディレクトリをチェック
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    // .gitディレクトリが存在するかチェック
                    if let Some(file_name) = path.file_name() {
                        if file_name == ".git" {
                            has_git_directory = true;
                        }
                    }
                    continue; // ディレクトリはファイルカウントから除外
                }
            }
            
            // 拡張子がある通常ファイルのみを対象にする
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                let category = FileCategory::from_extension(extension);
                let category_name = category.to_string();
                
                *category_counts.entry(category_name).or_insert(0) += 1;
                total_files += 1;
            }
        }
    }
    
    // Gitディレクトリが存在する場合はgitタグを追加
    if has_git_directory {
        let db = Database;
        let git_tag = match db.get_tag_by_name(&pool, "git").await {
            Ok(existing_tag) => existing_tag,
            Err(_) => {
                // gitタグが存在しない場合は新規作成
                let new_tag = db.create_tag(&pool, "git", "#F05032") // Git オレンジ色
                    .await.map_err(|e| e.to_string())?;
                new_tag
            }
        };
        
        // ディレクトリにgitタグを付与
        if let Ok(directory_file_id) = get_or_create_directory_file_entry(pool, directory_path).await {
            if let Err(e) = db.add_file_tag(&pool, &directory_file_id, &git_tag.id).await {
                // 既に存在する場合のエラーは無視
                if !e.to_string().contains("UNIQUE constraint failed") {
                    eprintln!("Gitタグ追加エラー (directory_file_id: {}, tag_id: {}): {}", directory_file_id, git_tag.id, e);
                }
            } else {
                println!("Git自動タグ付け完了: ディレクトリ '{}' に 'git' タグを追加", directory_path.display());
            }
        }
    }
    
    // ファイルが少ない場合はファイルカテゴリ分析をスキップ（Gitタグは付与済み）
    if total_files < 2 {
        return Ok(());
    }
    
    // 設定された閾値を超えるカテゴリをチェック
    for (category_name, count) in category_counts {
        if total_files > 0 && (count as f64 / total_files as f64) >= threshold {
            // 自動タグを作成・取得
            let db = Database;
            let tag = match db.get_tag_by_name(&pool, &category_name).await {
                Ok(existing_tag) => existing_tag,
                Err(_) => {
                    // タグが存在しない場合は新規作成
                    let new_tag = db.create_tag(&pool, &category_name, &get_default_color_for_category(&category_name))
                        .await.map_err(|e| e.to_string())?;
                    new_tag
                }
            };
            
            // ディレクトリにタグを付与（ディレクトリをfilesテーブルのエントリとして扱う）
            if let Ok(directory_file_id) = get_or_create_directory_file_entry(pool, directory_path).await {
                if let Err(e) = db.add_file_tag(&pool, &directory_file_id, &tag.id).await {
                    // 既に存在する場合のエラーは無視
                    if !e.to_string().contains("UNIQUE constraint failed") {
                        eprintln!("ディレクトリタグ追加エラー (directory_file_id: {}, tag_id: {}): {}", directory_file_id, tag.id, e);
                    }
                } else {
                    println!("自動タグ付け完了: ディレクトリ '{}' に '{}' タグを追加 ({}% - {} ファイル)", 
                             directory_path.display(),
                             category_name, 
                             ((count as f64 / total_files as f64) * 100.0) as u32,
                             count);
                }
            }
        }
    }
    
    Ok(())
}

/// ファイルパスからファイルIDを取得する関数
async fn get_file_id_by_path(pool: &SqlitePool, file_path: &str) -> Result<String, sqlx::Error> {
    let file_id: String = sqlx::query_scalar("SELECT id FROM files WHERE path = ?")
        .bind(file_path)
        .fetch_one(pool)
        .await?;
    Ok(file_id)
}

/// ディレクトリをfilesテーブルのエントリとして作成または取得する関数
async fn get_or_create_directory_file_entry(
    pool: &SqlitePool,
    directory_path: &std::path::Path,
) -> Result<String, String> {
    let path_str = directory_path.to_string_lossy().to_string();
    
    // 既存のディレクトリファイルエントリを検索
    if let Ok(file_id) = sqlx::query_scalar::<_, String>(
        "SELECT id FROM files WHERE path = ? AND is_directory = true"
    )
    .bind(&path_str)
    .fetch_one(pool)
    .await
    {
        return Ok(file_id);
    }
    
    // ディレクトリファイルエントリが存在しない場合は作成
    let file_id = Uuid::new_v4().to_string();
    let name = directory_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    // 親ディレクトリIDを取得（存在しない場合は空文字列）
    let parent_path = directory_path.parent();
    let directory_id = if let Some(parent) = parent_path {
        match sqlx::query_scalar::<_, String>(
            "SELECT id FROM directories WHERE path = ?"
        )
        .bind(parent.to_string_lossy().as_ref())
        .fetch_optional(pool)
        .await
        {
            Ok(Some(id)) => id,
            _ => String::new(), // 親が見つからない場合は空文字列
        }
    } else {
        String::new()
    };
    
    let now = Utc::now();
    
    // ディレクトリメタデータを取得
    let (size, created_at, modified_at, inode, permissions, owner_uid, group_gid, hard_links, device_id, last_accessed) = 
        if let Ok(metadata) = std::fs::metadata(directory_path) {
            use std::os::unix::fs::MetadataExt;
            (
                metadata.len() as i64,
                metadata.created().ok().map(DateTime::from),
                metadata.modified().ok().map(DateTime::from),
                Some(metadata.ino() as i64),
                Some(format!("{:o}", metadata.mode() & 0o777)),
                Some(metadata.uid() as i64),
                Some(metadata.gid() as i64),
                Some(metadata.nlink() as i64),
                Some(metadata.dev() as i64),
                metadata.accessed().ok().map(DateTime::from),
            )
        } else {
            (0, None::<DateTime<Utc>>, None::<DateTime<Utc>>, None, None, None, None, None, None, None::<DateTime<Utc>>)
        };
    
    // ディレクトリをfilesテーブルに挿入
    sqlx::query(
        "INSERT INTO files (id, path, name, directory_id, size, file_type, created_at, modified_at, birth_time, inode, is_directory, created_at_db, updated_at_db, file_size, mime_type, permissions, owner_uid, group_gid, hard_links, device_id, last_accessed, metadata) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&file_id)
    .bind(&path_str)
    .bind(&name)
    .bind(&directory_id)
    .bind(size)
    .bind(None::<String>) // file_type (ディレクトリなのでnull)
    .bind(created_at)
    .bind(modified_at)
    .bind(None::<DateTime<Utc>>) // birth_time
    .bind(inode)
    .bind(true) // is_directory
    .bind(now)
    .bind(now)
    .bind(Some(size))
    .bind(Some("inode/directory".to_string())) // mime_type
    .bind(permissions)
    .bind(owner_uid)
    .bind(group_gid)
    .bind(hard_links)
    .bind(device_id)
    .bind(last_accessed)
    .bind(Some("{}".to_string())) // metadata (JSON)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(file_id)
}

/// カテゴリに応じたデフォルトカラーを取得
fn get_default_color_for_category(category: &str) -> String {
    match category {
        "Image" => "#FF6B6B".to_string(),     // 赤系
        "Video" => "#4ECDC4".to_string(),     // 青緑系
        "Audio" => "#45B7D1".to_string(),     // 青系
        "Document" => "#96CEB4".to_string(),  // 緑系
        "Text" => "#FFEAA7".to_string(),      // 黄系
        "Programming" => "#DDA0DD".to_string(), // 紫系
        "Archive" => "#F4A261".to_string(),   // オレンジ系
        "Other" => "#95A5A6".to_string(),     // グレー系
        _ => "#95A5A6".to_string(),           // デフォルト
    }
}
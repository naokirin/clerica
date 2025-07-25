use crate::database::{Database, DatabaseTrait, Directory, File};
use crate::exclusion_patterns::ExclusionPatternManager;
use crate::watcher::FileWatcher;
use crate::settings;
use crate::ShelfManager;
use crate::file_manager::FileCategory;
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use walkdir::WalkDir;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug, serde::Serialize)]
pub struct DirectoryRemovalResult {
    pub success: bool,
    pub deleted_tags_count: usize,
    pub deleted_tag_ids: Vec<String>,
}

/// ディレクトリを追加し、ファイルスキャンと監視を開始する
#[tauri::command]
pub async fn add_directory(
    pools: State<'_, ShelfManager>,
    watcher: State<'_, Arc<Mutex<FileWatcher>>>,
    path: String,
    name: String,
) -> Result<Directory, String> {
    let db = Database;
    let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
    let directory = db.add_directory(&data_pool, &path, &name)
        .await
        .map_err(|e| e.to_string())?;
    
    // ディレクトリ追加後、ファイルスキャンを実行
    if let Err(e) = scan_directory(&pools, &directory.id, &path).await {
        eprintln!("ファイルスキャンエラー: {e}");
    }
    
    // 設定を確認して自動タグ付けが有効な場合のみ実行
    let settings = settings::get_all_settings(pools.get_settings_pool()).await.unwrap_or_default();
    if settings.auto_tag_directories {
        if let Err(e) = analyze_and_auto_tag_directory(&pools, &directory.id, &path, settings.auto_tag_threshold).await {
            eprintln!("ファイルカテゴリ分析エラー: {e}");
        }
    }
    
    // ファイル監視を開始
    let mut watcher_guard = watcher.lock().map_err(|e| e.to_string())?;
    if let Err(e) = watcher_guard.watch_directory(&directory.id, &path) {
        eprintln!("ファイル監視開始エラー: {e}");
    } else {
        #[cfg(debug_assertions)]
        println!("ディレクトリの監視を開始しました: {path}");
    }
    
    Ok(directory)
}

/// ディレクトリを削除し、関連ファイルと孤児タグを削除する
#[tauri::command]
pub async fn remove_directory(
    pools: State<'_, ShelfManager>,
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
    let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
    let mut tx = data_pool.begin().await.map_err(|e| e.to_string())?;
    
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
        #[cfg(debug_assertions)]
        println!("削除されたタグ数: {deleted_tags_count}");
    }
    
    // トランザクションをコミット
    tx.commit().await.map_err(|e| e.to_string())?;
    
    Ok(DirectoryRemovalResult {
        success: true,
        deleted_tags_count,
        deleted_tag_ids: orphaned_tag_ids,
    })
}

/// すべてのディレクトリを取得する
#[tauri::command]
pub async fn get_directories(
    pools: State<'_, ShelfManager>,
) -> Result<Vec<Directory>, String> {
    let db = Database;
    let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
    db.get_directories(&data_pool)
        .await
        .map_err(|e| e.to_string())
}

/// ディレクトリを再スキャンする
#[tauri::command]
pub async fn rescan_directory(
    pools: State<'_, ShelfManager>,
    directory_id: String,
) -> Result<(), String> {
    let db = Database;
    let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
    
    // ディレクトリ情報を取得
    let directories = db.get_directories(&data_pool)
        .await
        .map_err(|e| e.to_string())?;
    
    let directory = directories.iter()
        .find(|d| d.id == directory_id)
        .ok_or("ディレクトリが見つかりません")?;
    
    // 既存のファイル情報を削除
    sqlx::query("DELETE FROM files WHERE directory_id = ?")
        .bind(&directory_id)
        .execute(&data_pool)
        .await
        .map_err(|e| e.to_string())?;
    
    // ディレクトリを再スキャン
    scan_directory(&pools, &directory_id, &directory.path).await
}

/// ディレクトリをスキャンしてファイル情報をデータベースに追加する
pub async fn scan_directory(pools: &ShelfManager, directory_id: &str, path: &str) -> Result<(), String> {
    // 除外パターンマネージャーを初期化
    let exclusion_manager = ExclusionPatternManager::new();
    exclusion_manager.refresh_patterns(pools.get_settings_pool()).await?;
    
    let walker = WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok());
    
    for entry in walker {
        let path = entry.path();
        let path_str = path.to_string_lossy();
        
        // 除外パターンチェック
        if exclusion_manager.should_exclude(&path_str) {
            continue;
        }
        
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
            let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
            db.add_file(&data_pool, &file)
                .await
                .map_err(|e| e.to_string())?;
        }
    }
    
    Ok(())
}

/// ディレクトリツリーを分析して自動タグ付けを行う
async fn analyze_and_auto_tag_directory(
    pools: &ShelfManager,
    _directory_id: &str,
    directory_path: &str,
    threshold: f64,
) -> Result<(), String> {
    #[cfg(debug_assertions)]
    println!("ディレクトリツリー '{directory_path}' の自動タグ付けを開始");
    
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
        
        if let Err(e) = analyze_and_auto_tag_single_directory(pools, dir_path, threshold).await {
            eprintln!("ディレクトリ '{}' の分析エラー: {}", dir_path.display(), e);
        }
        
        processed_directories += 1;
    }
    
    #[cfg(debug_assertions)]
    println!("自動タグ付け完了: {processed_directories} 個のディレクトリを処理しました");
    Ok(())
}

/// 単一ディレクトリのファイル分析と自動タグ付けを実行する関数
async fn analyze_and_auto_tag_single_directory(
    pools: &ShelfManager,
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
    for entry in entries.flatten() {
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
    
    // Gitディレクトリが存在する場合はgitタグを追加
    if has_git_directory {
        let db = Database;
        let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
        let git_tag = match db.get_tag_by_name(&data_pool, "git").await {
            Ok(existing_tag) => existing_tag,
            Err(_) => {
                // gitタグが存在しない場合は新規作成
                let new_tag = db.create_tag(&data_pool, "git", "#F05032") // Git オレンジ色
                    .await.map_err(|e| e.to_string())?;
                new_tag
            }
        };
        
        // ディレクトリにgitタグを付与
        if let Ok(directory_file_id) = get_or_create_directory_file_entry(&data_pool, directory_path).await {
            if let Err(e) = db.add_file_tag(&data_pool, &directory_file_id, &git_tag.id).await {
                // 既に存在する場合のエラーは無視
                if !e.to_string().contains("UNIQUE constraint failed") {
                    eprintln!("Gitタグ追加エラー (directory_file_id: {}, tag_id: {}): {}", directory_file_id, git_tag.id, e);
                }
            } else {
                #[cfg(debug_assertions)]
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
            let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
            let tag = match db.get_tag_by_name(&data_pool, &category_name).await {
                Ok(existing_tag) => existing_tag,
                Err(_) => {
                    // タグが存在しない場合は新規作成
                    let new_tag = db.create_tag(&data_pool, &category_name, &get_default_color_for_category(&category_name))
                        .await.map_err(|e| e.to_string())?;
                    new_tag
                }
            };
            
            // ディレクトリにタグを付与（ディレクトリをfilesテーブルのエントリとして扱う）
            if let Ok(directory_file_id) = get_or_create_directory_file_entry(&data_pool, directory_path).await {
                if let Err(e) = db.add_file_tag(&data_pool, &directory_file_id, &tag.id).await {
                    // 既に存在する場合のエラーは無視
                    if !e.to_string().contains("UNIQUE constraint failed") {
                        eprintln!("ディレクトリタグ追加エラー (directory_file_id: {}, tag_id: {}): {}", directory_file_id, tag.id, e);
                    }
                } else {
                    #[cfg(debug_assertions)]
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

/// ファイル拡張子からMIMEタイプを推定する
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

/// ファイルからメタデータを抽出する
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

/// EXIF メタデータを抽出する
fn extract_exif_metadata(file_path: &std::path::Path) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufReader;
    use exif::{Reader, Value};
    use serde_json::json;
    use crate::file_manager::files::get_tag_name;
    
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

/// 音声ファイルのメタデータを抽出する
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
            tags["title"] = json!(title.to_string());
        }
        if let Some(artist) = tag.artist() {
            tags["artist"] = json!(artist.to_string());
        }
        if let Some(album) = tag.album() {
            tags["album"] = json!(album.to_string());
        }
        if let Some(year) = tag.year() {
            tags["year"] = json!(year);
        }
        if let Some(track) = tag.track() {
            tags["track"] = json!(track);
        }
        if let Some(genre) = tag.genre() {
            tags["genre"] = json!(genre.to_string());
        }
    }
    
    if !tags.as_object().unwrap().is_empty() {
        metadata["tags"] = tags;
    }
    
    Ok(metadata)
}
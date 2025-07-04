use crate::database::{Database, DatabaseTrait, Directory, File};
use crate::watcher::FileWatcher;
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

#[cfg(test)]
mod tests;

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
    
    // ファイル監視を開始
    let mut watcher_guard = watcher.lock().map_err(|e| e.to_string())?;
    if let Err(e) = watcher_guard.watch_directory(&directory.id, &path) {
        eprintln!("ファイル監視開始エラー: {e}");
    } else {
        println!("ディレクトリの監視を開始しました: {path}");
    }
    
    Ok(directory)
}

#[tauri::command]
pub async fn remove_directory(
    pool: State<'_, SqlitePool>,
    watcher: State<'_, Arc<Mutex<FileWatcher>>>,
    id: String,
) -> Result<(), String> {
    let db = Database;
    
    // ファイル監視を停止
    {
        let mut watcher_guard = watcher.lock().map_err(|e| e.to_string())?;
        if let Err(e) = watcher_guard.unwatch_directory(&id) {
            eprintln!("ファイル監視停止エラー: {e}");
        }
    }
    
    db.remove_directory(&pool, &id)
        .await
        .map_err(|e| e.to_string())
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
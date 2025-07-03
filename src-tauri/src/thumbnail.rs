use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs::File;
use std::io::{BufReader, Read};
use zip::ZipArchive;
// use sevenz_rust::SevenZReader;  // 7zサポートは一時的に無効化
use unrar::Archive as RarArchive;
use tar::Archive as TarArchive;
use flate2::read::GzDecoder;

#[derive(Debug, Serialize, Deserialize)]
pub struct ThumbnailError {
    pub message: String,
}

impl std::fmt::Display for ThumbnailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ThumbnailError {}

pub struct ThumbnailGenerator {
    cache_dir: PathBuf,
}

impl ThumbnailGenerator {
    pub fn new() -> Result<Self, ThumbnailError> {
        let cache_dir = Self::get_cache_dir()?;

        // キャッシュディレクトリが存在しない場合は作成
        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir).map_err(|e| ThumbnailError {
                message: format!("Failed to create cache directory: {}", e),
            })?;
        }

        Ok(Self { cache_dir })
    }

    fn get_cache_dir() -> Result<PathBuf, ThumbnailError> {
        let cache_dir = if cfg!(debug_assertions) {
            // 開発モード: プロジェクトルートのcacheディレクトリ
            PathBuf::from("./cache/thumbnails")
        } else {
            // 本番モード: ホームディレクトリのcacheディレクトリ
            let home = env::var("HOME").map_err(|_| ThumbnailError {
                message: "Failed to get home directory".to_string(),
            })?;
            PathBuf::from(format!("{}/Library/Caches/Clerica/thumbnails", home))
        };

        Ok(cache_dir)
    }

    fn get_thumbnail_path(&self, video_path: &Path) -> PathBuf {
        let file_name = video_path.file_name().unwrap_or_default().to_string_lossy();
        let hash = format!(
            "{:x}",
            md5::compute(video_path.to_string_lossy().as_bytes())
        );
        let thumbnail_name = format!("{}_{}.jpg", file_name, hash);
        self.cache_dir.join(thumbnail_name)
    }

    pub fn generate_thumbnail(&self, video_path: &Path) -> Result<PathBuf, ThumbnailError> {
        let thumbnail_path = self.get_thumbnail_path(video_path);

        // 既にサムネイルが存在する場合はそれを返す
        if thumbnail_path.exists() {
            return Ok(thumbnail_path);
        }

        // ffmpegを使用してサムネイルを生成
        let output = Command::new("ffmpeg")
            .args([
                "-i",
                video_path.to_str().unwrap(),
                "-vf",
                "thumbnail,scale=192:192:force_original_aspect_ratio=increase,crop=192:192",
                "-frames:v",
                "1",
                "-q:v",
                "2",
                thumbnail_path.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| ThumbnailError {
                message: format!("Failed to execute ffmpeg: {}", e),
            })?;

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(ThumbnailError {
                message: format!("ffmpeg failed: {}", error_message),
            });
        }

        if !thumbnail_path.exists() {
            return Err(ThumbnailError {
                message: "Thumbnail file was not created".to_string(),
            });
        }

        Ok(thumbnail_path)
    }

    pub fn cleanup_old_thumbnails(&self) -> Result<(), ThumbnailError> {
        let max_age = std::time::Duration::from_secs(10 * 24 * 60 * 60); // 10日

        let entries = std::fs::read_dir(&self.cache_dir).map_err(|e| ThumbnailError {
            message: format!("Failed to read cache directory: {}", e),
        })?;

        let now = std::time::SystemTime::now();

        for entry in entries {
            let entry = entry.map_err(|e| ThumbnailError {
                message: format!("Failed to read directory entry: {}", e),
            })?;

            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = std::fs::metadata(&path) {
                    if let Ok(modified) = metadata.modified() {
                        if let Ok(elapsed) = now.duration_since(modified) {
                            if elapsed > max_age {
                                let _ = std::fs::remove_file(&path);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn is_video_file(path: &Path) -> bool {
        let video_extensions = [
            "mp4", "avi", "mov", "wmv", "flv", "webm", "mkv", "m4v", "3gp",
        ];

        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                return video_extensions.contains(&ext_str.to_lowercase().as_str());
            }
        }

        false
    }

    pub fn is_audio_file(path: &Path) -> bool {
        let audio_extensions = [
            "mp3", "wav", "ogg", "flac", "aac", "m4a", "wma", "opus",
        ];

        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                return audio_extensions.contains(&ext_str.to_lowercase().as_str());
            }
        }

        false
    }

    pub fn is_archive_file(path: &Path) -> bool {
        let archive_extensions = [
            "zip", "rar", "7z", "tar", "gz", "bz2", "xz", "lzma",
        ];

        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                return archive_extensions.contains(&ext_str.to_lowercase().as_str());
            }
        }

        false
    }

    pub fn is_image_file(path: &Path) -> bool {
        let image_extensions = [
            "jpg", "jpeg", "png", "gif", "bmp", "webp", "svg", "ico", "tiff", "raw",
        ];

        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                return image_extensions.contains(&ext_str.to_lowercase().as_str());
            }
        }

        false
    }

    fn get_audio_thumbnail_path(&self, audio_path: &Path) -> PathBuf {
        let file_name = audio_path.file_name().unwrap_or_default().to_string_lossy();
        let hash = format!(
            "{:x}",
            md5::compute(audio_path.to_string_lossy().as_bytes())
        );
        let thumbnail_name = format!("audio_{}_{}.jpg", file_name, hash);
        self.cache_dir.join(thumbnail_name)
    }

    pub fn extract_album_art(&self, audio_path: &Path) -> Result<PathBuf, ThumbnailError> {
        let thumbnail_path = self.get_audio_thumbnail_path(audio_path);

        // 既にサムネイルが存在する場合はそれを返す
        if thumbnail_path.exists() {
            return Ok(thumbnail_path);
        }

        // loftyを使用してアルバムアートを抽出
        use lofty::prelude::*;
        use lofty::probe::Probe;

        let tagged_file = Probe::open(audio_path)
            .map_err(|e| ThumbnailError {
                message: format!("Failed to open audio file: {}", e),
            })?
            .read()
            .map_err(|e| ThumbnailError {
                message: format!("Failed to read audio file: {}", e),
            })?;

        // アルバムアートを取得
        let picture = tagged_file
            .first_tag()
            .and_then(|tag| tag.pictures().first())
            .ok_or_else(|| ThumbnailError {
                message: "No album art found".to_string(),
            })?;

        // 画像データをファイルに保存
        std::fs::write(&thumbnail_path, picture.data())
            .map_err(|e| ThumbnailError {
                message: format!("Failed to write album art: {}", e),
            })?;

        Ok(thumbnail_path)
    }

    pub fn get_cache_size(&self) -> Result<u64, ThumbnailError> {
        let mut total_size = 0;

        let entries = std::fs::read_dir(&self.cache_dir).map_err(|e| ThumbnailError {
            message: format!("Failed to read cache directory: {}", e),
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| ThumbnailError {
                message: format!("Failed to read directory entry: {}", e),
            })?;

            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = std::fs::metadata(&path) {
                    total_size += metadata.len();
                }
            }
        }

        Ok(total_size)
    }

    fn get_archive_thumbnail_path(&self, archive_path: &Path) -> PathBuf {
        let file_name = archive_path.file_name().unwrap_or_default().to_string_lossy();
        let hash = format!(
            "{:x}",
            md5::compute(archive_path.to_string_lossy().as_bytes())
        );
        let thumbnail_name = format!("archive_{}_{}.jpg", file_name, hash);
        self.cache_dir.join(thumbnail_name)
    }

    fn get_first_image_from_zip(&self, archive_path: &Path) -> Result<(String, Vec<u8>), ThumbnailError> {
        let file = File::open(archive_path).map_err(|e| ThumbnailError {
            message: format!("Failed to open archive file: {}", e),
        })?;

        let reader = BufReader::new(file);
        let mut archive = ZipArchive::new(reader).map_err(|e| ThumbnailError {
            message: format!("Failed to read ZIP archive: {}", e),
        })?;

        // アーカイブ内のファイル名を取得してソート
        let mut file_names: Vec<String> = Vec::new();
        for i in 0..archive.len() {
            let file = archive.by_index(i).map_err(|e| ThumbnailError {
                message: format!("Failed to read file from archive: {}", e),
            })?;
            file_names.push(file.name().to_string());
        }

        // ファイル名でソート
        file_names.sort();

        // 最初の画像ファイルを探す
        for file_name in file_names {
            let file_path = Path::new(&file_name);
            if Self::is_image_file(file_path) {
                let mut file = archive.by_name(&file_name).map_err(|e| ThumbnailError {
                    message: format!("Failed to read file from archive: {}", e),
                })?;

                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).map_err(|e| ThumbnailError {
                    message: format!("Failed to read file contents: {}", e),
                })?;

                return Ok((file_name, buffer));
            }
        }

        Err(ThumbnailError {
            message: "No image files found in archive".to_string(),
        })
    }

    // 7zサポートは一時的に無効化
    /*
    fn get_first_image_from_7z(&self, archive_path: &Path) -> Result<(String, Vec<u8>), ThumbnailError> {
        let file = File::open(archive_path).map_err(|e| ThumbnailError {
            message: format!("Failed to open 7z file: {}", e),
        })?;

        let mut reader = SevenZReader::new(file).map_err(|e| ThumbnailError {
            message: format!("Failed to read 7z archive: {}", e),
        })?;

        // アーカイブ内のファイル名を取得してソート
        let mut file_names: Vec<String> = Vec::new();
        for entry in reader.archive().files.iter() {
            if !entry.is_directory() {
                file_names.push(entry.name().to_string());
            }
        }

        // ファイル名でソート
        file_names.sort();

        // 最初の画像ファイルを探す
        for file_name in file_names {
            let file_path = Path::new(&file_name);
            if Self::is_image_file(file_path) {
                let mut buffer = Vec::new();
                reader.for_each_entries(|entry, reader| {
                    if entry.name() == file_name {
                        reader.read_to_end(&mut buffer).map_err(|e| ThumbnailError {
                            message: format!("Failed to read file contents: {}", e),
                        })?;
                    }
                    Ok(true)
                }).map_err(|e| ThumbnailError {
                    message: format!("Failed to extract from 7z: {}", e),
                })?;

                if !buffer.is_empty() {
                    return Ok((file_name, buffer));
                }
            }
        }

        Err(ThumbnailError {
            message: "No image files found in 7z archive".to_string(),
        })
    }
    */

    fn get_first_image_from_rar(&self, archive_path: &Path) -> Result<(String, Vec<u8>), ThumbnailError> {
        let mut archive = RarArchive::new(archive_path.to_str().unwrap())
            .open_for_processing()
            .map_err(|e| ThumbnailError {
                message: format!("Failed to open RAR archive: {}", e),
            })?;

        // アーカイブ内のファイル名を取得してソート
        let mut file_names: Vec<String> = Vec::new();
        while let Some(header) = archive.read_header().map_err(|e| ThumbnailError {
            message: format!("Failed to read RAR header: {}", e),
        })? {
            if !header.entry().is_directory() {
                file_names.push(header.entry().filename.to_string());
            }
            archive.skip().map_err(|e| ThumbnailError {
                message: format!("Failed to skip RAR entry: {}", e),
            })?;
        }

        // ファイル名でソート
        file_names.sort();

        // 最初の画像ファイルを探す
        for file_name in file_names {
            let file_path = Path::new(&file_name);
            if Self::is_image_file(file_path) {
                // RAR ファイルを再度開いて対象ファイルを抽出
                let mut archive = RarArchive::new(archive_path.to_str().unwrap())
                    .open_for_processing()
                    .map_err(|e| ThumbnailError {
                        message: format!("Failed to reopen RAR archive: {}", e),
                    })?;

                while let Some(header) = archive.read_header().map_err(|e| ThumbnailError {
                    message: format!("Failed to read RAR header: {}", e),
                })? {
                    if header.entry().filename == file_name {
                        let mut buffer = Vec::new();
                        archive.read_data(&mut buffer).map_err(|e| ThumbnailError {
                            message: format!("Failed to read RAR file contents: {}", e),
                        })?;
                        return Ok((file_name, buffer));
                    } else {
                        archive.skip().map_err(|e| ThumbnailError {
                            message: format!("Failed to skip RAR entry: {}", e),
                        })?;
                    }
                }
            }
        }

        Err(ThumbnailError {
            message: "No image files found in RAR archive".to_string(),
        })
    }

    fn get_first_image_from_tar(&self, archive_path: &Path, is_gzipped: bool) -> Result<(String, Vec<u8>), ThumbnailError> {
        let file = File::open(archive_path).map_err(|e| ThumbnailError {
            message: format!("Failed to open tar file: {}", e),
        })?;

        let mut file_names: Vec<String> = Vec::new();
        
        if is_gzipped {
            let decoder = GzDecoder::new(file);
            let mut archive = TarArchive::new(decoder);
            
            // ファイル名を収集
            for entry in archive.entries().map_err(|e| ThumbnailError {
                message: format!("Failed to read tar.gz entries: {}", e),
            })? {
                let entry = entry.map_err(|e| ThumbnailError {
                    message: format!("Failed to read tar.gz entry: {}", e),
                })?;
                
                if entry.header().entry_type().is_file() {
                    if let Ok(path) = entry.path() {
                        file_names.push(path.to_string_lossy().to_string());
                    }
                }
            }
        } else {
            let mut archive = TarArchive::new(file);
            
            // ファイル名を収集
            for entry in archive.entries().map_err(|e| ThumbnailError {
                message: format!("Failed to read tar entries: {}", e),
            })? {
                let entry = entry.map_err(|e| ThumbnailError {
                    message: format!("Failed to read tar entry: {}", e),
                })?;
                
                if entry.header().entry_type().is_file() {
                    if let Ok(path) = entry.path() {
                        file_names.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }

        // ファイル名でソート
        file_names.sort();

        // 最初の画像ファイルを探す
        for file_name in file_names {
            let file_path = Path::new(&file_name);
            if Self::is_image_file(file_path) {
                // tarファイルを再度開いて対象ファイルを抽出
                let file = File::open(archive_path).map_err(|e| ThumbnailError {
                    message: format!("Failed to reopen tar file: {}", e),
                })?;

                if is_gzipped {
                    let decoder = GzDecoder::new(file);
                    let mut archive = TarArchive::new(decoder);
                    
                    for entry in archive.entries().map_err(|e| ThumbnailError {
                        message: format!("Failed to read tar.gz entries: {}", e),
                    })? {
                        let mut entry = entry.map_err(|e| ThumbnailError {
                            message: format!("Failed to read tar.gz entry: {}", e),
                        })?;
                        
                        if let Ok(path) = entry.path() {
                            if path.to_string_lossy() == file_name {
                                let mut buffer = Vec::new();
                                entry.read_to_end(&mut buffer).map_err(|e| ThumbnailError {
                                    message: format!("Failed to read tar.gz file contents: {}", e),
                                })?;
                                return Ok((file_name, buffer));
                            }
                        }
                    }
                } else {
                    let mut archive = TarArchive::new(file);
                    
                    for entry in archive.entries().map_err(|e| ThumbnailError {
                        message: format!("Failed to read tar entries: {}", e),
                    })? {
                        let mut entry = entry.map_err(|e| ThumbnailError {
                            message: format!("Failed to read tar entry: {}", e),
                        })?;
                        
                        if let Ok(path) = entry.path() {
                            if path.to_string_lossy() == file_name {
                                let mut buffer = Vec::new();
                                entry.read_to_end(&mut buffer).map_err(|e| ThumbnailError {
                                    message: format!("Failed to read tar file contents: {}", e),
                                })?;
                                return Ok((file_name, buffer));
                            }
                        }
                    }
                }
            }
        }

        Err(ThumbnailError {
            message: "No image files found in tar archive".to_string(),
        })
    }

    pub fn generate_archive_thumbnail(&self, archive_path: &Path) -> Result<PathBuf, ThumbnailError> {
        let thumbnail_path = self.get_archive_thumbnail_path(archive_path);

        // 既にサムネイルが存在する場合はそれを返す
        if thumbnail_path.exists() {
            return Ok(thumbnail_path);
        }

        // ファイル拡張子を取得
        let extension = archive_path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
            .ok_or_else(|| ThumbnailError {
                message: "Invalid file extension".to_string(),
            })?;

        // アーカイブの種類に応じて画像を抽出
        let (_file_name, image_data) = match extension.as_str() {
            "zip" => self.get_first_image_from_zip(archive_path)?,
            "7z" => {
                return Err(ThumbnailError {
                    message: "7z format is not yet supported".to_string(),
                });
            },
            "rar" => self.get_first_image_from_rar(archive_path)?,
            "tar" => self.get_first_image_from_tar(archive_path, false)?,
            "gz" => {
                // .tar.gz形式かどうかチェック
                let file_name = archive_path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("");
                if file_name.ends_with(".tar.gz") {
                    self.get_first_image_from_tar(archive_path, true)?
                } else {
                    return Err(ThumbnailError {
                        message: "Unsupported .gz format (only .tar.gz is supported)".to_string(),
                    });
                }
            },
            _ => {
                return Err(ThumbnailError {
                    message: format!("Unsupported archive format: {}", extension),
                });
            }
        };

        // 画像データを一時的にファイルに保存
        let temp_image_path = thumbnail_path.with_extension("temp");
        std::fs::write(&temp_image_path, image_data).map_err(|e| ThumbnailError {
            message: format!("Failed to write temporary image file: {}", e),
        })?;

        // ffmpegを使用してサムネイルを生成
        let output = Command::new("ffmpeg")
            .args([
                "-i",
                temp_image_path.to_str().unwrap(),
                "-vf",
                "scale=192:192:force_original_aspect_ratio=increase,crop=192:192",
                "-frames:v",
                "1",
                "-q:v",
                "2",
                thumbnail_path.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| ThumbnailError {
                message: format!("Failed to execute ffmpeg: {}", e),
            })?;

        // 一時ファイルを削除
        let _ = std::fs::remove_file(&temp_image_path);

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(ThumbnailError {
                message: format!("ffmpeg failed: {}", error_message),
            });
        }

        if !thumbnail_path.exists() {
            return Err(ThumbnailError {
                message: "Thumbnail file was not created".to_string(),
            });
        }

        Ok(thumbnail_path)
    }
}

// Tauriコマンド関数
#[tauri::command]
pub async fn generate_video_thumbnail(file_path: String) -> Result<String, String> {
    let video_path = Path::new(&file_path);

    if !video_path.exists() {
        return Err("Video file does not exist".to_string());
    }

    if !ThumbnailGenerator::is_video_file(video_path) {
        return Err("File is not a video".to_string());
    }

    let generator = ThumbnailGenerator::new().map_err(|e| e.message)?;

    let thumbnail_path = generator
        .generate_thumbnail(video_path)
        .map_err(|e| e.message)?;

    Ok(thumbnail_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn cleanup_thumbnail_cache() -> Result<(), String> {
    let generator = ThumbnailGenerator::new().map_err(|e| e.message)?;

    generator.cleanup_old_thumbnails().map_err(|e| e.message)?;

    Ok(())
}

#[tauri::command]
pub async fn get_thumbnail_cache_size() -> Result<u64, String> {
    let generator = ThumbnailGenerator::new().map_err(|e| e.message)?;

    generator.get_cache_size().map_err(|e| e.message)
}

#[tauri::command]
pub async fn extract_audio_album_art(file_path: String) -> Result<String, String> {
    let audio_path = Path::new(&file_path);

    if !audio_path.exists() {
        return Err("Audio file does not exist".to_string());
    }

    if !ThumbnailGenerator::is_audio_file(audio_path) {
        return Err("File is not an audio file".to_string());
    }

    let generator = ThumbnailGenerator::new().map_err(|e| e.message)?;

    let thumbnail_path = generator
        .extract_album_art(audio_path)
        .map_err(|e| e.message)?;

    Ok(thumbnail_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn generate_archive_thumbnail(file_path: String) -> Result<String, String> {
    let archive_path = Path::new(&file_path);

    if !archive_path.exists() {
        return Err("Archive file does not exist".to_string());
    }

    if !ThumbnailGenerator::is_archive_file(archive_path) {
        return Err("File is not an archive".to_string());
    }

    let generator = ThumbnailGenerator::new().map_err(|e| e.message)?;

    let thumbnail_path = generator
        .generate_archive_thumbnail(archive_path)
        .map_err(|e| e.message)?;

    Ok(thumbnail_path.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_is_video_file() {
        assert!(ThumbnailGenerator::is_video_file(Path::new("test.mp4")));
        assert!(ThumbnailGenerator::is_video_file(Path::new("test.avi")));
        assert!(ThumbnailGenerator::is_video_file(Path::new("test.mov")));
        assert!(!ThumbnailGenerator::is_video_file(Path::new("test.jpg")));
        assert!(!ThumbnailGenerator::is_video_file(Path::new("test.txt")));
    }

    #[test]
    fn test_get_cache_dir() {
        let cache_dir = ThumbnailGenerator::get_cache_dir().unwrap();
        assert!(cache_dir.to_string_lossy().contains("thumbnails"));
    }
}

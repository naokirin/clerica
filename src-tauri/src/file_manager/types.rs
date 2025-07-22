use crate::database::{File, Tag};
use std::fmt::{Display, Formatter};
use thiserror::Error;

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
}

impl Display for FileCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileCategory::Image => write!(f, "Image"),
            FileCategory::Video => write!(f, "Video"),
            FileCategory::Audio => write!(f, "Audio"),
            FileCategory::Document => write!(f, "Document"),
            FileCategory::Text => write!(f, "Text"),
            FileCategory::Programming => write!(f, "Programming"),
            FileCategory::Archive => write!(f, "Archive"),
            FileCategory::Other => write!(f, "Other"),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct DirectoryRemovalResult {
    pub success: bool,
    pub deleted_tags_count: usize,
    pub deleted_tag_ids: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct DeleteResult {
    pub deleted_files: Vec<String>,
    pub failed_files: Vec<(String, String)>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BatchRenameOperation {
    pub file_id: String,
    pub old_name: String,
    pub new_name: String,
}

#[derive(Debug, serde::Serialize)]
pub struct BatchRenameResult {
    pub success_count: usize,
    pub failed_operations: Vec<(String, String)>,
}

#[allow(dead_code)]
pub type RenameResult<T> = Result<T, RenameError>;

#[derive(Debug, Error)]
#[allow(clippy::enum_variant_names)]
#[allow(dead_code)]
pub enum RenameError {
    #[error("Template compilation failed: {0}")]
    Template(String),
    #[error("Template render failed: {0}")]
    Render(String),
    #[error("File operation failed: {0}")]
    File(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Regex compilation failed: {0}")]
    Regex(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Conflicting filename: {0} already exists")]
    Conflict(String),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AdvancedBatchRenameOperation {
    pub file_ids: Vec<String>,
    pub pattern: String,
    pub replacement: String,
    pub use_regex: bool,
    pub case_sensitive: bool,
}

#[derive(Debug, serde::Serialize)]
pub struct AdvancedBatchRenamePreview {
    pub file_id: String,
    pub old_name: String,
    pub new_name: String,
    pub success: bool,
    pub error_message: Option<String>,
}
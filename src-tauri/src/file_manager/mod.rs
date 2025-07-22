// File Manager Module
// Provides organized file and directory management functionality for Clerica

// Submodules
pub mod types;
pub mod directories;
pub mod files;
pub mod tags;

// Re-export commonly used types
pub use types::{
    FileWithTags, FileCategory
};


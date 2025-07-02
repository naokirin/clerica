use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExifConfig {
    pub exif_tags: HashMap<u32, String>,
    pub orientation_values: HashMap<u8, String>,
    pub color_space_values: HashMap<u16, String>,
    pub metering_mode_values: HashMap<u8, String>,
    pub light_source_values: HashMap<u8, String>,
    pub white_balance_values: HashMap<u8, String>,
    pub scene_capture_type_values: HashMap<u8, String>,
    pub enhancement_values: HashMap<u8, String>,
    pub subject_distance_range_values: HashMap<u8, String>,
}

impl ExifConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: ExifConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

// グローバルな設定インスタンス
use std::sync::OnceLock;

static EXIF_CONFIG: OnceLock<ExifConfig> = OnceLock::new();

pub fn initialize_exif_config() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "config/exif_tags.yaml";
    let config = ExifConfig::load_from_file(config_path)?;
    EXIF_CONFIG.set(config).map_err(|_| "Failed to set EXIF config")?;
    Ok(())
}

pub fn get_exif_config() -> Option<&'static ExifConfig> {
    EXIF_CONFIG.get()
}

// Tauriコマンドとして公開
#[tauri::command]
pub fn get_exif_config_data() -> Result<ExifConfig, String> {
    match get_exif_config() {
        Some(config) => Ok(config.clone()),
        None => Err("EXIF設定が初期化されていません".to_string()),
    }
}
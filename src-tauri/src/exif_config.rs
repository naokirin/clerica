use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::exif_constants::*;

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
    pub fn new() -> Self {
        Self {
            exif_tags: get_exif_tag_names().into_iter().map(|(k, v)| (k, v.to_string())).collect(),
            orientation_values: get_orientation_values().into_iter().map(|(k, v)| (k, v.to_string())).collect(),
            color_space_values: get_color_space_values().into_iter().map(|(k, v)| (k, v.to_string())).collect(),
            metering_mode_values: get_metering_mode_values().into_iter().map(|(k, v)| (k, v.to_string())).collect(),
            light_source_values: get_light_source_values().into_iter().map(|(k, v)| (k, v.to_string())).collect(),
            white_balance_values: get_white_balance_values().into_iter().map(|(k, v)| (k, v.to_string())).collect(),
            scene_capture_type_values: get_scene_capture_type_values().into_iter().map(|(k, v)| (k, v.to_string())).collect(),
            enhancement_values: get_enhancement_values().into_iter().map(|(k, v)| (k, v.to_string())).collect(),
            subject_distance_range_values: get_subject_distance_range_values().into_iter().map(|(k, v)| (k, v.to_string())).collect(),
        }
    }
}

// グローバルな設定インスタンス
use std::sync::OnceLock;

static EXIF_CONFIG: OnceLock<ExifConfig> = OnceLock::new();

pub fn initialize_exif_config() -> Result<(), Box<dyn std::error::Error>> {
    let config = ExifConfig::new();
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
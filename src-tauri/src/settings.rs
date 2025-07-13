use crate::ShelfManager;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::error::Error;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub id: i64,
    pub key: String,
    pub value: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub show_hidden_files: bool,
    pub show_directories: bool,
    pub files_per_page: i32,
    pub auto_tag_directories: bool,
    pub auto_tag_threshold: f64,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            show_hidden_files: false,
            show_directories: true,
            files_per_page: 20,
            auto_tag_directories: true,
            auto_tag_threshold: 0.5,
        }
    }
}

pub async fn get_setting(pool: &SqlitePool, key: &str) -> Result<Option<String>, Box<dyn Error>> {
    let result = sqlx::query_scalar::<_, String>("SELECT value FROM settings WHERE key = ?")
        .bind(key)
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

pub async fn set_setting(pool: &SqlitePool, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        "INSERT INTO settings (key, value) VALUES (?, ?) 
         ON CONFLICT(key) DO UPDATE SET value = ?, updated_at = CURRENT_TIMESTAMP",
    )
    .bind(key)
    .bind(value)
    .bind(value)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_all_settings(pool: &SqlitePool) -> Result<AppSettings, Box<dyn Error>> {
    let show_hidden_files = get_setting(pool, "show_hidden_files")
        .await?
        .unwrap_or_else(|| "false".to_string());

    let show_directories = get_setting(pool, "show_directories")
        .await?
        .unwrap_or_else(|| "true".to_string());

    let files_per_page = get_setting(pool, "files_per_page")
        .await?
        .unwrap_or_else(|| "20".to_string());

    let auto_tag_directories = get_setting(pool, "auto_tag_directories")
        .await?
        .unwrap_or_else(|| "true".to_string());

    let auto_tag_threshold = get_setting(pool, "auto_tag_threshold")
        .await?
        .unwrap_or_else(|| "0.7".to_string());

    Ok(AppSettings {
        show_hidden_files: show_hidden_files == "true",
        show_directories: show_directories == "true",
        files_per_page: files_per_page.parse().unwrap_or(20),
        auto_tag_directories: auto_tag_directories == "true",
        auto_tag_threshold: auto_tag_threshold.parse().unwrap_or(0.7),
    })
}

pub async fn update_setting_bool(
    pool: &SqlitePool,
    key: &str,
    value: bool,
) -> Result<(), Box<dyn Error>> {
    let value_str = if value { "true" } else { "false" };
    set_setting(pool, key, value_str).await
}

pub async fn update_setting_int(
    pool: &SqlitePool,
    key: &str,
    value: i32,
) -> Result<(), Box<dyn Error>> {
    set_setting(pool, key, &value.to_string()).await
}

pub async fn update_setting_float(
    pool: &SqlitePool,
    key: &str,
    value: f64,
) -> Result<(), Box<dyn Error>> {
    set_setting(pool, key, &value.to_string()).await
}

pub async fn update_setting_string(
    pool: &SqlitePool,
    key: &str,
    value: &str,
) -> Result<(), Box<dyn Error>> {
    set_setting(pool, key, value).await
}

#[tauri::command]
pub async fn get_settings(pools: tauri::State<'_, ShelfManager>) -> Result<AppSettings, String> {
    get_all_settings(pools.get_settings_pool())
        .await
        .map_err(|e| format!("Failed to get settings: {}", e))
}

#[tauri::command]
pub async fn update_setting_bool_cmd(
    pools: tauri::State<'_, ShelfManager>,
    key: String,
    value: bool,
) -> Result<(), String> {
    update_setting_bool(pools.get_settings_pool(), &key, value)
        .await
        .map_err(|e| format!("Failed to update setting: {}", e))
}

#[tauri::command]
pub async fn update_setting_int_cmd(
    pools: tauri::State<'_, ShelfManager>,
    key: String,
    value: i32,
) -> Result<(), String> {
    update_setting_int(pools.get_settings_pool(), &key, value)
        .await
        .map_err(|e| format!("Failed to update setting: {}", e))
}

#[tauri::command]
pub async fn update_setting_float_cmd(
    pools: tauri::State<'_, ShelfManager>,
    key: String,
    value: f64,
) -> Result<(), String> {
    update_setting_float(pools.get_settings_pool(), &key, value)
        .await
        .map_err(|e| format!("Failed to update setting: {}", e))
}

#[tauri::command]
pub async fn update_setting_string_cmd(
    pools: tauri::State<'_, ShelfManager>,
    key: String,
    value: String,
) -> Result<(), String> {
    update_setting_string(pools.get_settings_pool(), &key, &value)
        .await
        .map_err(|e| format!("Failed to update setting: {}", e))
}

#[tauri::command]
pub async fn get_language_setting(
    pools: tauri::State<'_, ShelfManager>,
) -> Result<String, String> {
    get_setting(pools.get_settings_pool(), "language")
        .await
        .map(|lang| lang.unwrap_or_else(|| "ja".to_string()))
        .map_err(|e| format!("Failed to get language setting: {}", e))
}

pub fn is_hidden_file(path: &str) -> bool {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

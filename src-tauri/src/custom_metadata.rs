use tauri::State;
use uuid::Uuid;
use chrono::Utc;

use crate::database::{Database, DatabaseTrait, CustomMetadataKey, CustomMetadataValue};
use crate::group_manager::GroupManager;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateCustomMetadataKeyRequest {
    pub name: String,
    pub display_name: String,
    pub data_type: String,
    pub description: Option<String>,
    pub is_required: bool,
    pub default_value: Option<String>,
    pub validation_pattern: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdateCustomMetadataKeyRequest {
    pub id: String,
    pub display_name: String,
    pub data_type: String,
    pub description: Option<String>,
    pub is_required: bool,
    pub default_value: Option<String>,
    pub validation_pattern: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SetCustomMetadataValueRequest {
    pub file_id: String,
    pub key_id: String,
    pub value: Option<String>,
}

/// カスタムメタデータキーを作成
#[tauri::command]
pub async fn create_custom_metadata_key(
    db_manager: State<'_, GroupManager>,
    request: CreateCustomMetadataKeyRequest,
) -> Result<CustomMetadataKey, String> {
    // データ型の妥当性チェック
    let valid_data_types = ["text", "number", "date", "boolean", "json"];
    if !valid_data_types.contains(&request.data_type.as_str()) {
        return Err(format!(
            "Invalid data type: {}. Valid types are: {:?}",
            request.data_type, valid_data_types
        ));
    }

    // 同じ名前のキーが既に存在しないかチェック
    let db = Database;
    if let Ok(Some(_)) = db.get_custom_metadata_key_by_name(db_manager.get_settings_pool(), &request.name).await {
        return Err(format!("カスタムメタデータキー '{}' は既に存在します", request.name));
    }

    let key = CustomMetadataKey {
        id: Uuid::new_v4().to_string(),
        name: request.name,
        display_name: request.display_name,
        data_type: request.data_type,
        description: request.description,
        is_required: request.is_required,
        default_value: request.default_value,
        validation_pattern: request.validation_pattern,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    match db.create_custom_metadata_key(db_manager.get_settings_pool(), &key).await {
        Ok(created_key) => Ok(created_key),
        Err(e) => Err(format!("カスタムメタデータキーの作成に失敗しました: {e}")),
    }
}

/// 全てのカスタムメタデータキーを取得
#[tauri::command]
pub async fn get_custom_metadata_keys(
    db_manager: State<'_, GroupManager>,
) -> Result<Vec<CustomMetadataKey>, String> {
    let db = Database;
    match db.get_all_custom_metadata_keys(db_manager.get_settings_pool()).await {
        Ok(keys) => Ok(keys),
        Err(e) => Err(format!("カスタムメタデータキーの取得に失敗しました: {e}")),
    }
}

/// カスタムメタデータキーを更新
#[tauri::command]
pub async fn update_custom_metadata_key(
    db_manager: State<'_, GroupManager>,
    request: UpdateCustomMetadataKeyRequest,
) -> Result<CustomMetadataKey, String> {
    // データ型の妥当性チェック
    let valid_data_types = ["text", "number", "date", "boolean", "json"];
    if !valid_data_types.contains(&request.data_type.as_str()) {
        return Err(format!(
            "Invalid data type: {}. Valid types are: {:?}",
            request.data_type, valid_data_types
        ));
    }

    let db = Database;

    // 既存のキーを取得してnameを保持
    let keys = match db.get_all_custom_metadata_keys(db_manager.get_settings_pool()).await {
        Ok(keys) => keys,
        Err(e) => return Err(format!("カスタムメタデータキーの取得に失敗しました: {e}")),
    };

    let existing_key = keys.iter().find(|k| k.id == request.id);
    let existing_key = match existing_key {
        Some(key) => key,
        None => return Err("指定されたカスタムメタデータキーが見つかりません".to_string()),
    };

    let updated_key = CustomMetadataKey {
        id: request.id,
        name: existing_key.name.clone(), // nameは変更不可
        display_name: request.display_name,
        data_type: request.data_type,
        description: request.description,
        is_required: request.is_required,
        default_value: request.default_value,
        validation_pattern: request.validation_pattern,
        created_at: existing_key.created_at,
        updated_at: Utc::now(),
    };

    match db.update_custom_metadata_key(db_manager.get_settings_pool(), &updated_key).await {
        Ok(key) => Ok(key),
        Err(e) => Err(format!("カスタムメタデータキーの更新に失敗しました: {e}")),
    }
}

/// カスタムメタデータキーを削除
#[tauri::command]
pub async fn delete_custom_metadata_key(
    db_manager: State<'_, GroupManager>,
    key_id: String,
) -> Result<(), String> {
    let db = Database;
    match db.delete_custom_metadata_key(db_manager.get_settings_pool(), &db_manager.get_active_data_pool().map_err(|e| e.to_string())?, &key_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("カスタムメタデータキーの削除に失敗しました: {e}")),
    }
}

/// 名前でカスタムメタデータキーを取得
#[tauri::command]
pub async fn get_custom_metadata_key_by_name(
    db_manager: State<'_, GroupManager>,
    name: String,
) -> Result<Option<CustomMetadataKey>, String> {
    let db = Database;
    match db.get_custom_metadata_key_by_name(db_manager.get_settings_pool(), &name).await {
        Ok(key) => Ok(key),
        Err(e) => Err(format!("カスタムメタデータキーの取得に失敗しました: {e}")),
    }
}

/// カスタムメタデータ値を設定
#[tauri::command]
pub async fn set_custom_metadata_value(
    db_manager: State<'_, GroupManager>,
    request: SetCustomMetadataValueRequest,
) -> Result<CustomMetadataValue, String> {
    let db = Database;
    match db.set_custom_metadata_value(&db_manager.get_active_data_pool().map_err(|e| e.to_string())?, db_manager.get_settings_pool(), &request.file_id, &request.key_id, request.value).await {
        Ok(value) => Ok(value),
        Err(e) => Err(format!("カスタムメタデータ値の設定に失敗しました: {e}")),
    }
}

/// ファイルのカスタムメタデータ値を全て取得
#[tauri::command]
pub async fn get_custom_metadata_values_by_file(
    db_manager: State<'_, GroupManager>,
    file_id: String,
) -> Result<Vec<CustomMetadataValue>, String> {
    let db = Database;
    match db.get_custom_metadata_values_by_file(&db_manager.get_active_data_pool().map_err(|e| e.to_string())?, &file_id).await {
        Ok(values) => Ok(values),
        Err(e) => Err(format!("カスタムメタデータ値の取得に失敗しました: {e}")),
    }
}

/// 特定のカスタムメタデータ値を取得
#[tauri::command]
pub async fn get_custom_metadata_value(
    db_manager: State<'_, GroupManager>,
    file_id: String,
    key_id: String,
) -> Result<Option<CustomMetadataValue>, String> {
    let db = Database;
    match db.get_custom_metadata_value(&db_manager.get_active_data_pool().map_err(|e| e.to_string())?, &file_id, &key_id).await {
        Ok(value) => Ok(value),
        Err(e) => Err(format!("カスタムメタデータ値の取得に失敗しました: {e}")),
    }
}

/// カスタムメタデータ値を削除
#[tauri::command]
pub async fn delete_custom_metadata_value(
    db_manager: State<'_, GroupManager>,
    file_id: String,
    key_id: String,
) -> Result<(), String> {
    let db = Database;
    match db.delete_custom_metadata_value(&db_manager.get_active_data_pool().map_err(|e| e.to_string())?, &file_id, &key_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("カスタムメタデータ値の削除に失敗しました: {e}")),
    }
}
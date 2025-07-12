use crate::group_manager::{Group, GroupManager};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGroupRequest {
    pub id: String,
    pub name: String,
}

#[tauri::command]
pub async fn get_groups(group_manager: State<'_, GroupManager>) -> Result<Vec<Group>, String> {
    group_manager
        .get_groups()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_active_group_id(group_manager: State<'_, GroupManager>) -> Result<String, String> {
    Ok(group_manager.get_active_group_id_sync())
}

#[tauri::command]
pub async fn create_group(
    group_manager: State<'_, GroupManager>,
    request: CreateGroupRequest,
) -> Result<Group, String> {
    let group = Group {
        id: Uuid::new_v4().to_string(),
        name: request.name,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    group_manager
        .create_group(&group)
        .await
        .map_err(|e| e.to_string())?;

    Ok(group)
}

#[tauri::command]
pub async fn switch_group(
    group_manager: State<'_, GroupManager>,
    group_id: String,
) -> Result<(), String> {
    group_manager
        .switch_group(&group_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_group(
    group_manager: State<'_, GroupManager>,
    group_id: String,
) -> Result<(), String> {
    group_manager
        .delete_group(&group_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_group_name(
    group_manager: State<'_, GroupManager>,
    request: UpdateGroupRequest,
) -> Result<(), String> {
    let settings_pool = &group_manager.settings_pool;
    
    sqlx::query(
        "UPDATE groups SET name = ? WHERE id = ?"
    )
    .bind(&request.name)
    .bind(&request.id)
    .execute(settings_pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
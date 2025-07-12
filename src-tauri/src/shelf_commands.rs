use crate::{Shelf, ShelfManager};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateShelfRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateShelfRequest {
    pub id: String,
    pub name: String,
}

#[tauri::command]
pub async fn get_shelves(shelf_manager: State<'_, ShelfManager>) -> Result<Vec<Shelf>, String> {
    shelf_manager
        .get_shelves()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_active_shelf_id(shelf_manager: State<'_, ShelfManager>) -> Result<String, String> {
    Ok(shelf_manager.get_active_shelf_id_sync())
}

#[tauri::command]
pub async fn create_shelf(
    shelf_manager: State<'_, ShelfManager>,
    request: CreateShelfRequest,
) -> Result<Shelf, String> {
    let shelf = Shelf {
        id: Uuid::new_v4().to_string(),
        name: request.name,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    shelf_manager
        .create_shelf(&shelf)
        .await
        .map_err(|e| e.to_string())?;

    Ok(shelf)
}

#[tauri::command]
pub async fn switch_shelf(
    shelf_manager: State<'_, ShelfManager>,
    shelf_id: String,
) -> Result<(), String> {
    shelf_manager
        .switch_shelf(&shelf_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_shelf(
    shelf_manager: State<'_, ShelfManager>,
    shelf_id: String,
) -> Result<(), String> {
    shelf_manager
        .delete_shelf(&shelf_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_shelf_name(
    shelf_manager: State<'_, ShelfManager>,
    request: UpdateShelfRequest,
) -> Result<(), String> {
    let settings_pool = &shelf_manager.settings_pool;
    
    sqlx::query(
        "UPDATE shelves SET name = ? WHERE id = ?"
    )
    .bind(&request.name)
    .bind(&request.id)
    .execute(settings_pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
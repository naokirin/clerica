use crate::database::{Database, DatabaseTrait, Tag};
use crate::ShelfManager;
use tauri::State;

#[tauri::command]
pub async fn update_file_tags(
    pools: State<'_, ShelfManager>,
    file_id: String,
    tag_ids: Vec<String>,
) -> Result<(), String> {
    let db = Database;
    let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
    
    // 既存のタグを削除
    let current_tags = db.get_file_tags(&data_pool, &file_id)
        .await
        .map_err(|e| e.to_string())?;
    
    for tag in current_tags {
        db.remove_file_tag(&data_pool, &file_id, &tag.id)
            .await
            .map_err(|e| e.to_string())?;
    }
    
    // 新しいタグを追加
    for tag_id in tag_ids {
        db.add_file_tag(&data_pool, &file_id, &tag_id)
            .await
            .map_err(|e| e.to_string())?;
    }
    
    // 未参照タグを削除
    db.delete_orphaned_tags(&data_pool)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_file_tags(
    pools: State<'_, ShelfManager>,
    file_id: String,
) -> Result<Vec<Tag>, String> {
    let db = Database;
    let data_pool = pools.get_active_data_pool().map_err(|e| e.to_string())?;
    db.get_file_tags(&data_pool, &file_id)
        .await
        .map_err(|e| e.to_string())
}
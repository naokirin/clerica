use crate::database::{self, File, Tag};
use sqlx::{SqlitePool, Row};
use tauri::State;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    pub file: File,
    pub tags: Vec<Tag>,
    pub score: f64,
}

#[tauri::command]
pub async fn search_files(
    pool: State<'_, SqlitePool>,
    query: String,
    tag_ids: Option<Vec<String>>,
    sort_by: Option<String>,
) -> Result<Vec<SearchResult>, String> {
    let mut sql = String::from(
        "SELECT f.*, GROUP_CONCAT(t.name) as tag_names 
         FROM files f 
         LEFT JOIN file_tags ft ON f.id = ft.file_id 
         LEFT JOIN tags t ON ft.tag_id = t.id"
    );
    
    let mut conditions = Vec::new();
    let mut params: Vec<String> = Vec::new();
    
    // ファイル名検索
    if !query.is_empty() {
        conditions.push("f.name LIKE ?".to_string());
        params.push(format!("%{}%", query));
    }
    
    // タグフィルタ
    if let Some(tag_ids) = tag_ids {
        if !tag_ids.is_empty() {
            let placeholders = tag_ids.iter().map(|_| "?".to_string()).collect::<Vec<_>>().join(",");
            conditions.push(format!("ft.tag_id IN ({})", placeholders));
            params.extend(tag_ids);
        }
    }
    
    if !conditions.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&conditions.join(" AND "));
    }
    
    sql.push_str(" GROUP BY f.id");
    
    // ソート
    match sort_by.as_deref() {
        Some("name") => sql.push_str(" ORDER BY f.name"),
        Some("modified") => sql.push_str(" ORDER BY f.modified_at DESC"),
        Some("created") => sql.push_str(" ORDER BY f.created_at DESC"),
        Some("size") => sql.push_str(" ORDER BY f.size DESC"),
        _ => sql.push_str(" ORDER BY f.name"),
    }
    
    // クエリ実行
    let mut query_builder = sqlx::query(&sql);
    for param in params {
        query_builder = query_builder.bind(param);
    }
    
    let rows = query_builder
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    
    let mut results = Vec::new();
    for row in rows {
        let file = File {
            id: row.get("id"),
            path: row.get("path"),
            name: row.get("name"),
            directory_id: row.get("directory_id"),
            size: row.get("size"),
            file_type: row.get("file_type"),
            created_at: row.get("created_at"),
            modified_at: row.get("modified_at"),
            birth_time: row.get("birth_time"),
            inode: row.get("inode"),
            is_directory: row.get("is_directory"),
            created_at_db: row.get("created_at_db"),
            updated_at_db: row.get("updated_at_db"),
        };
        
        let tag_names: Option<String> = row.get("tag_names");
        let tags = if let Some(names) = tag_names {
            names.split(',')
                .filter(|s| !s.is_empty())
                .map(|name| Tag {
                    id: Uuid::new_v4().to_string(), // 仮のID
                    name: name.to_string(),
                    color: "#3B82F6".to_string(),
                    created_at: Utc::now(),
                })
                .collect()
        } else {
            Vec::new()
        };
        
        // 簡易的なスコア計算（実装予定）
        let score = 1.0;
        
        results.push(SearchResult {
            file,
            tags,
            score,
        });
    }
    
    Ok(results)
}

#[tauri::command]
pub async fn get_tags(pool: State<'_, SqlitePool>) -> Result<Vec<Tag>, String> {
    database::get_all_tags(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_tag(
    pool: State<'_, SqlitePool>,
    name: String,
    color: String,
) -> Result<Tag, String> {
    database::create_tag(&pool, &name, &color)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_tag(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM tags WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
} 
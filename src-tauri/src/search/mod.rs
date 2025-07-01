use crate::database::{Database, DatabaseTrait, File, Tag};
use chrono::Utc;
use sqlx::{Row, SqlitePool};
use tauri::State;
use uuid::Uuid;

#[cfg(test)]
mod tests;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    pub file: File,
    pub tags: Vec<Tag>,
    pub score: f64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MetadataSearchFilter {
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(rename = "keyName")]
    pub key_name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "dataType")]
    pub data_type: String,
    pub operator: String, // 'equals', 'contains', 'greater_than', 'less_than', 'not_equals'
    pub value: String,
}

#[tauri::command]
pub async fn search_files(
    pool: State<'_, SqlitePool>,
    query: String,
    tag_ids: Option<Vec<String>>,
    metadata_filters: Vec<MetadataSearchFilter>,
    sort_by: Option<String>,
) -> Result<Vec<SearchResult>, String> {

    // メタデータフィルタの有効性をチェック
    let valid_metadata_filters: Vec<&MetadataSearchFilter> = metadata_filters
            .iter()
            .filter(|f| !f.key_id.is_empty() && !f.value.is_empty())
            .collect();

    let has_metadata_filters = !valid_metadata_filters.is_empty();

    let mut sql = String::from(
        "SELECT DISTINCT f.*, GROUP_CONCAT(DISTINCT t.name) as tag_names 
         FROM files f 
         LEFT JOIN file_tags ft ON f.id = ft.file_id 
         LEFT JOIN tags t ON ft.tag_id = t.id",
    );

    // カスタムメタデータフィルタがある場合、JOINを追加
    if has_metadata_filters {
        for (i, _) in valid_metadata_filters.iter().enumerate() {
            sql.push_str(&format!(
                " LEFT JOIN custom_metadata_values cmv{} ON f.id = cmv{}.file_id",
                i, i
            ));
        }
    }

    let mut conditions = Vec::new();
    let mut params: Vec<String> = Vec::new();

    // ファイル名検索
    if !query.is_empty() {
        conditions.push("f.name LIKE ?".to_string());
        params.push(format!("%{query}%"));
    }

    // タグフィルタ
    if let Some(tag_ids) = tag_ids {
        if !tag_ids.is_empty() {
            let placeholders = tag_ids
                .iter()
                .map(|_| "?".to_string())
                .collect::<Vec<_>>()
                .join(",");
            conditions.push(format!("ft.tag_id IN ({placeholders})"));
            params.extend(tag_ids);
        }
    }

    // カスタムメタデータフィルタ
    for (i, filter) in valid_metadata_filters.iter().enumerate() {
        let metadata_condition = match filter.operator.as_str() {
            "equals" => format!("(cmv{}.key_id = ? AND cmv{}.value = ?)", i, i),
            "contains" => format!("(cmv{}.key_id = ? AND cmv{}.value LIKE ?)", i, i),
            "greater_than" => format!(
                "(cmv{}.key_id = ? AND CAST(cmv{}.value AS REAL) > CAST(? AS REAL))",
                i, i
            ),
            "less_than" => format!(
                "(cmv{}.key_id = ? AND CAST(cmv{}.value AS REAL) < CAST(? AS REAL))",
                i, i
            ),
            "not_equals" => format!("(cmv{}.key_id = ? AND cmv{}.value != ?)", i, i),
            _ => format!("(cmv{}.key_id = ? AND cmv{}.value = ?)", i, i), // デフォルトは equals
        };

        conditions.push(metadata_condition);
        params.push(filter.key_id.clone());

        if filter.operator == "contains" {
            params.push(format!("%{}%", filter.value));
        } else {
            params.push(filter.value.clone());
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
            file_size: row.get("file_size"),
            mime_type: row.get("mime_type"),
            permissions: row.get("permissions"),
            owner_uid: row.get("owner_uid"),
            group_gid: row.get("group_gid"),
            hard_links: row.get("hard_links"),
            device_id: row.get("device_id"),
            last_accessed: row.get("last_accessed"),
        };

        let tag_names: Option<String> = row.get("tag_names");
        let tags = if let Some(names) = tag_names {
            names
                .split(',')
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

        results.push(SearchResult { file, tags, score });
    }

    Ok(results)
}

#[tauri::command]
pub async fn get_tags(pool: State<'_, SqlitePool>) -> Result<Vec<Tag>, String> {
    let db = Database;
    db.get_all_tags(&pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_tag(
    pool: State<'_, SqlitePool>,
    name: String,
    color: String,
) -> Result<Tag, String> {
    let db = Database;
    db.create_tag(&pool, &name, &color)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_tag(pool: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    sqlx::query("DELETE FROM tags WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

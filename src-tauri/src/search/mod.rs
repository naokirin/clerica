use crate::database::{Database, DatabaseTrait, File, Tag};
use crate::settings;
use crate::ShelfManager;
use chrono::Utc;
use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
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
pub struct PaginatedSearchResult {
    pub results: Vec<SearchResult>,
    pub total_count: i64,
    pub category_counts: std::collections::HashMap<String, i64>,
    pub total_category_counts: std::collections::HashMap<String, i64>,
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
    pools: State<'_, ShelfManager>,
    query: String,
    tag_ids: Option<Vec<String>>,
    metadata_filters: Vec<MetadataSearchFilter>,
    metadata_logic: Option<String>,
    sort_field: Option<String>,
    sort_order: Option<String>,
    directory_id: Option<String>,
    category: Option<String>,
) -> Result<Vec<SearchResult>, String> {
    // 後方互換性のため、全件取得版を残す
    let paginated_result = search_files_paginated(
        pools,
        query,
        tag_ids,
        metadata_filters,
        metadata_logic,
        sort_field,
        sort_order,
        directory_id,
        None,
        None,
        category,
    )
    .await?;

    // 実際にはSearchResult構造体でカテゴリ別件数も返すべきだが、
    // 後方互換性のためとりあえずresultsのみ返す
    Ok(paginated_result.results)
}

#[tauri::command]
pub async fn search_files_paginated(
    pools: State<'_, ShelfManager>,
    query: String,
    tag_ids: Option<Vec<String>>,
    metadata_filters: Vec<MetadataSearchFilter>,
    metadata_logic: Option<String>,
    sort_field: Option<String>,
    sort_order: Option<String>,
    directory_id: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
    category: Option<String>,
) -> Result<PaginatedSearchResult, String> {
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
                " LEFT JOIN custom_metadata_values cmv{i} ON f.id = cmv{i}.file_id"
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

    // タグフィルタ - 複数のタグがAND条件で絞り込まれる
    if let Some(ref tag_ids) = tag_ids {
        if !tag_ids.is_empty() {
            let tag_count = tag_ids.len();
            let placeholders = tag_ids
                .iter()
                .map(|_| "?".to_string())
                .collect::<Vec<_>>()
                .join(",");
            // 指定されたすべてのタグを持つファイルのみを取得
            conditions.push(format!(
                "f.id IN (SELECT file_id FROM file_tags WHERE tag_id IN ({}) GROUP BY file_id HAVING COUNT(DISTINCT tag_id) = {})",
                placeholders, tag_count
            ));
            params.extend(tag_ids.clone());
        }
    }

    // ディレクトリフィルタ
    if let Some(dir_id) = directory_id {
        if dir_id != "all" {
            conditions.push("f.directory_id = ?".to_string());
            params.push(dir_id);
        }
    }

    // カテゴリフィルタ適用前の総件数を計算（現在のSQL + 条件をコピー）
    let pre_category_sql = sql.clone();
    let pre_category_conditions = conditions.clone();
    let pre_category_params = params.clone();

    // カテゴリフィルタ
    if let Some(ref cat) = category {
        if cat != "all" {
            match cat.as_str() {
                "image" => {
                    let image_exts = vec![
                        "jpg", "jpeg", "png", "gif", "bmp", "webp", "svg", "ico", "tiff", "raw",
                    ];
                    let ext_conditions: Vec<String> = image_exts
                        .iter()
                        .map(|ext| format!("LOWER(f.name) LIKE '%.{}'", ext))
                        .collect();
                    conditions.push(format!(
                        "(f.mime_type LIKE 'image/%' OR {} )",
                        ext_conditions.join(" OR ")
                    ));
                }
                "audio" => {
                    let audio_exts = vec!["mp3", "wav", "ogg", "flac", "aac", "m4a", "wma", "opus"];
                    let ext_conditions: Vec<String> = audio_exts
                        .iter()
                        .map(|ext| format!("LOWER(f.name) LIKE '%.{}'", ext))
                        .collect();
                    conditions.push(format!(
                        "(f.mime_type LIKE 'audio/%' OR {} )",
                        ext_conditions.join(" OR ")
                    ));
                }
                "video" => {
                    let video_exts = vec![
                        "mp4", "avi", "mov", "wmv", "flv", "webm", "mkv", "m4v", "3gp",
                    ];
                    let ext_conditions: Vec<String> = video_exts
                        .iter()
                        .map(|ext| format!("LOWER(f.name) LIKE '%.{}'", ext))
                        .collect();
                    conditions.push(format!(
                        "(f.mime_type LIKE 'video/%' OR {} )",
                        ext_conditions.join(" OR ")
                    ));
                }
                "document" => {
                    let doc_exts = vec![
                        "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "md", "html",
                        "htm", "css", "js", "json", "xml", "csv", "rtf",
                    ];
                    let ext_conditions: Vec<String> = doc_exts
                        .iter()
                        .map(|ext| format!("LOWER(f.name) LIKE '%.{}'", ext))
                        .collect();
                    conditions.push(format!("(f.mime_type LIKE 'application/pdf' OR f.mime_type LIKE 'application/msword' OR f.mime_type LIKE 'application/vnd.%' OR f.mime_type LIKE 'text/%' OR {} )", ext_conditions.join(" OR ")));
                }
                "archive" => {
                    let archive_exts = vec!["zip", "rar", "7z", "tar", "gz", "bz2", "xz", "lzma"];
                    let ext_conditions: Vec<String> = archive_exts
                        .iter()
                        .map(|ext| format!("LOWER(f.name) LIKE '%.{}'", ext))
                        .collect();
                    conditions.push(format!("(f.mime_type LIKE 'application/zip' OR f.mime_type LIKE 'application/x-rar%' OR f.mime_type LIKE 'application/x-7z%' OR f.mime_type LIKE 'application/x-tar%' OR f.mime_type LIKE 'application/gzip' OR {} )", ext_conditions.join(" OR ")));
                }
                "other" => {
                    let known_exts = vec![
                        "jpg", "jpeg", "png", "gif", "bmp", "webp", "svg", "ico", "tiff", "raw",
                        "mp3", "wav", "ogg", "flac", "aac", "m4a", "wma", "opus", "mp4", "avi",
                        "mov", "wmv", "flv", "webm", "mkv", "m4v", "3gp", "pdf", "doc", "docx",
                        "xls", "xlsx", "ppt", "pptx", "txt", "md", "html", "htm", "css", "js",
                        "json", "xml", "csv", "rtf", "zip", "rar", "7z", "tar", "gz", "bz2", "xz",
                        "lzma",
                    ];
                    let not_ext_conditions: Vec<String> = known_exts
                        .iter()
                        .map(|ext| format!("LOWER(f.name) NOT LIKE '%.{}'", ext))
                        .collect();
                    conditions.push(format!("(f.mime_type IS NULL OR (f.mime_type NOT LIKE 'image/%' AND f.mime_type NOT LIKE 'audio/%' AND f.mime_type NOT LIKE 'video/%' AND f.mime_type NOT LIKE 'application/pdf' AND f.mime_type NOT LIKE 'application/msword' AND f.mime_type NOT LIKE 'application/vnd.%' AND f.mime_type NOT LIKE 'text/%' AND f.mime_type NOT LIKE 'application/zip' AND f.mime_type NOT LIKE 'application/x-rar%' AND f.mime_type NOT LIKE 'application/x-7z%' AND f.mime_type NOT LIKE 'application/x-tar%' AND f.mime_type NOT LIKE 'application/gzip' AND {} ))", not_ext_conditions.join(" AND ")));
                }
                _ => {}
            }
        }
    }

    // カスタムメタデータフィルタ
    if !valid_metadata_filters.is_empty() {
        let metadata_logic = metadata_logic.unwrap_or_else(|| "AND".to_string());
        let mut metadata_conditions = Vec::new();

        for (i, filter) in valid_metadata_filters.iter().enumerate() {
            let metadata_condition = match filter.operator.as_str() {
                "equals" => format!("(cmv{i}.key_id = ? AND cmv{i}.value = ?)"),
                "contains" => format!("(cmv{i}.key_id = ? AND cmv{i}.value LIKE ?)"),
                "greater_than" => {
                    format!("(cmv{i}.key_id = ? AND CAST(cmv{i}.value AS REAL) > CAST(? AS REAL))")
                }
                "less_than" => {
                    format!("(cmv{i}.key_id = ? AND CAST(cmv{i}.value AS REAL) < CAST(? AS REAL))")
                }
                "not_equals" => format!("(cmv{i}.key_id = ? AND cmv{i}.value != ?)"),
                _ => format!("(cmv{i}.key_id = ? AND cmv{i}.value = ?)"), // デフォルトは equals
            };

            metadata_conditions.push(metadata_condition);
            params.push(filter.key_id.clone());

            if filter.operator == "contains" {
                params.push(format!("%{}%", filter.value));
            } else {
                params.push(filter.value.clone());
            }
        }

        if !metadata_conditions.is_empty() {
            let combined_condition = match metadata_logic.as_str() {
                "OR" => format!("({})", metadata_conditions.join(" OR ")),
                _ => format!("({})", metadata_conditions.join(" AND ")),
            };
            conditions.push(combined_condition);
        }
    }

    // 設定を取得してフィルタリング条件を追加
    let settings = settings::get_all_settings(pools.get_settings_pool())
        .await
        .map_err(|e| e.to_string())?;

    if !settings.show_hidden_files {
        conditions.push("f.name NOT LIKE '.%'".to_string());
    }
    
    if !settings.show_directories {
        conditions.push("f.is_directory = FALSE".to_string());
    }

    if !conditions.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&conditions.join(" AND "));
    }

    sql.push_str(" GROUP BY f.id");

    // ソート
    let sort_field = sort_field.as_deref().unwrap_or("modified_at");
    let sort_order = sort_order.as_deref().unwrap_or("desc");

    let sort_column = match sort_field {
        "name" => "f.name",
        "size" => "f.size",
        "created_at" => "f.created_at",
        "modified_at" => "f.modified_at",
        "last_accessed" => "f.last_accessed",
        "file_type" => "f.file_type",
        _ => "f.modified_at",
    };

    let order_direction = match sort_order {
        "asc" => "ASC",
        _ => "DESC",
    };

    sql.push_str(&format!(
        " ORDER BY {} {} NULLS LAST",
        sort_column, order_direction
    ));

    // 総件数取得用のクエリ
    let count_sql = format!(
        "SELECT COUNT(DISTINCT f.id) as total_count FROM files f {} {}",
        if has_metadata_filters {
            valid_metadata_filters
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    format!(" LEFT JOIN custom_metadata_values cmv{i} ON f.id = cmv{i}.file_id")
                })
                .collect::<Vec<_>>()
                .join("")
        } else {
            String::new()
        },
        if !conditions.is_empty() {
            format!(" WHERE {}", conditions.join(" AND "))
        } else {
            String::new()
        }
    );

    // SQLクエリとパラメータをログに出力
    println!("=== SEARCH SQL DEBUG ===");
    println!("Main SQL: {}", sql);
    println!("Count SQL: {}", count_sql);
    println!("Conditions: {:?}", conditions);
    println!("Parameters: {:?}", params);
    println!("Tag IDs: {:?}", tag_ids);
    println!("Category: {:?}", category);
    println!("========================");

    // 総件数を取得
    let mut count_query = sqlx::query(&count_sql);
    for param in &params {
        count_query = count_query.bind(param);
    }

    let total_count: i64 = count_query
        .fetch_one(&pools.get_active_data_pool().map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())?
        .get("total_count");

    // ページネーション追加
    if let (Some(limit), Some(offset)) = (limit, offset) {
        sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
    }

    // クエリ実行
    let mut query_builder = sqlx::query(&sql);
    for param in &params {
        query_builder = query_builder.bind(param);
    }

    println!("=== EXECUTING MAIN QUERY ===");
    println!("Final SQL: {}", sql);
    println!("Final Parameters: {:?}", params);
    println!("============================");

    let rows = query_builder
        .fetch_all(&pools.get_active_data_pool().map_err(|e| e.to_string())?)
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
            metadata: row.get("metadata"),
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

    // フィルタリングはSQLクエリレベルで処理されるため、ここでは不要

    // カテゴリフィルタ適用前の総件数を計算
    let mut total_sql = pre_category_sql;
    if !pre_category_conditions.is_empty() {
        total_sql.push_str(" WHERE ");
        total_sql.push_str(&pre_category_conditions.join(" AND "));
    }
    total_sql.push_str(" GROUP BY f.id");
    let total_category_counts =
        calculate_category_counts(&pools.get_active_data_pool().map_err(|e| e.to_string())?, &total_sql, &pre_category_params).await?;

    // カテゴリフィルタ適用後の件数を計算
    let category_counts = calculate_category_counts(&pools.get_active_data_pool().map_err(|e| e.to_string())?, &sql, &params).await?;

    Ok(PaginatedSearchResult {
        results,
        total_count,
        category_counts,
        total_category_counts,
    })
}

// カテゴリ別件数を計算する関数
async fn calculate_category_counts(
    pool: &SqlitePool,
    base_sql: &str,
    params: &[String],
) -> Result<HashMap<String, i64>, String> {
    // LIMITとOFFSETを除いたクエリを作成してカテゴリ別件数を計算
    let category_sql = base_sql.split(" LIMIT").next().unwrap_or(base_sql);

    let mut category_counts = HashMap::new();
    category_counts.insert("all".to_string(), 0i64);
    category_counts.insert("image".to_string(), 0i64);
    category_counts.insert("audio".to_string(), 0i64);
    category_counts.insert("video".to_string(), 0i64);
    category_counts.insert("document".to_string(), 0i64);
    category_counts.insert("archive".to_string(), 0i64);
    category_counts.insert("other".to_string(), 0i64);

    // 全ファイルを取得してカテゴリ分類
    let mut query_builder = sqlx::query(category_sql);
    for param in params {
        query_builder = query_builder.bind(param);
    }

    let rows = query_builder
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

    for row in rows {
        let file_name: String = row.get("name");
        let mime_type: Option<String> = row.get("mime_type");

        let category = classify_file_category(&file_name, mime_type.as_deref());

        *category_counts.get_mut(&category).unwrap() += 1;
        *category_counts.get_mut("all").unwrap() += 1;
    }

    Ok(category_counts)
}

// ファイルのカテゴリを分類する関数
fn classify_file_category(file_name: &str, mime_type: Option<&str>) -> String {
    let lower_name = file_name.to_lowercase();

    // MIMEタイプでの判定を優先
    if let Some(mime) = mime_type {
        if mime.starts_with("image/") {
            return "image".to_string();
        } else if mime.starts_with("audio/") {
            return "audio".to_string();
        } else if mime.starts_with("video/") {
            return "video".to_string();
        } else if mime.starts_with("application/pdf")
            || mime.starts_with("application/msword")
            || mime.starts_with("application/vnd.")
            || mime.starts_with("text/")
        {
            return "document".to_string();
        } else if mime.starts_with("application/zip")
            || mime.starts_with("application/x-rar")
            || mime.starts_with("application/x-7z")
            || mime.starts_with("application/x-tar")
            || mime.starts_with("application/gzip")
        {
            return "archive".to_string();
        }
    }

    // 拡張子での判定
    if lower_name.ends_with(".jpg")
        || lower_name.ends_with(".jpeg")
        || lower_name.ends_with(".png")
        || lower_name.ends_with(".gif")
        || lower_name.ends_with(".bmp")
        || lower_name.ends_with(".webp")
        || lower_name.ends_with(".svg")
        || lower_name.ends_with(".ico")
        || lower_name.ends_with(".tiff")
        || lower_name.ends_with(".raw")
    {
        "image".to_string()
    } else if lower_name.ends_with(".mp3")
        || lower_name.ends_with(".wav")
        || lower_name.ends_with(".ogg")
        || lower_name.ends_with(".flac")
        || lower_name.ends_with(".aac")
        || lower_name.ends_with(".m4a")
        || lower_name.ends_with(".wma")
        || lower_name.ends_with(".opus")
    {
        "audio".to_string()
    } else if lower_name.ends_with(".mp4")
        || lower_name.ends_with(".avi")
        || lower_name.ends_with(".mov")
        || lower_name.ends_with(".wmv")
        || lower_name.ends_with(".flv")
        || lower_name.ends_with(".webm")
        || lower_name.ends_with(".mkv")
        || lower_name.ends_with(".m4v")
        || lower_name.ends_with(".3gp")
    {
        "video".to_string()
    } else if lower_name.ends_with(".pdf")
        || lower_name.ends_with(".doc")
        || lower_name.ends_with(".docx")
        || lower_name.ends_with(".xls")
        || lower_name.ends_with(".xlsx")
        || lower_name.ends_with(".ppt")
        || lower_name.ends_with(".pptx")
        || lower_name.ends_with(".txt")
        || lower_name.ends_with(".md")
        || lower_name.ends_with(".html")
        || lower_name.ends_with(".htm")
        || lower_name.ends_with(".css")
        || lower_name.ends_with(".js")
        || lower_name.ends_with(".json")
        || lower_name.ends_with(".xml")
        || lower_name.ends_with(".csv")
        || lower_name.ends_with(".rtf")
    {
        "document".to_string()
    } else if lower_name.ends_with(".zip")
        || lower_name.ends_with(".rar")
        || lower_name.ends_with(".7z")
        || lower_name.ends_with(".tar")
        || lower_name.ends_with(".gz")
        || lower_name.ends_with(".bz2")
        || lower_name.ends_with(".xz")
        || lower_name.ends_with(".lzma")
    {
        "archive".to_string()
    } else {
        "other".to_string()
    }
}

#[tauri::command]
pub async fn get_tags(pools: State<'_, ShelfManager>) -> Result<Vec<Tag>, String> {
    let db = Database;
    db.get_all_tags(&pools.get_active_data_pool().map_err(|e| e.to_string())?).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_top_tags(pools: State<'_, ShelfManager>, limit: u32) -> Result<Vec<Tag>, String> {
    let db = Database;
    db.get_top_tags(&pools.get_active_data_pool().map_err(|e| e.to_string())?, limit)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_tags_by_name(
    pools: State<'_, ShelfManager>,
    query: String,
) -> Result<Vec<Tag>, String> {
    let db = Database;
    db.search_tags_by_name(&pools.get_active_data_pool().map_err(|e| e.to_string())?, &query)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_tag(
    pools: State<'_, ShelfManager>,
    name: String,
    color: String,
) -> Result<Tag, String> {
    let db = Database;
    db.create_tag(&pools.get_active_data_pool().map_err(|e| e.to_string())?, &name, &color)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_tag(pools: State<'_, ShelfManager>, id: String) -> Result<(), String> {
    sqlx::query("DELETE FROM tags WHERE id = ?")
        .bind(&id)
        .execute(&pools.get_active_data_pool().map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

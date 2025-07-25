use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;


#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Directory {
    pub id: String,
    pub path: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct File {
    pub id: String,
    pub path: String,
    pub name: String,
    pub directory_id: String,
    pub size: i64,
    pub file_type: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    pub birth_time: Option<DateTime<Utc>>,
    pub inode: Option<i64>,
    pub is_directory: bool,
    pub created_at_db: DateTime<Utc>,
    pub updated_at_db: DateTime<Utc>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub permissions: Option<String>,
    pub owner_uid: Option<i64>,
    pub group_gid: Option<i64>,
    pub hard_links: Option<i64>,
    pub device_id: Option<i64>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub metadata: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CustomMetadataKey {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub data_type: String,
    pub description: Option<String>,
    pub is_required: bool,
    pub default_value: Option<String>,
    pub validation_pattern: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CustomMetadataValue {
    pub id: String,
    pub file_id: String,
    pub key_id: String,
    pub value: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
use mockall::predicate::*;

#[cfg_attr(test, mockall::automock)]
pub trait DatabaseTrait {
    async fn init_database(&self, data_pool: &SqlitePool, settings_pool: &SqlitePool) -> Result<(), sqlx::Error>;
    async fn add_directory(
        &self,
        pool: &SqlitePool,
        path: &str,
        name: &str,
    ) -> Result<Directory, sqlx::Error>;
    async fn get_directories(&self, pool: &SqlitePool) -> Result<Vec<Directory>, sqlx::Error>;
    async fn add_file(&self, pool: &SqlitePool, file: &File) -> Result<(), sqlx::Error>;
    async fn get_files_by_directory_sorted(
        &self,
        pool: &SqlitePool,
        directory_id: &str,
        sort_field: Option<String>,
        sort_order: Option<String>,
    ) -> Result<Vec<File>, sqlx::Error>;
    async fn get_all_files_sorted(
        &self,
        pool: &SqlitePool,
        sort_field: Option<String>,
        sort_order: Option<String>,
    ) -> Result<Vec<File>, sqlx::Error>;
    #[allow(clippy::too_many_arguments)]
    async fn get_all_files_paginated(
        &self,
        pool: &SqlitePool,
        sort_field: Option<String>,
        sort_order: Option<String>,
        limit: u32,
        offset: u32,
        show_hidden_files: bool,
        show_directories: bool,
    ) -> Result<Vec<File>, sqlx::Error>;
    #[allow(clippy::too_many_arguments)]
    async fn get_files_by_directory_paginated(
        &self,
        pool: &SqlitePool,
        directory_id: &str,
        sort_field: Option<String>,
        sort_order: Option<String>,
        limit: u32,
        offset: u32,
        show_hidden_files: bool,
        show_directories: bool,
    ) -> Result<Vec<File>, sqlx::Error>;
    async fn count_all_files(&self, pool: &SqlitePool, show_hidden_files: bool, show_directories: bool) -> Result<u32, sqlx::Error>;
    async fn count_files_by_directory(&self, pool: &SqlitePool, directory_id: &str, show_hidden_files: bool, show_directories: bool) -> Result<u32, sqlx::Error>;
    #[allow(clippy::too_many_arguments)]
    async fn get_files_paginated_with_category(
        &self,
        pool: &SqlitePool,
        category: &str,
        sort_field: Option<String>,
        sort_order: Option<String>,
        limit: u32,
        offset: u32,
        show_hidden_files: bool,
        show_directories: bool,
    ) -> Result<Vec<File>, sqlx::Error>;
    #[allow(clippy::too_many_arguments)]
    async fn get_files_by_directory_paginated_with_category(
        &self,
        pool: &SqlitePool,
        directory_id: &str,
        category: &str,
        sort_field: Option<String>,
        sort_order: Option<String>,
        limit: u32,
        offset: u32,
        show_hidden_files: bool,
        show_directories: bool,
    ) -> Result<Vec<File>, sqlx::Error>;
    async fn count_files_with_category(&self, pool: &SqlitePool, category: &str, show_hidden_files: bool, show_directories: bool) -> Result<u32, sqlx::Error>;
    async fn count_files_by_directory_with_category(&self, pool: &SqlitePool, directory_id: &str, category: &str, show_hidden_files: bool, show_directories: bool) -> Result<u32, sqlx::Error>;
    async fn get_all_tags(&self, pool: &SqlitePool) -> Result<Vec<Tag>, sqlx::Error>;
    async fn get_top_tags(&self, pool: &SqlitePool, limit: u32) -> Result<Vec<Tag>, sqlx::Error>;
    async fn search_tags_by_name(
        &self,
        pool: &SqlitePool,
        query: &str,
    ) -> Result<Vec<Tag>, sqlx::Error>;
    async fn get_tag_by_name(&self, pool: &SqlitePool, name: &str) -> Result<Tag, sqlx::Error>;
    async fn create_tag(
        &self,
        pool: &SqlitePool,
        name: &str,
        color: &str,
    ) -> Result<Tag, sqlx::Error>;
    async fn add_file_tag(
        &self,
        pool: &SqlitePool,
        file_id: &str,
        tag_id: &str,
    ) -> Result<(), sqlx::Error>;
    async fn remove_file_tag(
        &self,
        pool: &SqlitePool,
        file_id: &str,
        tag_id: &str,
    ) -> Result<(), sqlx::Error>;
    async fn delete_orphaned_tags(&self, pool: &SqlitePool) -> Result<Vec<String>, sqlx::Error>;
    async fn get_file_tags(
        &self,
        pool: &SqlitePool,
        file_id: &str,
    ) -> Result<Vec<Tag>, sqlx::Error>;
    // カスタムメタデータキー管理（設定用データベース）
    async fn create_custom_metadata_key(
        &self,
        settings_pool: &SqlitePool,
        key: &CustomMetadataKey,
    ) -> Result<CustomMetadataKey, sqlx::Error>;
    async fn get_all_custom_metadata_keys(
        &self,
        settings_pool: &SqlitePool,
    ) -> Result<Vec<CustomMetadataKey>, sqlx::Error>;
    async fn update_custom_metadata_key(
        &self,
        settings_pool: &SqlitePool,
        key: &CustomMetadataKey,
    ) -> Result<CustomMetadataKey, sqlx::Error>;
    async fn delete_custom_metadata_key(
        &self,
        settings_pool: &SqlitePool,
        data_pool: &SqlitePool,
        key_id: &str,
    ) -> Result<(), sqlx::Error>;
    async fn get_custom_metadata_key_by_name(
        &self,
        settings_pool: &SqlitePool,
        name: &str,
    ) -> Result<Option<CustomMetadataKey>, sqlx::Error>;
    // カスタムメタデータ値管理（データ用データベース、設定用データベースの参照が必要）
    async fn set_custom_metadata_value(
        &self,
        data_pool: &SqlitePool,
        settings_pool: &SqlitePool,
        file_id: &str,
        key_id: &str,
        value: Option<String>,
    ) -> Result<CustomMetadataValue, sqlx::Error>;
    async fn get_custom_metadata_values_by_file(
        &self,
        data_pool: &SqlitePool,
        file_id: &str,
    ) -> Result<Vec<CustomMetadataValue>, sqlx::Error>;
    async fn get_custom_metadata_value(
        &self,
        data_pool: &SqlitePool,
        file_id: &str,
        key_id: &str,
    ) -> Result<Option<CustomMetadataValue>, sqlx::Error>;
    async fn delete_custom_metadata_value(
        &self,
        data_pool: &SqlitePool,
        file_id: &str,
        key_id: &str,
    ) -> Result<(), sqlx::Error>;
    // ファイル監視に必要な関数
    async fn remove_file_by_path(&self, pool: &SqlitePool, path: &str) -> Result<(), sqlx::Error>;
    async fn update_file_metadata(
        &self,
        pool: &SqlitePool,
        path: &str,
        metadata: &std::fs::Metadata,
    ) -> Result<(), sqlx::Error>;
    async fn file_exists_by_path(&self, pool: &SqlitePool, path: &str)
        -> Result<bool, sqlx::Error>;
    async fn find_file_by_inode(
        &self,
        pool: &SqlitePool,
        inode: i64,
        device_id: Option<i64>,
    ) -> Result<Option<File>, sqlx::Error>;
    async fn update_file_path(
        &self,
        pool: &SqlitePool,
        file_id: &str,
        new_path: &str,
        new_name: &str,
    ) -> Result<(), sqlx::Error>;
}

pub struct Database;

impl DatabaseTrait for Database {
    async fn init_database(&self, _data_pool: &SqlitePool, _settings_pool: &SqlitePool) -> Result<(), sqlx::Error> {
        // マイグレーションは自動的に実行されるため、ここでは初期データの挿入のみ
        Ok(())
    }

    async fn add_directory(
        &self,
        pool: &SqlitePool,
        path: &str,
        name: &str,
    ) -> Result<Directory, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO directories (id, path, name, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(path)
        .bind(name)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;

        Ok(Directory {
            id,
            path: path.to_string(),
            name: name.to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    async fn get_directories(&self, pool: &SqlitePool) -> Result<Vec<Directory>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM directories ORDER BY name")
            .fetch_all(pool)
            .await?;

        let mut directories = Vec::new();
        for row in rows {
            directories.push(Directory {
                id: row.get("id"),
                path: row.get("path"),
                name: row.get("name"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(directories)
    }


    async fn add_file(&self, pool: &SqlitePool, file: &File) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT OR REPLACE INTO files (id, path, name, directory_id, size, file_type, created_at, modified_at, birth_time, inode, is_directory, created_at_db, updated_at_db, file_size, mime_type, permissions, owner_uid, group_gid, hard_links, device_id, last_accessed, metadata) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&file.id)
        .bind(&file.path)
        .bind(&file.name)
        .bind(&file.directory_id)
        .bind(file.size)
        .bind(&file.file_type)
        .bind(file.created_at)
        .bind(file.modified_at)
        .bind(file.birth_time)
        .bind(file.inode)
        .bind(file.is_directory)
        .bind(file.created_at_db)
        .bind(file.updated_at_db)
        .bind(file.file_size)
        .bind(&file.mime_type)
        .bind(&file.permissions)
        .bind(file.owner_uid)
        .bind(file.group_gid)
        .bind(file.hard_links)
        .bind(file.device_id)
        .bind(file.last_accessed)
        .bind(&file.metadata)
        .execute(pool)
        .await?;

        Ok(())
    }



    async fn get_all_files_sorted(
        &self,
        pool: &SqlitePool,
        sort_field: Option<String>,
        sort_order: Option<String>,
    ) -> Result<Vec<File>, sqlx::Error> {
        let sort_field = sort_field.as_deref().unwrap_or("modified_at");
        let sort_order = sort_order.as_deref().unwrap_or("desc");

        let sort_column = match sort_field {
            "name" => "name",
            "size" => "size",
            "created_at" => "created_at",
            "modified_at" => "modified_at",
            "last_accessed" => "last_accessed",
            "file_type" => "file_type",
            _ => "modified_at",
        };

        let order_direction = match sort_order {
            "asc" => "ASC",
            _ => "DESC",
        };

        let query = format!(
            "SELECT * FROM files ORDER BY {sort_column} {order_direction} NULLS LAST"
        );

        let rows = sqlx::query(&query).fetch_all(pool).await?;

        let mut files = Vec::new();
        for row in rows {
            files.push(File {
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
            });
        }

        Ok(files)
    }

    async fn get_files_by_directory_sorted(
        &self,
        pool: &SqlitePool,
        directory_id: &str,
        sort_field: Option<String>,
        sort_order: Option<String>,
    ) -> Result<Vec<File>, sqlx::Error> {
        let sort_field = sort_field.as_deref().unwrap_or("modified_at");
        let sort_order = sort_order.as_deref().unwrap_or("desc");

        let sort_column = match sort_field {
            "name" => "name",
            "size" => "size",
            "created_at" => "created_at",
            "modified_at" => "modified_at",
            "last_accessed" => "last_accessed",
            "file_type" => "file_type",
            _ => "modified_at",
        };

        let order_direction = match sort_order {
            "asc" => "ASC",
            _ => "DESC",
        };

        let query = format!(
            "SELECT * FROM files WHERE directory_id = ? ORDER BY {sort_column} {order_direction} NULLS LAST"
        );

        let rows = sqlx::query(&query)
            .bind(directory_id)
            .fetch_all(pool)
            .await?;

        let mut files = Vec::new();
        for row in rows {
            files.push(File {
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
            });
        }

        Ok(files)
    }

    async fn get_all_files_paginated(
        &self,
        pool: &SqlitePool,
        sort_field: Option<String>,
        sort_order: Option<String>,
        limit: u32,
        offset: u32,
        show_hidden_files: bool,
        show_directories: bool,
    ) -> Result<Vec<File>, sqlx::Error> {
        let sort_field = sort_field.as_deref().unwrap_or("modified_at");
        let sort_order = sort_order.as_deref().unwrap_or("desc");

        let sort_column = match sort_field {
            "name" => "name",
            "size" => "size",
            "created_at" => "created_at",
            "modified_at" => "modified_at",
            "last_accessed" => "last_accessed",
            "file_type" => "file_type",
            _ => "modified_at",
        };

        let order_direction = match sort_order {
            "asc" => "ASC",
            _ => "DESC",
        };

        let mut where_conditions = Vec::new();
        
        if !show_hidden_files {
            where_conditions.push("name NOT LIKE '.%'");
        }
        
        if !show_directories {
            where_conditions.push("is_directory = FALSE");
        }

        let where_clause = if where_conditions.is_empty() {
            "".to_string()
        } else {
            format!("WHERE {}", where_conditions.join(" AND "))
        };

        let query = format!(
            "SELECT * FROM files {where_clause} ORDER BY {sort_column} {order_direction} NULLS LAST LIMIT ? OFFSET ?"
        );

        let rows = sqlx::query(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        let mut files = Vec::new();
        for row in rows {
            files.push(File {
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
            });
        }

        Ok(files)
    }

    async fn get_files_by_directory_paginated(
        &self,
        pool: &SqlitePool,
        directory_id: &str,
        sort_field: Option<String>,
        sort_order: Option<String>,
        limit: u32,
        offset: u32,
        show_hidden_files: bool,
        show_directories: bool,
    ) -> Result<Vec<File>, sqlx::Error> {
        let sort_field = sort_field.as_deref().unwrap_or("modified_at");
        let sort_order = sort_order.as_deref().unwrap_or("desc");

        let sort_column = match sort_field {
            "name" => "name",
            "size" => "size",
            "created_at" => "created_at",
            "modified_at" => "modified_at",
            "last_accessed" => "last_accessed",
            "file_type" => "file_type",
            _ => "modified_at",
        };

        let order_direction = match sort_order {
            "asc" => "ASC",
            _ => "DESC",
        };

        let mut where_conditions = vec!["directory_id = ?"];
        
        if !show_hidden_files {
            where_conditions.push("name NOT LIKE '.%'");
        }
        
        if !show_directories {
            where_conditions.push("is_directory = FALSE");
        }

        let query = format!(
            "SELECT * FROM files WHERE {} ORDER BY {} {} NULLS LAST LIMIT ? OFFSET ?",
            where_conditions.join(" AND "), sort_column, order_direction
        );

        let rows = sqlx::query(&query)
            .bind(directory_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        let mut files = Vec::new();
        for row in rows {
            files.push(File {
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
            });
        }

        Ok(files)
    }

    async fn count_all_files(&self, pool: &SqlitePool, show_hidden_files: bool, show_directories: bool) -> Result<u32, sqlx::Error> {
        let mut where_conditions = Vec::new();
        
        if !show_hidden_files {
            where_conditions.push("name NOT LIKE '.%'");
        }
        
        if !show_directories {
            where_conditions.push("is_directory = FALSE");
        }

        let where_clause = if where_conditions.is_empty() {
            "".to_string()
        } else {
            format!("WHERE {}", where_conditions.join(" AND "))
        };

        let query = format!("SELECT COUNT(*) FROM files {where_clause}");
        
        let count: i64 = sqlx::query_scalar(&query)
            .fetch_one(pool)
            .await?;
        Ok(count as u32)
    }

    async fn count_files_by_directory(&self, pool: &SqlitePool, directory_id: &str, show_hidden_files: bool, show_directories: bool) -> Result<u32, sqlx::Error> {
        let mut where_conditions = vec!["directory_id = ?"];
        
        if !show_hidden_files {
            where_conditions.push("name NOT LIKE '.%'");
        }
        
        if !show_directories {
            where_conditions.push("is_directory = FALSE");
        }

        let query = format!("SELECT COUNT(*) FROM files WHERE {}", where_conditions.join(" AND "));
        
        let count: i64 = sqlx::query_scalar(&query)
            .bind(directory_id)
            .fetch_one(pool)
            .await?;
        Ok(count as u32)
    }

    async fn get_all_tags(&self, pool: &SqlitePool) -> Result<Vec<Tag>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM tags ORDER BY name")
            .fetch_all(pool)
            .await?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(Tag {
                id: row.get("id"),
                name: row.get("name"),
                color: row.get("color"),
                created_at: row.get("created_at"),
            });
        }

        Ok(tags)
    }

    async fn get_top_tags(&self, pool: &SqlitePool, limit: u32) -> Result<Vec<Tag>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT t.id, t.name, t.color, t.created_at, COUNT(ft.tag_id) as file_count
             FROM tags t
             LEFT JOIN file_tags ft ON t.id = ft.tag_id
             GROUP BY t.id, t.name, t.color, t.created_at
             ORDER BY file_count DESC, t.name ASC
             LIMIT ?",
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(Tag {
                id: row.get("id"),
                name: row.get("name"),
                color: row.get("color"),
                created_at: row.get("created_at"),
            });
        }

        Ok(tags)
    }

    async fn search_tags_by_name(
        &self,
        pool: &SqlitePool,
        query: &str,
    ) -> Result<Vec<Tag>, sqlx::Error> {
        let search_pattern = format!("%{query}%");
        let rows = sqlx::query(
            "SELECT * FROM tags 
             WHERE name LIKE ? 
             ORDER BY name ASC",
        )
        .bind(&search_pattern)
        .fetch_all(pool)
        .await?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(Tag {
                id: row.get("id"),
                name: row.get("name"),
                color: row.get("color"),
                created_at: row.get("created_at"),
            });
        }

        Ok(tags)
    }

    async fn get_tag_by_name(&self, pool: &SqlitePool, name: &str) -> Result<Tag, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM tags WHERE name = ? LIMIT 1")
            .bind(name)
            .fetch_one(pool)
            .await?;

        Ok(Tag {
            id: row.get("id"),
            name: row.get("name"),
            color: row.get("color"),
            created_at: row.get("created_at"),
        })
    }

    async fn create_tag(
        &self,
        pool: &SqlitePool,
        name: &str,
        color: &str,
    ) -> Result<Tag, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query("INSERT INTO tags (id, name, color, created_at) VALUES (?, ?, ?, ?)")
            .bind(&id)
            .bind(name)
            .bind(color)
            .bind(now)
            .execute(pool)
            .await?;

        Ok(Tag {
            id,
            name: name.to_string(),
            color: color.to_string(),
            created_at: now,
        })
    }

    async fn add_file_tag(
        &self,
        pool: &SqlitePool,
        file_id: &str,
        tag_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT OR IGNORE INTO file_tags (file_id, tag_id) VALUES (?, ?)")
            .bind(file_id)
            .bind(tag_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    async fn remove_file_tag(
        &self,
        pool: &SqlitePool,
        file_id: &str,
        tag_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM file_tags WHERE file_id = ? AND tag_id = ?")
            .bind(file_id)
            .bind(tag_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    async fn delete_orphaned_tags(&self, pool: &SqlitePool) -> Result<Vec<String>, sqlx::Error> {
        // 参照されていないタグのIDを取得（ファイルタグのみをチェック）
        let orphaned_tag_ids: Vec<String> = sqlx::query_scalar(
            "SELECT t.id FROM tags t 
             LEFT JOIN file_tags ft ON t.id = ft.tag_id 
             WHERE ft.tag_id IS NULL",
        )
        .fetch_all(pool)
        .await?;

        // 参照されていないタグを削除
        if !orphaned_tag_ids.is_empty() {
            for tag_id in &orphaned_tag_ids {
                sqlx::query("DELETE FROM tags WHERE id = ?")
                    .bind(tag_id)
                    .execute(pool)
                    .await?;
            }
        }

        Ok(orphaned_tag_ids)
    }

    async fn get_file_tags(
        &self,
        pool: &SqlitePool,
        file_id: &str,
    ) -> Result<Vec<Tag>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT t.* FROM tags t 
             INNER JOIN file_tags ft ON t.id = ft.tag_id 
             WHERE ft.file_id = ? 
             ORDER BY t.name",
        )
        .bind(file_id)
        .fetch_all(pool)
        .await?;

        let mut tags = Vec::new();
        for row in rows {
            tags.push(Tag {
                id: row.get("id"),
                name: row.get("name"),
                color: row.get("color"),
                created_at: row.get("created_at"),
            });
        }

        Ok(tags)
    }

    async fn create_custom_metadata_key(
        &self,
        settings_pool: &SqlitePool,
        key: &CustomMetadataKey,
    ) -> Result<CustomMetadataKey, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO custom_metadata_keys (id, name, display_name, data_type, description, is_required, default_value, validation_pattern, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&key.name)
        .bind(&key.display_name)
        .bind(&key.data_type)
        .bind(&key.description)
        .bind(key.is_required)
        .bind(&key.default_value)
        .bind(&key.validation_pattern)
        .bind(now)
        .bind(now)
        .execute(settings_pool)
        .await?;

        Ok(CustomMetadataKey {
            id,
            name: key.name.clone(),
            display_name: key.display_name.clone(),
            data_type: key.data_type.clone(),
            description: key.description.clone(),
            is_required: key.is_required,
            default_value: key.default_value.clone(),
            validation_pattern: key.validation_pattern.clone(),
            created_at: now,
            updated_at: now,
        })
    }

    async fn get_all_custom_metadata_keys(
        &self,
        settings_pool: &SqlitePool,
    ) -> Result<Vec<CustomMetadataKey>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM custom_metadata_keys ORDER BY name")
            .fetch_all(settings_pool)
            .await?;

        let mut keys = Vec::new();
        for row in rows {
            keys.push(CustomMetadataKey {
                id: row.get("id"),
                name: row.get("name"),
                display_name: row.get("display_name"),
                data_type: row.get("data_type"),
                description: row.get("description"),
                is_required: row.get("is_required"),
                default_value: row.get("default_value"),
                validation_pattern: row.get("validation_pattern"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(keys)
    }

    async fn update_custom_metadata_key(
        &self,
        settings_pool: &SqlitePool,
        key: &CustomMetadataKey,
    ) -> Result<CustomMetadataKey, sqlx::Error> {
        let now = Utc::now();

        sqlx::query(
            "UPDATE custom_metadata_keys SET display_name = ?, data_type = ?, description = ?, is_required = ?, default_value = ?, validation_pattern = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&key.display_name)
        .bind(&key.data_type)
        .bind(&key.description)
        .bind(key.is_required)
        .bind(&key.default_value)
        .bind(&key.validation_pattern)
        .bind(now)
        .bind(&key.id)
        .execute(settings_pool)
        .await?;

        Ok(CustomMetadataKey {
            id: key.id.clone(),
            name: key.name.clone(),
            display_name: key.display_name.clone(),
            data_type: key.data_type.clone(),
            description: key.description.clone(),
            is_required: key.is_required,
            default_value: key.default_value.clone(),
            validation_pattern: key.validation_pattern.clone(),
            created_at: key.created_at,
            updated_at: now,
        })
    }

    async fn delete_custom_metadata_key(
        &self,
        settings_pool: &SqlitePool,
        data_pool: &SqlitePool,
        key_id: &str,
    ) -> Result<(), sqlx::Error> {
        // まず、このキーを参照しているカスタムメタデータ値を削除
        sqlx::query("DELETE FROM custom_metadata_values WHERE key_id = ?")
            .bind(key_id)
            .execute(data_pool)
            .await?;

        // その後、キー定義を削除
        sqlx::query("DELETE FROM custom_metadata_keys WHERE id = ?")
            .bind(key_id)
            .execute(settings_pool)
            .await?;

        Ok(())
    }

    async fn get_custom_metadata_key_by_name(
        &self,
        settings_pool: &SqlitePool,
        name: &str,
    ) -> Result<Option<CustomMetadataKey>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM custom_metadata_keys WHERE name = ?")
            .bind(name)
            .fetch_optional(settings_pool)
            .await?;

        match row {
            Some(row) => Ok(Some(CustomMetadataKey {
                id: row.get("id"),
                name: row.get("name"),
                display_name: row.get("display_name"),
                data_type: row.get("data_type"),
                description: row.get("description"),
                is_required: row.get("is_required"),
                default_value: row.get("default_value"),
                validation_pattern: row.get("validation_pattern"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })),
            None => Ok(None),
        }
    }

    async fn set_custom_metadata_value(
        &self,
        data_pool: &SqlitePool,
        settings_pool: &SqlitePool,
        file_id: &str,
        key_id: &str,
        value: Option<String>,
    ) -> Result<CustomMetadataValue, sqlx::Error> {
        // キーが存在することを確認
        let key_exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM custom_metadata_keys WHERE id = ?")
            .bind(key_id)
            .fetch_one(settings_pool)
            .await?;
        
        if key_exists == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        let now = Utc::now();

        // 既存のレコードをチェック
        if let Ok(existing) = sqlx::query_as::<_, (String, DateTime<Utc>)>(
            "SELECT id, created_at FROM custom_metadata_values WHERE file_id = ? AND key_id = ?"
        )
        .bind(file_id)
        .bind(key_id)
        .fetch_one(data_pool)
        .await
        {
            // 既存レコードがある場合は更新
            sqlx::query(
                "UPDATE custom_metadata_values SET value = ?, updated_at = ? WHERE file_id = ? AND key_id = ?"
            )
            .bind(value.as_deref())
            .bind(now)
            .bind(file_id)
            .bind(key_id)
            .execute(data_pool)
            .await?;

            Ok(CustomMetadataValue {
                id: existing.0,
                file_id: file_id.to_string(),
                key_id: key_id.to_string(),
                value,
                created_at: existing.1,
                updated_at: now,
            })
        } else {
            // 既存レコードがない場合は新規作成
            let id = Uuid::new_v4().to_string();
            
            sqlx::query(
                "INSERT INTO custom_metadata_values (id, file_id, key_id, value, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(&id)
            .bind(file_id)
            .bind(key_id)
            .bind(value.as_deref())
            .bind(now)
            .bind(now)
            .execute(data_pool)
            .await?;

            Ok(CustomMetadataValue {
                id,
                file_id: file_id.to_string(),
                key_id: key_id.to_string(),
                value,
                created_at: now,
                updated_at: now,
            })
        }
    }

    async fn get_custom_metadata_values_by_file(
        &self,
        data_pool: &SqlitePool,
        file_id: &str,
    ) -> Result<Vec<CustomMetadataValue>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM custom_metadata_values WHERE file_id = ?")
            .bind(file_id)
            .fetch_all(data_pool)
            .await?;

        let mut values = Vec::new();
        for row in rows {
            values.push(CustomMetadataValue {
                id: row.get("id"),
                file_id: row.get("file_id"),
                key_id: row.get("key_id"),
                value: row.get("value"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(values)
    }

    async fn get_custom_metadata_value(
        &self,
        data_pool: &SqlitePool,
        file_id: &str,
        key_id: &str,
    ) -> Result<Option<CustomMetadataValue>, sqlx::Error> {
        let row =
            sqlx::query("SELECT * FROM custom_metadata_values WHERE file_id = ? AND key_id = ?")
                .bind(file_id)
                .bind(key_id)
                .fetch_optional(data_pool)
                .await?;

        match row {
            Some(row) => Ok(Some(CustomMetadataValue {
                id: row.get("id"),
                file_id: row.get("file_id"),
                key_id: row.get("key_id"),
                value: row.get("value"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })),
            None => Ok(None),
        }
    }

    async fn delete_custom_metadata_value(
        &self,
        data_pool: &SqlitePool,
        file_id: &str,
        key_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM custom_metadata_values WHERE file_id = ? AND key_id = ?")
            .bind(file_id)
            .bind(key_id)
            .execute(data_pool)
            .await?;

        Ok(())
    }

    async fn remove_file_by_path(&self, pool: &SqlitePool, path: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM files WHERE path = ?")
            .bind(path)
            .execute(pool)
            .await?;

        Ok(())
    }

    async fn update_file_metadata(
        &self,
        pool: &SqlitePool,
        path: &str,
        metadata: &std::fs::Metadata,
    ) -> Result<(), sqlx::Error> {
        use chrono::{DateTime, Utc};
        use std::os::unix::fs::MetadataExt;

        let now = Utc::now();
        let size = metadata.len() as i64;
        let modified_at = metadata.modified().ok().and_then(|st| {
            DateTime::from_timestamp(
                st.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64,
                0,
            )
        });
        let created_at = metadata.created().ok().and_then(|st| {
            DateTime::from_timestamp(
                st.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64,
                0,
            )
        });
        let last_accessed = metadata.accessed().ok().and_then(|st| {
            DateTime::from_timestamp(
                st.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64,
                0,
            )
        });

        let inode = metadata.ino() as i64;
        let owner_uid = metadata.uid() as i64;
        let group_gid = metadata.gid() as i64;
        let hard_links = metadata.nlink() as i64;
        let device_id = metadata.dev() as i64;
        let permissions = format!("{:o}", metadata.mode() & 0o777);

        sqlx::query(
            "UPDATE files SET 
                size = ?, 
                modified_at = ?, 
                created_at = ?, 
                last_accessed = ?, 
                inode = ?, 
                owner_uid = ?, 
                group_gid = ?, 
                hard_links = ?, 
                device_id = ?, 
                permissions = ?, 
                updated_at_db = ?
            WHERE path = ?",
        )
        .bind(size)
        .bind(modified_at)
        .bind(created_at)
        .bind(last_accessed)
        .bind(inode)
        .bind(owner_uid)
        .bind(group_gid)
        .bind(hard_links)
        .bind(device_id)
        .bind(permissions)
        .bind(now)
        .bind(path)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn file_exists_by_path(
        &self,
        pool: &SqlitePool,
        path: &str,
    ) -> Result<bool, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM files WHERE path = ?")
            .bind(path)
            .fetch_one(pool)
            .await?;

        Ok(count > 0)
    }

    async fn find_file_by_inode(
        &self,
        pool: &SqlitePool,
        inode: i64,
        device_id: Option<i64>,
    ) -> Result<Option<File>, sqlx::Error> {
        let query = if let Some(_device_id) = device_id {
            "SELECT * FROM files WHERE inode = ? AND device_id = ?"
        } else {
            "SELECT * FROM files WHERE inode = ? AND device_id IS NULL"
        };

        let row = if let Some(device_id) = device_id {
            sqlx::query(query)
                .bind(inode)
                .bind(device_id)
                .fetch_optional(pool)
                .await?
        } else {
            sqlx::query("SELECT * FROM files WHERE inode = ?")
                .bind(inode)
                .fetch_optional(pool)
                .await?
        };

        match row {
            Some(row) => Ok(Some(File {
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
            })),
            None => Ok(None),
        }
    }

    async fn update_file_path(
        &self,
        pool: &SqlitePool,
        file_id: &str,
        new_path: &str,
        new_name: &str,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now();

        sqlx::query("UPDATE files SET path = ?, name = ?, updated_at_db = ? WHERE id = ?")
            .bind(new_path)
            .bind(new_name)
            .bind(now)
            .bind(file_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    async fn get_files_paginated_with_category(
        &self,
        pool: &SqlitePool,
        category: &str,
        sort_field: Option<String>,
        sort_order: Option<String>,
        limit: u32,
        offset: u32,
        show_hidden_files: bool,
        show_directories: bool,
    ) -> Result<Vec<File>, sqlx::Error> {
        let sort_field = sort_field.as_deref().unwrap_or("modified_at");
        let sort_order = sort_order.as_deref().unwrap_or("desc");

        let sort_column = match sort_field {
            "name" => "name",
            "size" => "size",
            "created_at" => "created_at",
            "modified_at" => "modified_at",
            "last_accessed" => "last_accessed",
            "file_type" => "file_type",
            _ => "modified_at",
        };

        let order_direction = match sort_order {
            "asc" => "ASC",
            _ => "DESC",
        };

        let category_where_clause = build_category_where_clause(category);
        
        let mut additional_conditions = Vec::new();
        if !show_hidden_files {
            additional_conditions.push(" AND name NOT LIKE '.%'");
        }
        if !show_directories {
            additional_conditions.push(" AND is_directory = FALSE");
        }

        let query = format!(
            "SELECT * FROM files WHERE 1=1{}{} ORDER BY {} {} NULLS LAST LIMIT ? OFFSET ?",
            category_where_clause, additional_conditions.join(""), sort_column, order_direction
        );

        let rows = sqlx::query(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        let mut files = Vec::new();
        for row in rows {
            files.push(File {
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
            });
        }

        Ok(files)
    }

    async fn get_files_by_directory_paginated_with_category(
        &self,
        pool: &SqlitePool,
        directory_id: &str,
        category: &str,
        sort_field: Option<String>,
        sort_order: Option<String>,
        limit: u32,
        offset: u32,
        show_hidden_files: bool,
        show_directories: bool,
    ) -> Result<Vec<File>, sqlx::Error> {
        let sort_field = sort_field.as_deref().unwrap_or("modified_at");
        let sort_order = sort_order.as_deref().unwrap_or("desc");

        let sort_column = match sort_field {
            "name" => "name",
            "size" => "size",
            "created_at" => "created_at",
            "modified_at" => "modified_at",
            "last_accessed" => "last_accessed",
            "file_type" => "file_type",
            _ => "modified_at",
        };

        let order_direction = match sort_order {
            "asc" => "ASC",
            _ => "DESC",
        };

        let category_where_clause = build_category_where_clause(category);
        
        let mut additional_conditions = Vec::new();
        if !show_hidden_files {
            additional_conditions.push(" AND name NOT LIKE '.%'");
        }
        if !show_directories {
            additional_conditions.push(" AND is_directory = FALSE");
        }

        let query = format!(
            "SELECT * FROM files WHERE directory_id = ?{}{} ORDER BY {} {} NULLS LAST LIMIT ? OFFSET ?",
            category_where_clause, additional_conditions.join(""), sort_column, order_direction
        );

        let rows = sqlx::query(&query)
            .bind(directory_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        let mut files = Vec::new();
        for row in rows {
            files.push(File {
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
            });
        }

        Ok(files)
    }

    async fn count_files_with_category(&self, pool: &SqlitePool, category: &str, show_hidden_files: bool, show_directories: bool) -> Result<u32, sqlx::Error> {
        let category_where_clause = build_category_where_clause(category);
        
        let mut additional_conditions = Vec::new();
        if !show_hidden_files {
            additional_conditions.push(" AND name NOT LIKE '.%'");
        }
        if !show_directories {
            additional_conditions.push(" AND is_directory = FALSE");
        }

        let query = format!("SELECT COUNT(*) FROM files WHERE 1=1{}{}", category_where_clause, additional_conditions.join(""));
        
        let count: i64 = sqlx::query_scalar(&query)
            .fetch_one(pool)
            .await?;
        Ok(count as u32)
    }

    async fn count_files_by_directory_with_category(&self, pool: &SqlitePool, directory_id: &str, category: &str, show_hidden_files: bool, show_directories: bool) -> Result<u32, sqlx::Error> {
        let category_where_clause = build_category_where_clause(category);
        
        let mut additional_conditions = Vec::new();
        if !show_hidden_files {
            additional_conditions.push(" AND name NOT LIKE '.%'");
        }
        if !show_directories {
            additional_conditions.push(" AND is_directory = FALSE");
        }

        let query = format!("SELECT COUNT(*) FROM files WHERE directory_id = ?{}{}", category_where_clause, additional_conditions.join(""));
        
        let count: i64 = sqlx::query_scalar(&query)
            .bind(directory_id)
            .fetch_one(pool)
            .await?;
        Ok(count as u32)
    }

}

fn build_category_where_clause(category: &str) -> String {
    if category == "all" {
        return String::new();
    }
    
    let extensions = match category {
        "image" => vec!["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg", "ico", "tiff", "raw"],
        "audio" => vec!["mp3", "wav", "ogg", "flac", "aac", "m4a", "wma", "opus"],
        "video" => vec!["mp4", "avi", "mov", "wmv", "flv", "webm", "mkv", "m4v", "3gp"],
        "document" => vec!["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "md", "html", "htm", "css", "js", "json", "xml", "csv", "rtf"],
        "archive" => vec!["zip", "rar", "7z", "tar", "gz", "bz2", "xz", "lzma"],
        "other" => return " AND (path NOT LIKE '%.jpg' AND path NOT LIKE '%.jpeg' AND path NOT LIKE '%.png' AND path NOT LIKE '%.gif' AND path NOT LIKE '%.bmp' AND path NOT LIKE '%.webp' AND path NOT LIKE '%.svg' AND path NOT LIKE '%.ico' AND path NOT LIKE '%.tiff' AND path NOT LIKE '%.raw' AND path NOT LIKE '%.mp3' AND path NOT LIKE '%.wav' AND path NOT LIKE '%.ogg' AND path NOT LIKE '%.flac' AND path NOT LIKE '%.aac' AND path NOT LIKE '%.m4a' AND path NOT LIKE '%.wma' AND path NOT LIKE '%.opus' AND path NOT LIKE '%.mp4' AND path NOT LIKE '%.avi' AND path NOT LIKE '%.mov' AND path NOT LIKE '%.wmv' AND path NOT LIKE '%.flv' AND path NOT LIKE '%.webm' AND path NOT LIKE '%.mkv' AND path NOT LIKE '%.m4v' AND path NOT LIKE '%.3gp' AND path NOT LIKE '%.pdf' AND path NOT LIKE '%.doc' AND path NOT LIKE '%.docx' AND path NOT LIKE '%.xls' AND path NOT LIKE '%.xlsx' AND path NOT LIKE '%.ppt' AND path NOT LIKE '%.pptx' AND path NOT LIKE '%.txt' AND path NOT LIKE '%.md' AND path NOT LIKE '%.html' AND path NOT LIKE '%.htm' AND path NOT LIKE '%.css' AND path NOT LIKE '%.js' AND path NOT LIKE '%.json' AND path NOT LIKE '%.xml' AND path NOT LIKE '%.csv' AND path NOT LIKE '%.rtf' AND path NOT LIKE '%.zip' AND path NOT LIKE '%.rar' AND path NOT LIKE '%.7z' AND path NOT LIKE '%.tar' AND path NOT LIKE '%.gz' AND path NOT LIKE '%.bz2' AND path NOT LIKE '%.xz' AND path NOT LIKE '%.lzma')".to_string(),
        _ => return String::new(),
    };
    
    let conditions: Vec<String> = extensions.iter()
        .map(|ext| format!("path LIKE '%.{}' OR path LIKE '%.{}'", ext, ext.to_uppercase()))
        .collect();
    
    format!(" AND ({})", conditions.join(" OR "))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use chrono::Utc;
    use sqlx::SqlitePool;
    use std::path::PathBuf;
    use std::mem;
    use uuid::Uuid;

    /// テスト用の一時DBファイルを管理する構造体
    /// Dropが呼ばれた際に自動的にDBファイルを削除する
    pub struct TestDatabase {
        pub pool: SqlitePool,
        pub file_path: Option<PathBuf>,
    }

    impl TestDatabase {
        /// Poolを取得してTestDatabaseを消費する
        /// ファイルパスを無効化してからPoolを返す
        pub fn into_pool(mut self) -> SqlitePool {
            // ファイルパスを取得してDrop時の削除対象から外す
            let file_path = self.file_path.take();
            
            // 手動でファイル削除（必要な場合）
            if let Some(path) = file_path {
                if path.exists() {
                    let _ = std::fs::remove_file(path);
                }
            }
            
            // Dropを回避してPoolを安全に取り出す
            let pool = unsafe {
                std::ptr::read(&self.pool)
            };
            mem::forget(self); // Dropを防ぐ
            
            pool
        }

        
        /// インメモリDBを作成（削除処理なし）
        pub async fn new_in_memory() -> Self {
            let pool = SqlitePool::connect(":memory:").await.unwrap();
            
            // マイグレーションを適用
            sqlx::migrate!("./data_migrations")
                .run(&pool)
                .await
                .unwrap();

            Self {
                pool,
                file_path: None,
            }
        }

        /// 一時ファイルDBを作成（テスト終了時に自動削除）
        pub async fn new_temp_file() -> Self {
            let temp_dir = std::env::temp_dir();
            let file_name = format!("test_clerica_{}.db", Uuid::new_v4());
            let file_path = temp_dir.join(file_name);
            
            // ディレクトリが存在することを確認
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            
            let connection_string = format!("sqlite:{}?mode=rwc", file_path.display());
            let pool = SqlitePool::connect(&connection_string).await.unwrap();
            
            // マイグレーションを適用
            sqlx::migrate!("./data_migrations")
                .run(&pool)
                .await
                .unwrap();

            Self {
                pool,
                file_path: Some(file_path),
            }
        }

        /// エラーハンドリングテスト用の空DB（テーブル未作成）
        pub async fn new_empty() -> Self {
            Self {
                pool: SqlitePool::connect(":memory:").await.unwrap(),
                file_path: None,
            }
        }
    }

    impl Drop for TestDatabase {
        fn drop(&mut self) {
            if let Some(file_path) = &self.file_path {
                if file_path.exists() {
                    let _ = std::fs::remove_file(file_path);
                }
            }
        }
    }

    async fn setup_test_db() -> SqlitePool {
        let test_db = TestDatabase::new_in_memory().await;
        test_db.into_pool()
    }


    #[tokio::test]
    async fn test_add_directory() {
        let pool = setup_test_db().await;
        let db = Database;

        let result = db.add_directory(&pool, "/test/path", "test_dir").await;
        assert!(result.is_ok());

        let directory = result.unwrap();
        assert_eq!(directory.path, "/test/path");
        assert_eq!(directory.name, "test_dir");
    }

    #[tokio::test]
    async fn test_add_directory_with_temp_file() {
        // 一時ファイルDBを使用したテスト（自動削除確認）
        let test_db = TestDatabase::new_temp_file().await;
        let db = Database;

        let result = db.add_directory(&test_db.pool, "/test/path", "test_dir").await;
        assert!(result.is_ok());

        let directory = result.unwrap();
        assert_eq!(directory.path, "/test/path");
        assert_eq!(directory.name, "test_dir");

        // test_db がDropされる際に一時ファイルが自動削除される
    }

    #[tokio::test]
    async fn test_get_directories() {
        let pool = setup_test_db().await;
        let db = Database;

        db.add_directory(&pool, "/test/path1", "dir1")
            .await
            .unwrap();
        db.add_directory(&pool, "/test/path2", "dir2")
            .await
            .unwrap();

        let directories = db.get_directories(&pool).await.unwrap();
        assert_eq!(directories.len(), 2);
        assert_eq!(directories[0].name, "dir1"); // ORDER BY name
        assert_eq!(directories[1].name, "dir2");
    }

    #[tokio::test]
    async fn test_create_tag() {
        let pool = setup_test_db().await;
        let db = Database;

        let result = db.create_tag(&pool, "important", "#ff0000").await;
        assert!(result.is_ok());

        let tag = result.unwrap();
        assert_eq!(tag.name, "important");
        assert_eq!(tag.color, "#ff0000");
    }

    #[tokio::test]
    async fn test_file_tag_operations() {
        let pool = setup_test_db().await;
        let db = Database;

        // ディレクトリとタグを作成
        let dir = db.add_directory(&pool, "/test", "test").await.unwrap();
        let tag = db.create_tag(&pool, "test_tag", "#ff0000").await.unwrap();

        // ファイルを作成
        let file = File {
            id: "test_file_id".to_string(),
            path: "/test/file.txt".to_string(),
            name: "file.txt".to_string(),
            directory_id: dir.id.clone(),
            size: 1024,
            file_type: Some("txt".to_string()),
            created_at: Some(Utc::now()),
            modified_at: Some(Utc::now()),
            birth_time: Some(Utc::now()),
            inode: Some(12345),
            is_directory: false,
            created_at_db: Utc::now(),
            updated_at_db: Utc::now(),
            file_size: Some(1024),
            mime_type: Some("text/plain".to_string()),
            permissions: Some("644".to_string()),
            owner_uid: Some(1000),
            group_gid: Some(1000),
            hard_links: Some(1),
            device_id: Some(12345),
            last_accessed: None,
            metadata: None,
        };

        db.add_file(&pool, &file).await.unwrap();

        // ファイルにタグを追加
        db.add_file_tag(&pool, &file.id, &tag.id).await.unwrap();

        // ファイルのタグを取得
        let file_tags = db.get_file_tags(&pool, &file.id).await.unwrap();
        assert_eq!(file_tags.len(), 1);
        assert_eq!(file_tags[0].name, "test_tag");

        // ファイルからタグを削除
        db.remove_file_tag(&pool, &file.id, &tag.id).await.unwrap();

        let file_tags = db.get_file_tags(&pool, &file.id).await.unwrap();
        assert_eq!(file_tags.len(), 0);
    }

    #[tokio::test]
    async fn test_get_all_tags() {
        let pool = setup_test_db().await;
        let db = Database;

        db.create_tag(&pool, "tag1", "#ff0000").await.unwrap();
        db.create_tag(&pool, "tag2", "#00ff00").await.unwrap();

        let tags = db.get_all_tags(&pool).await.unwrap();
        assert_eq!(tags.len(), 2);
    }

    #[tokio::test]
    async fn test_init_database() {
        let pool = setup_test_db().await;
        let db = Database;

        let result = db.init_database(&pool, &pool).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_file_with_various_types() {
        let pool = setup_test_db().await;
        let db = Database;

        let dir = db.add_directory(&pool, "/test", "test").await.unwrap();

        // ディレクトリファイル
        let dir_file = File {
            id: "dir_file".to_string(),
            path: "/test/subdir".to_string(),
            name: "subdir".to_string(),
            directory_id: dir.id.clone(),
            size: 0,
            file_type: None,
            created_at: None,
            modified_at: None,
            birth_time: None,
            inode: None,
            is_directory: true,
            created_at_db: Utc::now(),
            updated_at_db: Utc::now(),
            file_size: Some(1024),
            mime_type: Some("text/plain".to_string()),
            permissions: Some("644".to_string()),
            owner_uid: Some(1000),
            group_gid: Some(1000),
            hard_links: Some(1),
            device_id: Some(12345),
            last_accessed: None,
            metadata: None,
        };

        let result = db.add_file(&pool, &dir_file).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_file_tag_duplicate() {
        let pool = setup_test_db().await;
        let db = Database;

        let dir = db.add_directory(&pool, "/test", "test").await.unwrap();
        let tag = db.create_tag(&pool, "test_tag", "#ff0000").await.unwrap();

        let file = File {
            id: "test_file".to_string(),
            path: "/test/file.txt".to_string(),
            name: "file.txt".to_string(),
            directory_id: dir.id.clone(),
            size: 1024,
            file_type: Some("txt".to_string()),
            created_at: Some(Utc::now()),
            modified_at: Some(Utc::now()),
            birth_time: Some(Utc::now()),
            inode: Some(12345),
            is_directory: false,
            created_at_db: Utc::now(),
            updated_at_db: Utc::now(),
            file_size: Some(1024),
            mime_type: Some("text/plain".to_string()),
            permissions: Some("644".to_string()),
            owner_uid: Some(1000),
            group_gid: Some(1000),
            hard_links: Some(1),
            device_id: Some(12345),
            last_accessed: None,
            metadata: None,
        };

        db.add_file(&pool, &file).await.unwrap();

        // 同じファイル・タグの組み合わせを複数回追加（IGNORE句でエラーにならない）
        db.add_file_tag(&pool, &file.id, &tag.id).await.unwrap();
        db.add_file_tag(&pool, &file.id, &tag.id).await.unwrap();

        let file_tags = db.get_file_tags(&pool, &file.id).await.unwrap();
        assert_eq!(file_tags.len(), 1);
    }

    #[tokio::test]
    async fn test_remove_nonexistent_file_tag() {
        let pool = setup_test_db().await;
        let db = Database;

        // 存在しないファイル・タグの組み合わせを削除（エラーにならない）
        let result = db
            .remove_file_tag(&pool, "nonexistent_file", "nonexistent_tag")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_file_tags_nonexistent_file() {
        let pool = setup_test_db().await;
        let db = Database;

        let file_tags = db.get_file_tags(&pool, "nonexistent_file").await.unwrap();
        assert_eq!(file_tags.len(), 0);
    }

    #[tokio::test]
    async fn test_file_exists_by_path() {
        let pool = setup_test_db().await;
        let db = Database;

        let dir = db.add_directory(&pool, "/test", "test").await.unwrap();

        let file = File {
            id: "test_file".to_string(),
            path: "/test/file.txt".to_string(),
            name: "file.txt".to_string(),
            directory_id: dir.id.clone(),
            size: 1024,
            file_type: Some("txt".to_string()),
            created_at: Some(Utc::now()),
            modified_at: Some(Utc::now()),
            birth_time: None,
            inode: Some(12345),
            is_directory: false,
            created_at_db: Utc::now(),
            updated_at_db: Utc::now(),
            file_size: Some(1024),
            mime_type: Some("text/plain".to_string()),
            permissions: Some("644".to_string()),
            owner_uid: Some(1000),
            group_gid: Some(1000),
            hard_links: Some(1),
            device_id: Some(12345),
            last_accessed: None,
            metadata: None,
        };

        // ファイルを追加
        db.add_file(&pool, &file).await.unwrap();

        // ファイルが存在することを確認
        let exists = db
            .file_exists_by_path(&pool, "/test/file.txt")
            .await
            .unwrap();
        assert!(exists);

        // 存在しないファイルを確認
        let not_exists = db
            .file_exists_by_path(&pool, "/test/nonexistent.txt")
            .await
            .unwrap();
        assert!(!not_exists);
    }

    #[tokio::test]
    async fn test_find_file_by_inode() {
        let pool = setup_test_db().await;
        let db = Database;

        let dir = db.add_directory(&pool, "/test", "test").await.unwrap();

        let file = File {
            id: "test_file".to_string(),
            path: "/test/file.txt".to_string(),
            name: "file.txt".to_string(),
            directory_id: dir.id.clone(),
            size: 1024,
            file_type: Some("txt".to_string()),
            created_at: Some(Utc::now()),
            modified_at: Some(Utc::now()),
            birth_time: None,
            inode: Some(12345),
            is_directory: false,
            created_at_db: Utc::now(),
            updated_at_db: Utc::now(),
            file_size: Some(1024),
            mime_type: Some("text/plain".to_string()),
            permissions: Some("644".to_string()),
            owner_uid: Some(1000),
            group_gid: Some(1000),
            hard_links: Some(1),
            device_id: Some(67890),
            last_accessed: None,
            metadata: None,
        };

        // ファイルを追加
        db.add_file(&pool, &file).await.unwrap();

        // inode番号とdevice_idでファイルを検索
        let found_file = db
            .find_file_by_inode(&pool, 12345, Some(67890))
            .await
            .unwrap();
        assert!(found_file.is_some());
        assert_eq!(found_file.unwrap().id, "test_file");

        // 存在しないinode番号で検索
        let not_found = db
            .find_file_by_inode(&pool, 99999, Some(67890))
            .await
            .unwrap();
        assert!(not_found.is_none());

        // 存在しないdevice_idで検索
        let not_found = db
            .find_file_by_inode(&pool, 12345, Some(99999))
            .await
            .unwrap();
        assert!(not_found.is_none());
    }
}

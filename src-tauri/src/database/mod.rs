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
    async fn init_database(&self, pool: &SqlitePool) -> Result<(), sqlx::Error>;
    async fn add_directory(
        &self,
        pool: &SqlitePool,
        path: &str,
        name: &str,
    ) -> Result<Directory, sqlx::Error>;
    async fn get_directories(&self, pool: &SqlitePool) -> Result<Vec<Directory>, sqlx::Error>;
    async fn remove_directory(&self, pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error>;
    async fn add_file(&self, pool: &SqlitePool, file: &File) -> Result<(), sqlx::Error>;
    async fn get_files_by_directory(
        &self,
        pool: &SqlitePool,
        directory_id: &str,
    ) -> Result<Vec<File>, sqlx::Error>;
    async fn get_files_by_directory_sorted(
        &self,
        pool: &SqlitePool,
        directory_id: &str,
        sort_field: Option<String>,
        sort_order: Option<String>,
    ) -> Result<Vec<File>, sqlx::Error>;
    async fn get_all_files(&self, pool: &SqlitePool) -> Result<Vec<File>, sqlx::Error>;
    async fn get_all_files_sorted(
        &self,
        pool: &SqlitePool,
        sort_field: Option<String>,
        sort_order: Option<String>,
    ) -> Result<Vec<File>, sqlx::Error>;
    async fn get_all_files_paginated(
        &self,
        pool: &SqlitePool,
        sort_field: Option<String>,
        sort_order: Option<String>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<File>, sqlx::Error>;
    async fn get_files_by_directory_paginated(
        &self,
        pool: &SqlitePool,
        directory_id: &str,
        sort_field: Option<String>,
        sort_order: Option<String>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<File>, sqlx::Error>;
    async fn count_all_files(&self, pool: &SqlitePool) -> Result<u32, sqlx::Error>;
    async fn count_files_by_directory(&self, pool: &SqlitePool, directory_id: &str) -> Result<u32, sqlx::Error>;
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
    // カスタムメタデータキー管理
    async fn create_custom_metadata_key(
        &self,
        pool: &SqlitePool,
        key: &CustomMetadataKey,
    ) -> Result<CustomMetadataKey, sqlx::Error>;
    async fn get_all_custom_metadata_keys(
        &self,
        pool: &SqlitePool,
    ) -> Result<Vec<CustomMetadataKey>, sqlx::Error>;
    async fn update_custom_metadata_key(
        &self,
        pool: &SqlitePool,
        key: &CustomMetadataKey,
    ) -> Result<CustomMetadataKey, sqlx::Error>;
    async fn delete_custom_metadata_key(
        &self,
        pool: &SqlitePool,
        key_id: &str,
    ) -> Result<(), sqlx::Error>;
    async fn get_custom_metadata_key_by_name(
        &self,
        pool: &SqlitePool,
        name: &str,
    ) -> Result<Option<CustomMetadataKey>, sqlx::Error>;
    // カスタムメタデータ値管理
    async fn set_custom_metadata_value(
        &self,
        pool: &SqlitePool,
        file_id: &str,
        key_id: &str,
        value: Option<String>,
    ) -> Result<CustomMetadataValue, sqlx::Error>;
    async fn get_custom_metadata_values_by_file(
        &self,
        pool: &SqlitePool,
        file_id: &str,
    ) -> Result<Vec<CustomMetadataValue>, sqlx::Error>;
    async fn get_custom_metadata_value(
        &self,
        pool: &SqlitePool,
        file_id: &str,
        key_id: &str,
    ) -> Result<Option<CustomMetadataValue>, sqlx::Error>;
    async fn delete_custom_metadata_value(
        &self,
        pool: &SqlitePool,
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
    async fn init_database(&self, _pool: &SqlitePool) -> Result<(), sqlx::Error> {
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

    async fn remove_directory(&self, pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM directories WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
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

    async fn get_files_by_directory(
        &self,
        pool: &SqlitePool,
        directory_id: &str,
    ) -> Result<Vec<File>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM files WHERE directory_id = ? ORDER BY name")
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

    async fn get_all_files(&self, pool: &SqlitePool) -> Result<Vec<File>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM files ORDER BY name")
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
            "SELECT * FROM files ORDER BY {} {} NULLS LAST",
            sort_column, order_direction
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
            "SELECT * FROM files WHERE directory_id = ? ORDER BY {} {} NULLS LAST",
            sort_column, order_direction
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
            "SELECT * FROM files ORDER BY {} {} NULLS LAST LIMIT ? OFFSET ?",
            sort_column, order_direction
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
            "SELECT * FROM files WHERE directory_id = ? ORDER BY {} {} NULLS LAST LIMIT ? OFFSET ?",
            sort_column, order_direction
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

    async fn count_all_files(&self, pool: &SqlitePool) -> Result<u32, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM files")
            .fetch_one(pool)
            .await?;
        Ok(count as u32)
    }

    async fn count_files_by_directory(&self, pool: &SqlitePool, directory_id: &str) -> Result<u32, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM files WHERE directory_id = ?")
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
        let search_pattern = format!("%{}%", query);
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
        pool: &SqlitePool,
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
        .execute(pool)
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
        pool: &SqlitePool,
    ) -> Result<Vec<CustomMetadataKey>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM custom_metadata_keys ORDER BY name")
            .fetch_all(pool)
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
        pool: &SqlitePool,
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
        .execute(pool)
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
        pool: &SqlitePool,
        key_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM custom_metadata_keys WHERE id = ?")
            .bind(key_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    async fn get_custom_metadata_key_by_name(
        &self,
        pool: &SqlitePool,
        name: &str,
    ) -> Result<Option<CustomMetadataKey>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM custom_metadata_keys WHERE name = ?")
            .bind(name)
            .fetch_optional(pool)
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
        pool: &SqlitePool,
        file_id: &str,
        key_id: &str,
        value: Option<String>,
    ) -> Result<CustomMetadataValue, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        sqlx::query(
            "INSERT OR REPLACE INTO custom_metadata_values (id, file_id, key_id, value, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(file_id)
        .bind(key_id)
        .bind(value.as_deref())
        .bind(now)
        .bind(now)
        .execute(pool)
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

    async fn get_custom_metadata_values_by_file(
        &self,
        pool: &SqlitePool,
        file_id: &str,
    ) -> Result<Vec<CustomMetadataValue>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM custom_metadata_values WHERE file_id = ?")
            .bind(file_id)
            .fetch_all(pool)
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
        pool: &SqlitePool,
        file_id: &str,
        key_id: &str,
    ) -> Result<Option<CustomMetadataValue>, sqlx::Error> {
        let row =
            sqlx::query("SELECT * FROM custom_metadata_values WHERE file_id = ? AND key_id = ?")
                .bind(file_id)
                .bind(key_id)
                .fetch_optional(pool)
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
        pool: &SqlitePool,
        file_id: &str,
        key_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM custom_metadata_values WHERE file_id = ? AND key_id = ?")
            .bind(file_id)
            .bind(key_id)
            .execute(pool)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use sqlx::SqlitePool;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();

        // テスト用のテーブル作成
        sqlx::query(
            "CREATE TABLE directories (
                id TEXT PRIMARY KEY,
                path TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE files (
                id TEXT PRIMARY KEY,
                path TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                directory_id TEXT NOT NULL,
                size INTEGER NOT NULL,
                file_type TEXT,
                created_at TIMESTAMP,
                modified_at TIMESTAMP,
                birth_time TIMESTAMP,
                inode INTEGER,
                is_directory BOOLEAN NOT NULL DEFAULT 0,
                created_at_db TIMESTAMP NOT NULL,
                updated_at_db TIMESTAMP NOT NULL,
                file_size INTEGER,
                mime_type TEXT,
                permissions TEXT,
                owner_uid INTEGER,
                group_gid INTEGER,
                hard_links INTEGER,
                device_id INTEGER,
                last_accessed TIMESTAMP,
                metadata TEXT DEFAULT '{}',
                FOREIGN KEY (directory_id) REFERENCES directories (id) ON DELETE CASCADE
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE tags (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                color TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE file_tags (
                file_id TEXT NOT NULL,
                tag_id TEXT NOT NULL,
                PRIMARY KEY (file_id, tag_id),
                FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
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
    async fn test_remove_directory() {
        let pool = setup_test_db().await;
        let db = Database;

        let dir = db
            .add_directory(&pool, "/test/remove", "remove_dir")
            .await
            .unwrap();
        let result = db.remove_directory(&pool, &dir.id).await;
        assert!(result.is_ok());

        let directories = db.get_directories(&pool).await.unwrap();
        assert_eq!(directories.len(), 0);
    }

    #[tokio::test]
    async fn test_get_files_by_directory() {
        let pool = setup_test_db().await;
        let db = Database;

        let dir = db.add_directory(&pool, "/test", "test").await.unwrap();

        let file1 = File {
            id: "file1".to_string(),
            path: "/test/file1.txt".to_string(),
            name: "file1.txt".to_string(),
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

        let file2 = File {
            id: "file2".to_string(),
            path: "/test/file2.txt".to_string(),
            name: "file2.txt".to_string(),
            directory_id: dir.id.clone(),
            size: 2048,
            file_type: Some("txt".to_string()),
            created_at: Some(Utc::now()),
            modified_at: Some(Utc::now()),
            birth_time: None,
            inode: Some(54321),
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

        db.add_file(&pool, &file1).await.unwrap();
        db.add_file(&pool, &file2).await.unwrap();

        let files = db.get_files_by_directory(&pool, &dir.id).await.unwrap();
        assert_eq!(files.len(), 2);
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
    async fn test_get_all_files() {
        let pool = setup_test_db().await;
        let db = Database;

        // ディレクトリを作成
        let dir1 = db.add_directory(&pool, "/test/dir1", "dir1").await.unwrap();
        let dir2 = db.add_directory(&pool, "/test/dir2", "dir2").await.unwrap();

        // ファイルを作成
        let file1 = File {
            id: "file1".to_string(),
            path: "/test/dir1/file1.txt".to_string(),
            name: "file1.txt".to_string(),
            directory_id: dir1.id.clone(),
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

        let file2 = File {
            id: "file2".to_string(),
            path: "/test/dir2/file2.txt".to_string(),
            name: "file2.txt".to_string(),
            directory_id: dir2.id.clone(),
            size: 2048,
            file_type: Some("txt".to_string()),
            created_at: Some(Utc::now()),
            modified_at: Some(Utc::now()),
            birth_time: None,
            inode: Some(54321),
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

        db.add_file(&pool, &file1).await.unwrap();
        db.add_file(&pool, &file2).await.unwrap();

        // 全ファイルを取得
        let all_files = db.get_all_files(&pool).await.unwrap();
        assert_eq!(all_files.len(), 2);

        // ファイル名でソートされていることを確認
        assert_eq!(all_files[0].name, "file1.txt");
        assert_eq!(all_files[1].name, "file2.txt");

        // 異なるディレクトリのファイルが含まれていることを確認
        assert_eq!(all_files[0].directory_id, dir1.id);
        assert_eq!(all_files[1].directory_id, dir2.id);
    }

    #[tokio::test]
    async fn test_get_all_files_empty() {
        let pool = setup_test_db().await;
        let db = Database;

        let all_files = db.get_all_files(&pool).await.unwrap();
        assert_eq!(all_files.len(), 0);
    }

    #[tokio::test]
    async fn test_get_all_files_mixed_types() {
        let pool = setup_test_db().await;
        let db = Database;

        let dir = db.add_directory(&pool, "/test", "test").await.unwrap();

        // 通常ファイル
        let file = File {
            id: "file1".to_string(),
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

        // ディレクトリファイル
        let subdir = File {
            id: "dir1".to_string(),
            path: "/test/subdir".to_string(),
            name: "subdir".to_string(),
            directory_id: dir.id.clone(),
            size: 0,
            file_type: None,
            created_at: Some(Utc::now()),
            modified_at: Some(Utc::now()),
            birth_time: None,
            inode: Some(54321),
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

        db.add_file(&pool, &file).await.unwrap();
        db.add_file(&pool, &subdir).await.unwrap();

        let all_files = db.get_all_files(&pool).await.unwrap();
        assert_eq!(all_files.len(), 2);

        // ファイルとディレクトリが両方含まれていることを確認
        let file_types: Vec<bool> = all_files.iter().map(|f| f.is_directory).collect();
        assert!(file_types.contains(&true)); // ディレクトリが含まれている
        assert!(file_types.contains(&false)); // 通常ファイルが含まれている
    }

    #[tokio::test]
    async fn test_init_database() {
        let pool = setup_test_db().await;
        let db = Database;

        let result = db.init_database(&pool).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_file_by_path() {
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
        let files = db.get_files_by_directory(&pool, &dir.id).await.unwrap();
        assert_eq!(files.len(), 1);

        // パスでファイルを削除
        db.remove_file_by_path(&pool, "/test/file.txt")
            .await
            .unwrap();

        // ファイルが削除されていることを確認
        let files = db.get_files_by_directory(&pool, &dir.id).await.unwrap();
        assert_eq!(files.len(), 0);
    }

    #[tokio::test]
    async fn test_update_file_metadata() {
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

        // テスト用の一時ファイルを作成
        let temp_file = std::env::temp_dir().join("test_file.txt");
        std::fs::write(&temp_file, "test content").unwrap();

        // ファイルメタデータを取得
        let metadata = std::fs::metadata(&temp_file).unwrap();

        // メタデータを更新
        let result = db
            .update_file_metadata(&pool, "/test/file.txt", &metadata)
            .await;
        assert!(result.is_ok());

        // 更新されたファイルを取得
        let updated_files = db.get_files_by_directory(&pool, &dir.id).await.unwrap();
        assert_eq!(updated_files.len(), 1);

        let updated_file = &updated_files[0];
        assert_eq!(updated_file.size, metadata.len() as i64);

        // 一時ファイルを削除
        std::fs::remove_file(&temp_file).unwrap();
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

    #[tokio::test]
    async fn test_update_file_path() {
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

        // ファイルパスを更新
        db.update_file_path(
            &pool,
            "test_file",
            "/test/renamed_file.txt",
            "renamed_file.txt",
        )
        .await
        .unwrap();

        // 更新されたファイルを取得
        let updated_files = db.get_files_by_directory(&pool, &dir.id).await.unwrap();
        assert_eq!(updated_files.len(), 1);
        assert_eq!(updated_files[0].path, "/test/renamed_file.txt");
        assert_eq!(updated_files[0].name, "renamed_file.txt");

        // 元のパスでは見つからないことを確認
        let exists = db
            .file_exists_by_path(&pool, "/test/file.txt")
            .await
            .unwrap();
        assert!(!exists);

        // 新しいパスでは見つかることを確認
        let exists = db
            .file_exists_by_path(&pool, "/test/renamed_file.txt")
            .await
            .unwrap();
        assert!(exists);
    }
}

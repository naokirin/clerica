use sqlx::{Row, SqlitePool};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Directory {
    pub id: String,
    pub path: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

pub async fn init_database(_pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // マイグレーションは自動的に実行されるため、ここでは初期データの挿入のみ
    Ok(())
}

pub async fn add_directory(pool: &SqlitePool, path: &str, name: &str) -> Result<Directory, sqlx::Error> {
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

pub async fn get_directories(pool: &SqlitePool) -> Result<Vec<Directory>, sqlx::Error> {
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

pub async fn remove_directory(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM directories WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    Ok(())
}

pub async fn add_file(pool: &SqlitePool, file: &File) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT OR REPLACE INTO files (id, path, name, directory_id, size, file_type, created_at, modified_at, birth_time, inode, is_directory, created_at_db, updated_at_db) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
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
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn get_files_by_directory(pool: &SqlitePool, directory_id: &str) -> Result<Vec<File>, sqlx::Error> {
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
        });
    }
    
    Ok(files)
}

pub async fn get_all_tags(pool: &SqlitePool) -> Result<Vec<Tag>, sqlx::Error> {
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

pub async fn create_tag(pool: &SqlitePool, name: &str, color: &str) -> Result<Tag, sqlx::Error> {
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

pub async fn add_file_tag(pool: &SqlitePool, file_id: &str, tag_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT OR IGNORE INTO file_tags (file_id, tag_id) VALUES (?, ?)")
        .bind(file_id)
        .bind(tag_id)
        .execute(pool)
        .await?;
    
    Ok(())
}

pub async fn remove_file_tag(pool: &SqlitePool, file_id: &str, tag_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM file_tags WHERE file_id = ? AND tag_id = ?")
        .bind(file_id)
        .bind(tag_id)
        .execute(pool)
        .await?;
    
    Ok(())
}

pub async fn get_file_tags(pool: &SqlitePool, file_id: &str) -> Result<Vec<Tag>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT t.* FROM tags t 
         INNER JOIN file_tags ft ON t.id = ft.tag_id 
         WHERE ft.file_id = ? 
         ORDER BY t.name"
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
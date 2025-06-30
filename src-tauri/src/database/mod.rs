use sqlx::{Row, SqlitePool};
use uuid::Uuid;
use chrono::{DateTime, Utc};

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
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
use mockall::predicate::*;

#[cfg_attr(test, mockall::automock)]
pub trait DatabaseTrait {
    async fn init_database(&self, pool: &SqlitePool) -> Result<(), sqlx::Error>;
    async fn add_directory(&self, pool: &SqlitePool, path: &str, name: &str) -> Result<Directory, sqlx::Error>;
    async fn get_directories(&self, pool: &SqlitePool) -> Result<Vec<Directory>, sqlx::Error>;
    async fn remove_directory(&self, pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error>;
    async fn add_file(&self, pool: &SqlitePool, file: &File) -> Result<(), sqlx::Error>;
    async fn get_files_by_directory(&self, pool: &SqlitePool, directory_id: &str) -> Result<Vec<File>, sqlx::Error>;
    async fn get_all_tags(&self, pool: &SqlitePool) -> Result<Vec<Tag>, sqlx::Error>;
    async fn create_tag(&self, pool: &SqlitePool, name: &str, color: &str) -> Result<Tag, sqlx::Error>;
    async fn add_file_tag(&self, pool: &SqlitePool, file_id: &str, tag_id: &str) -> Result<(), sqlx::Error>;
    async fn remove_file_tag(&self, pool: &SqlitePool, file_id: &str, tag_id: &str) -> Result<(), sqlx::Error>;
    async fn get_file_tags(&self, pool: &SqlitePool, file_id: &str) -> Result<Vec<Tag>, sqlx::Error>;
}

pub struct Database;

impl DatabaseTrait for Database {
    async fn init_database(&self, _pool: &SqlitePool) -> Result<(), sqlx::Error> {
        // マイグレーションは自動的に実行されるため、ここでは初期データの挿入のみ
        Ok(())
    }

    async fn add_directory(&self, pool: &SqlitePool, path: &str, name: &str) -> Result<Directory, sqlx::Error> {
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

    async fn get_files_by_directory(&self, pool: &SqlitePool, directory_id: &str) -> Result<Vec<File>, sqlx::Error> {
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

    async fn create_tag(&self, pool: &SqlitePool, name: &str, color: &str) -> Result<Tag, sqlx::Error> {
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

    async fn add_file_tag(&self, pool: &SqlitePool, file_id: &str, tag_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT OR IGNORE INTO file_tags (file_id, tag_id) VALUES (?, ?)")
            .bind(file_id)
            .bind(tag_id)
            .execute(pool)
            .await?;
        
        Ok(())
    }

    async fn remove_file_tag(&self, pool: &SqlitePool, file_id: &str, tag_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM file_tags WHERE file_id = ? AND tag_id = ?")
            .bind(file_id)
            .bind(tag_id)
            .execute(pool)
            .await?;
        
        Ok(())
    }

    async fn get_file_tags(&self, pool: &SqlitePool, file_id: &str) -> Result<Vec<Tag>, sqlx::Error> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use chrono::Utc;

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
            )"
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
                FOREIGN KEY (directory_id) REFERENCES directories (id) ON DELETE CASCADE
            )"
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
            )"
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
            )"
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
        
        db.add_directory(&pool, "/test/path1", "dir1").await.unwrap();
        db.add_directory(&pool, "/test/path2", "dir2").await.unwrap();
        
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
        
        let dir = db.add_directory(&pool, "/test/remove", "remove_dir").await.unwrap();
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
    async fn test_init_database() {
        let pool = setup_test_db().await;
        let db = Database;
        
        let result = db.init_database(&pool).await;
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
        let result = db.remove_file_tag(&pool, "nonexistent_file", "nonexistent_tag").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_file_tags_nonexistent_file() {
        let pool = setup_test_db().await;
        let db = Database;
        
        let file_tags = db.get_file_tags(&pool, "nonexistent_file").await.unwrap();
        assert_eq!(file_tags.len(), 0);
    }
}
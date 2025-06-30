#[cfg(test)]
mod tests {
    use crate::database::{Directory, File, Database, DatabaseTrait};
    use crate::file_manager::scan_directory;
    use sqlx::SqlitePool;
    use chrono::Utc;
    use std::fs;
    use tempfile::TempDir;

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

        pool
    }

    fn create_mock_directory() -> Directory {
        Directory {
            id: "test_id".to_string(),
            path: "/test/path".to_string(),
            name: "test_dir".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_mock_file() -> File {
        File {
            id: "file_id".to_string(),
            path: "/test/file.txt".to_string(),
            name: "file.txt".to_string(),
            directory_id: "dir_id".to_string(),
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
        }
    }

    #[tokio::test]
    async fn test_scan_directory_with_files() {
        let pool = setup_test_db().await;
        let db = Database;
        
        // 一時ディレクトリとファイルを作成
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // テストファイルを作成
        fs::write(temp_path.join("test1.txt"), "test content 1").unwrap();
        fs::write(temp_path.join("test2.md"), "test content 2").unwrap();
        
        // サブディレクトリとファイルを作成
        let sub_dir = temp_path.join("subdir");
        fs::create_dir(&sub_dir).unwrap();
        fs::write(sub_dir.join("test3.txt"), "test content 3").unwrap();
        
        // ディレクトリを登録
        let directory = db.add_directory(&pool, temp_path.to_str().unwrap(), "test_dir").await.unwrap();
        
        // ファイルスキャンを実行
        let result = scan_directory(&pool, &directory.id, temp_path.to_str().unwrap()).await;
        assert!(result.is_ok());
        
        // スキャンされたファイルを確認
        let files = db.get_files_by_directory(&pool, &directory.id).await.unwrap();
        assert!(files.len() >= 4); // ディレクトリ自体も含めて4つ以上のエントリがスキャンされる
        
        // ファイル名が含まれていることを確認
        let file_names: Vec<String> = files.iter().map(|f| f.name.clone()).collect();
        assert!(file_names.contains(&"test1.txt".to_string()));
        assert!(file_names.contains(&"test2.md".to_string()));
        assert!(file_names.contains(&"subdir".to_string()));
    }

    #[tokio::test]
    async fn test_scan_directory_empty() {
        let pool = setup_test_db().await;
        let db = Database;
        
        // 空の一時ディレクトリを作成
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // ディレクトリを登録
        let directory = db.add_directory(&pool, temp_path.to_str().unwrap(), "empty_dir").await.unwrap();
        
        // ファイルスキャンを実行
        let result = scan_directory(&pool, &directory.id, temp_path.to_str().unwrap()).await;
        assert!(result.is_ok());
        
        // 空ディレクトリでもディレクトリ自体のエントリが作成される
        let files = db.get_files_by_directory(&pool, &directory.id).await.unwrap();
        assert!(files.len() >= 1); // 少なくとも1つのエントリ（ディレクトリ自体）
    }

    #[tokio::test]
    async fn test_scan_directory_file_types() {
        let pool = setup_test_db().await;
        let db = Database;
        
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // 異なる拡張子のファイルを作成
        fs::write(temp_path.join("document.pdf"), "pdf content").unwrap();
        fs::write(temp_path.join("image.jpg"), "jpg content").unwrap();
        fs::write(temp_path.join("no_extension"), "no ext content").unwrap();
        
        let directory = db.add_directory(&pool, temp_path.to_str().unwrap(), "test_types").await.unwrap();
        scan_directory(&pool, &directory.id, temp_path.to_str().unwrap()).await.unwrap();
        
        let files = db.get_files_by_directory(&pool, &directory.id).await.unwrap();
        
        // ファイルタイプが正しく設定されていることを確認
        let pdf_file = files.iter().find(|f| f.name == "document.pdf").unwrap();
        assert_eq!(pdf_file.file_type, Some("pdf".to_string()));
        
        let jpg_file = files.iter().find(|f| f.name == "image.jpg").unwrap();
        assert_eq!(jpg_file.file_type, Some("jpg".to_string()));
        
        let no_ext_file = files.iter().find(|f| f.name == "no_extension").unwrap();
        assert_eq!(no_ext_file.file_type, None);
    }

    #[tokio::test]
    async fn test_get_all_files_integration() {
        let pool = setup_test_db().await;
        let db = Database;
        
        // 複数のディレクトリとファイルを作成
        let temp_dir1 = TempDir::new().unwrap();
        let temp_dir2 = TempDir::new().unwrap();
        
        fs::write(temp_dir1.path().join("file1.txt"), "content 1").unwrap();
        fs::write(temp_dir2.path().join("file2.txt"), "content 2").unwrap();
        
        // ディレクトリを登録してスキャン
        let dir1 = db.add_directory(&pool, temp_dir1.path().to_str().unwrap(), "dir1").await.unwrap();
        let dir2 = db.add_directory(&pool, temp_dir2.path().to_str().unwrap(), "dir2").await.unwrap();
        
        scan_directory(&pool, &dir1.id, temp_dir1.path().to_str().unwrap()).await.unwrap();
        scan_directory(&pool, &dir2.id, temp_dir2.path().to_str().unwrap()).await.unwrap();
        
        // 全ファイルを取得（ディレクトリ自体も含む）
        let all_files = db.get_all_files(&pool).await.unwrap();
        assert!(all_files.len() >= 2); // 最低2つのファイル（ディレクトリエントリも含む可能性）
        
        let file_names: Vec<String> = all_files.iter().map(|f| f.name.clone()).collect();
        assert!(file_names.contains(&"file1.txt".to_string()));
        assert!(file_names.contains(&"file2.txt".to_string()));
    }

    #[test]
    fn test_file_struct_creation() {
        let file = create_mock_file();
        
        assert_eq!(file.id, "file_id");
        assert_eq!(file.name, "file.txt");
        assert_eq!(file.size, 1024);
        assert!(!file.is_directory);
    }

    #[test]
    fn test_directory_struct_creation() {
        let directory = create_mock_directory();
        
        assert_eq!(directory.id, "test_id");
        assert_eq!(directory.path, "/test/path");
        assert_eq!(directory.name, "test_dir");
    }

    #[tokio::test]
    async fn test_rescan_directory_functionality() {
        let pool = setup_test_db().await;
        let db = Database;
        
        // 一時ディレクトリを作成
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // 初期ファイルを作成
        fs::write(temp_path.join("initial.txt"), "initial content").unwrap();
        
        // ディレクトリを登録してスキャン
        let directory = db.add_directory(&pool, temp_path.to_str().unwrap(), "test_rescan").await.unwrap();
        scan_directory(&pool, &directory.id, temp_path.to_str().unwrap()).await.unwrap();
        
        let initial_files = db.get_files_by_directory(&pool, &directory.id).await.unwrap();
        assert!(initial_files.len() >= 1);
        let file_names: Vec<String> = initial_files.iter().map(|f| f.name.clone()).collect();
        assert!(file_names.contains(&"initial.txt".to_string()));
        
        // 新しいファイルを追加
        fs::write(temp_path.join("new_file.txt"), "new content").unwrap();
        
        // 再スキャンを実行（rescan_directoryのロジックをテスト）
        // 既存のファイル情報を削除
        sqlx::query("DELETE FROM files WHERE directory_id = ?")
            .bind(&directory.id)
            .execute(&pool)
            .await
            .unwrap();
        
        // ディレクトリを再スキャン
        scan_directory(&pool, &directory.id, temp_path.to_str().unwrap()).await.unwrap();
        
        // 再スキャン後のファイルを確認
        let rescanned_files = db.get_files_by_directory(&pool, &directory.id).await.unwrap();
        assert!(rescanned_files.len() >= 2);
        
        let file_names: Vec<String> = rescanned_files.iter().map(|f| f.name.clone()).collect();
        assert!(file_names.contains(&"initial.txt".to_string()));
        assert!(file_names.contains(&"new_file.txt".to_string()));
    }

    #[tokio::test]
    async fn test_rescan_directory_with_deleted_files() {
        let pool = setup_test_db().await;
        let db = Database;
        
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // 複数のファイルを作成
        fs::write(temp_path.join("file1.txt"), "content 1").unwrap();
        fs::write(temp_path.join("file2.txt"), "content 2").unwrap();
        fs::write(temp_path.join("file3.txt"), "content 3").unwrap();
        
        let directory = db.add_directory(&pool, temp_path.to_str().unwrap(), "test_delete").await.unwrap();
        scan_directory(&pool, &directory.id, temp_path.to_str().unwrap()).await.unwrap();
        
        let initial_files = db.get_files_by_directory(&pool, &directory.id).await.unwrap();
        assert!(initial_files.len() >= 3);
        
        // ファイルを削除
        fs::remove_file(temp_path.join("file2.txt")).unwrap();
        
        // 再スキャン
        sqlx::query("DELETE FROM files WHERE directory_id = ?")
            .bind(&directory.id)
            .execute(&pool)
            .await
            .unwrap();
        
        scan_directory(&pool, &directory.id, temp_path.to_str().unwrap()).await.unwrap();
        
        // 削除されたファイルがスキャン結果に含まれていないことを確認
        let rescanned_files = db.get_files_by_directory(&pool, &directory.id).await.unwrap();
        assert!(rescanned_files.len() >= 2);
        
        let file_names: Vec<String> = rescanned_files.iter().map(|f| f.name.clone()).collect();
        assert!(file_names.contains(&"file1.txt".to_string()));
        assert!(!file_names.contains(&"file2.txt".to_string()));
        assert!(file_names.contains(&"file3.txt".to_string()));
    }

    #[tokio::test]
    async fn test_scan_directory_with_nested_structure() {
        let pool = setup_test_db().await;
        let db = Database;
        
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // 複雑なディレクトリ構造を作成
        let level1 = temp_path.join("level1");
        let level2 = level1.join("level2");
        fs::create_dir_all(&level2).unwrap();
        
        fs::write(temp_path.join("root.txt"), "root content").unwrap();
        fs::write(level1.join("level1.txt"), "level1 content").unwrap();
        fs::write(level2.join("level2.txt"), "level2 content").unwrap();
        
        let directory = db.add_directory(&pool, temp_path.to_str().unwrap(), "nested_test").await.unwrap();
        scan_directory(&pool, &directory.id, temp_path.to_str().unwrap()).await.unwrap();
        
        let files = db.get_files_by_directory(&pool, &directory.id).await.unwrap();
        
        // ネストしたファイルとディレクトリがすべてスキャンされていることを確認
        let file_names: Vec<String> = files.iter().map(|f| f.name.clone()).collect();
        assert!(file_names.contains(&"root.txt".to_string()));
        assert!(file_names.contains(&"level1".to_string()));
        assert!(file_names.contains(&"level1.txt".to_string()));
        assert!(file_names.contains(&"level2".to_string()));
        assert!(file_names.contains(&"level2.txt".to_string()));
        
        // ディレクトリエントリが正しく識別されていることを確認
        let level1_dir = files.iter().find(|f| f.name == "level1").unwrap();
        assert!(level1_dir.is_directory);
        
        let level2_dir = files.iter().find(|f| f.name == "level2").unwrap();
        assert!(level2_dir.is_directory);
    }
}
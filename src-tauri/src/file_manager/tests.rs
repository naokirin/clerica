#[cfg(test)]
mod tests {
    use crate::database::{Directory, File, Database, DatabaseTrait};
    use crate::file_manager::{scan_directory, add_directory, remove_directory, get_files, 
                             get_file_info, update_file_tags, delete_file, move_file, 
                             get_directories, get_files_by_directory};
    use sqlx::SqlitePool;
    use chrono::Utc;
    use tauri::State;

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
        }
    }

    #[tokio::test]
    async fn test_database_trait_functionality() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        let db = Database;
        
        // テーブルが存在しない場合のエラーハンドリングをテスト
        let result = db.add_directory(&pool, "test_path", "test_name").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_scan_directory_error_handling() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        
        // 存在しないディレクトリを指定
        let result = scan_directory(&pool, "test_id", "/nonexistent/path").await;
        
        // エラーハンドリングが適切に動作することを確認
        assert!(result.is_ok()); // 実際の実装では空のディレクトリとして処理される
    }

    #[test]
    fn test_file_struct_creation() {
        let file = create_mock_file();
        
        assert_eq!(file.id, "file_id");
        assert_eq!(file.name, "file.txt");
        assert_eq!(file.size, 1024);
        assert_eq!(file.is_directory, false);
    }

    #[test]
    fn test_directory_struct_creation() {
        let directory = create_mock_directory();
        
        assert_eq!(directory.id, "test_id");
        assert_eq!(directory.path, "/test/path");
        assert_eq!(directory.name, "test_dir");
    }

    // Note: Tauriコマンドのテストは実際のTauri環境でのみ可能
    // ここでは関数の内部ロジックをテストする代替手段を使用
}
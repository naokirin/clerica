#[cfg(test)]
mod tests {
    use crate::database::{Database, DatabaseTrait, File, Tag};
    use crate::search::SearchResult;
    use sqlx::SqlitePool;
    use chrono::Utc;
    

    #[tokio::test]
    async fn test_database_operations() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        let db = Database;
        
        // テーブルが存在しない場合のエラーハンドリングをテスト
        let result = db.get_all_tags(&pool).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_search_result_creation() {
        let file = File {
            id: "test_id".to_string(),
            path: "/test/file.txt".to_string(),
            name: "file.txt".to_string(),
            directory_id: "dir_id".to_string(),
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
            last_accessed: Some(Utc::now()),
            owner_uid: Some(1000),
            group_gid: Some(1000),
            hard_links: Some(1),
            device_id: Some(12345),
        };
        
        let tag = Tag {
            id: "tag_id".to_string(),
            name: "test_tag".to_string(),
            color: "#ff0000".to_string(),
            created_at: Utc::now(),
        };
        
        let search_result = SearchResult {
            file: file.clone(),
            tags: vec![tag],
            score: 0.8,
        };
        
        assert_eq!(search_result.file.id, "test_id");
        assert_eq!(search_result.tags.len(), 1);
        assert_eq!(search_result.score, 0.8);
    }

    // Note: Tauriコマンドのテストは実際のTauri環境でのみ可能
    // ここでは内部ロジックのテストにフォーカス
}
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Shelf {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShelfConfig {
    pub active_shelf_id: String,
    pub shelves: Vec<Shelf>,
}

#[derive(Clone)]
pub struct ShelfManager {
    pub settings_pool: SqlitePool,
    pub data_pools: Arc<Mutex<HashMap<String, SqlitePool>>>,
    pub active_shelf_id: Arc<Mutex<String>>,
}

impl ShelfManager {
    pub async fn new(settings_pool: SqlitePool) -> Result<Self, Box<dyn std::error::Error>> {
        let shelf_manager = ShelfManager {
            settings_pool,
            data_pools: Arc::new(Mutex::new(HashMap::new())),
            active_shelf_id: Arc::new(Mutex::new(String::new())),
        };

        shelf_manager.initialize_shelves().await?;
        Ok(shelf_manager)
    }

    async fn initialize_shelves(&self) -> Result<(), Box<dyn std::error::Error>> {
        // shelvesテーブルが存在しない場合は作成
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS shelves (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.settings_pool)
        .await?;

        // active_shelf設定テーブルを作成
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS active_shelf (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                shelf_id TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.settings_pool)
        .await?;

        // デフォルトグループが存在しない場合は作成
        let shelves = self.get_shelves().await?;
        if shelves.is_empty() {
            let default_shelf = Shelf {
                id: Uuid::new_v4().to_string(),
                name: "デフォルト".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
            };
            self.create_shelf(&default_shelf).await?;
            self.set_active_shelf(&default_shelf.id).await?;
        } else {
            // アクティブグループを取得または最初のグループを設定
            let active_shelf_id = match self.get_active_shelf_id().await {
                Ok(id) => id,
                Err(_) => {
                    let first_shelf_id = &shelves[0].id;
                    self.set_active_shelf(first_shelf_id).await?;
                    first_shelf_id.clone()
                }
            };
            *self.active_shelf_id.lock().unwrap() = active_shelf_id;
        }

        // アクティブグループのデータベース接続を初期化
        let active_id = self.active_shelf_id.lock().unwrap().clone();
        self.get_or_create_data_pool(&active_id).await?;

        Ok(())
    }

    pub async fn get_shelves(&self) -> Result<Vec<Shelf>, Box<dyn std::error::Error>> {
        let rows = sqlx::query_as::<_, Shelf>(
            "SELECT id, name, created_at FROM shelves ORDER BY created_at"
        )
        .fetch_all(&self.settings_pool)
        .await?;

        Ok(rows)
    }

    pub async fn create_shelf(&self, shelf: &Shelf) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(
            "INSERT INTO shelves (id, name, created_at) VALUES (?, ?, ?)"
        )
        .bind(&shelf.id)
        .bind(&shelf.name)
        .bind(&shelf.created_at)
        .execute(&self.settings_pool)
        .await?;

        // 新しいグループ用のデータベースを作成
        self.get_or_create_data_pool(&shelf.id).await?;

        Ok(())
    }

    pub async fn delete_shelf(&self, shelf_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        // グループが1つしかない場合は削除を拒否
        let shelves = self.get_shelves().await?;
        if shelves.len() <= 1 {
            return Err("最後のグループは削除できません".into());
        }

        // アクティブグループの場合は他のグループに切り替え
        let active_id = self.active_shelf_id.lock().unwrap().clone();
        if active_id == shelf_id {
            let other_shelf = shelves.iter().find(|g| g.id != shelf_id).unwrap();
            self.switch_shelf(&other_shelf.id).await?;
        }

        // データベースからグループを削除
        sqlx::query("DELETE FROM shelves WHERE id = ?")
            .bind(shelf_id)
            .execute(&self.settings_pool)
            .await?;

        // データベースファイルを削除
        let db_path = self.get_data_db_path(shelf_id);
        if Path::new(&db_path).exists() {
            std::fs::remove_file(&db_path)?;
        }

        // メモリからプールを削除
        self.data_pools.lock().unwrap().remove(shelf_id);

        Ok(())
    }

    pub async fn switch_shelf(&self, shelf_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 新しいグループのデータベース接続を取得または作成
        self.get_or_create_data_pool(shelf_id).await?;

        // アクティブグループを更新
        self.set_active_shelf(shelf_id).await?;
        *self.active_shelf_id.lock().unwrap() = shelf_id.to_string();

        Ok(())
    }

    pub fn get_active_data_pool(&self) -> Result<SqlitePool, String> {
        let active_id = self.active_shelf_id.lock().unwrap().clone();
        let pools = self.data_pools.lock().unwrap();
        pools
            .get(&active_id)
            .cloned()
            .ok_or_else(|| "アクティブグループのデータベース接続が見つかりません".to_string())
    }

    pub fn get_active_shelf_id_sync(&self) -> String {
        self.active_shelf_id.lock().unwrap().clone()
    }

    // DatabaseManagerとの互換性のためのメソッド
    pub fn get_settings_pool(&self) -> &SqlitePool {
        &self.settings_pool
    }

    pub fn get_data_pool(&self) -> Result<SqlitePool, String> {
        self.get_active_data_pool()
    }

    async fn get_active_shelf_id(&self) -> Result<String, Box<dyn std::error::Error>> {
        let row = sqlx::query("SELECT shelf_id FROM active_shelf WHERE id = 1")
            .fetch_one(&self.settings_pool)
            .await?;
        Ok(row.get("shelf_id"))
    }

    async fn set_active_shelf(&self, shelf_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(
            "INSERT OR REPLACE INTO active_shelf (id, shelf_id) VALUES (1, ?)"
        )
        .bind(shelf_id)
        .execute(&self.settings_pool)
        .await?;
        Ok(())
    }

    async fn get_or_create_data_pool(
        &self,
        shelf_id: &str,
    ) -> Result<SqlitePool, Box<dyn std::error::Error>> {
        {
            let pools = self.data_pools.lock().unwrap();
            if let Some(pool) = pools.get(shelf_id) {
                return Ok(pool.clone());
            }
        }

        // 新しい接続プールを作成
        let db_path = self.get_data_db_path(shelf_id);
        let db_url = format!("sqlite:{}", db_path);

        let connect_options = sqlx::sqlite::SqliteConnectOptions::from_str(&db_url)?
            .create_if_missing(true)
            .pragma("journal_mode", "WAL")
            .pragma("synchronous", "NORMAL")
            .pragma("cache_size", "64000")
            .pragma("temp_store", "MEMORY")
            .pragma("mmap_size", "268435456");

        let pool = SqlitePool::connect_with(connect_options).await?;

        // マイグレーションを実行
        let migrator = sqlx::migrate::Migrator::new(Path::new("./data_migrations")).await?;
        migrator.run(&pool).await?;

        // プールをキャッシュに追加
        {
            let mut pools = self.data_pools.lock().unwrap();
            pools.insert(shelf_id.to_string(), pool.clone());
        }

        Ok(pool)
    }

    fn get_data_db_path(&self, shelf_id: &str) -> String {
        if cfg!(debug_assertions) {
            format!("./data_{}.db", shelf_id)
        } else {
            let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
            format!("{}/clerica_data_{}.db", home, shelf_id)
        }
    }
}
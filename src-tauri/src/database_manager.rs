use sqlx::migrate::Migrator;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::env;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone)]
pub struct DatabaseManager {
    pub settings_pool: SqlitePool,
    pub data_pool: SqlitePool,
}

impl DatabaseManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (settings_url, data_url, settings_path, data_path) = if cfg!(debug_assertions) {
            // 開発モード: プロジェクトルートに配置
            (
                "sqlite:./settings.db".to_string(),
                "sqlite:./data.db".to_string(),
                "./settings.db".to_string(),
                "./data.db".to_string(),
            )
        } else {
            // 本番モード: ホームディレクトリに配置
            let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let settings_path = format!("{home}/clerica_settings.db");
            let data_path = format!("{home}/clerica_data.db");
            (
                format!("sqlite:{settings_path}"),
                format!("sqlite:{data_path}"),
                settings_path,
                data_path,
            )
        };

        // データベースファイルの存在確認
        let settings_exists = Path::new(&settings_path).exists();
        let data_exists = Path::new(&data_path).exists();

        if !settings_exists {
            println!("設定データベースファイルが存在しません。新規作成します: {settings_path}");
        }
        if !data_exists {
            println!("データベースファイルが存在しません。新規作成します: {data_path}");
        }

        // 設定用データベース接続
        let settings_connect_options = SqliteConnectOptions::from_str(&settings_url)?
            .create_if_missing(true)
            .pragma("journal_mode", "WAL")
            .pragma("synchronous", "NORMAL")
            .pragma("cache_size", "64000")
            .pragma("temp_store", "MEMORY")
            .pragma("mmap_size", "268435456");

        let settings_pool = SqlitePool::connect_with(settings_connect_options).await?;

        // データ用データベース接続（読み取り用と書き込み用）
        let data_connect_options_write = SqliteConnectOptions::from_str(&data_url)?
            .create_if_missing(true)
            .pragma("journal_mode", "WAL")
            .pragma("synchronous", "NORMAL")
            .pragma("cache_size", "64000")
            .pragma("temp_store", "MEMORY")
            .pragma("mmap_size", "268435456");

        let data_pool = SqlitePool::connect_with(data_connect_options_write).await?;

        // マイグレーション実行
        Self::run_migrations(&settings_pool, &data_pool).await?;

        Ok(DatabaseManager {
            settings_pool,
            data_pool,
        })
    }

    async fn run_migrations(
        settings_pool: &SqlitePool,
        data_pool: &SqlitePool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 設定用データベースのマイグレーション
        let settings_migrator = Migrator::new(Path::new("./settings_migrations")).await?;
        settings_migrator.run(settings_pool).await?;
        println!("設定データベースのマイグレーションが完了しました");

        // データ用データベースのマイグレーション
        let data_migrator = Migrator::new(Path::new("./data_migrations")).await?;
        data_migrator.run(data_pool).await?;
        println!("データデータベースのマイグレーションが完了しました");

        Ok(())
    }

    pub fn get_settings_pool(&self) -> &SqlitePool {
        &self.settings_pool
    }

    pub fn get_data_pool(&self) -> &SqlitePool {
        &self.data_pool
    }
}
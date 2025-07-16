use sqlx::migrate::Migrator;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::env;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone)]
pub struct DatabaseManager {
    pub settings_pool: SqlitePool,
}

impl DatabaseManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (settings_url, settings_path) = if cfg!(debug_assertions) {
            std::fs::create_dir_all("../db").unwrap_or_else(|_| {
                eprintln!("プロジェクトルートに設定ディレクトリを作成できません: ../db");
            });
            // 開発モード: プロジェクトルートに配置
            (
                "sqlite:../db/settings.db".to_string(),
                "../db/settings.db".to_string(),
            )
        } else {
            // 本番モード: ホームディレクトリに配置
            let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
            std::fs::create_dir_all(format!("{home}/.clerica")).unwrap_or_else(|_| {
                eprintln!("ホームディレクトリに設定ディレクトリを作成できません: {home}/.clerica");
            });
            let settings_path = format!("{home}/.clerica/clerica_settings.db");
            (format!("sqlite:{settings_path}"), settings_path)
        };

        // データベースファイルの存在確認
        let settings_exists = Path::new(&settings_path).exists();

        if !settings_exists {
            #[cfg(debug_assertions)]
            println!("設定データベースファイルが存在しません。新規作成します: {settings_path}");
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

        // マイグレーション実行
        Self::run_migrations(&settings_pool).await?;

        Ok(DatabaseManager { settings_pool })
    }

    async fn run_migrations(settings_pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
        // 設定用データベースのマイグレーション
        let settings_migrator = Migrator::new(Path::new("./settings_migrations")).await?;
        settings_migrator.run(settings_pool).await?;
        #[cfg(debug_assertions)]
        println!("設定データベースのマイグレーションが完了しました");

        Ok(())
    }

    pub fn get_settings_pool(&self) -> &SqlitePool {
        &self.settings_pool
    }
}

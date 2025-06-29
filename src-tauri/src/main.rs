// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Sqlite, SqlitePool};
use std::env;
use std::path::Path;
use std::str::FromStr;


mod database;
mod file_manager;
mod search;
mod watcher;

#[tokio::main]
async fn main() {
    // データベースファイルパスを設定
    let (database_url, db_file_path) = if cfg!(debug_assertions) {
        // 開発モード: プロジェクトルートに配置
        (
            "sqlite:./clerica.db".to_string(),
            "./clerica.db".to_string(),
        )
    } else {
        // 本番モード: ホームディレクトリに配置
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let db_path = format!("{}/clerica.db", home);
        (format!("sqlite:{}", db_path), db_path)
    };

    // データベースファイルが存在しない場合の処理
    let db_exists = Path::new(&db_file_path).exists();
    if !db_exists {
        println!(
            "データベースファイルが存在しません。新規作成します: {}",
            db_file_path
        );
    }

    let ops = SqliteConnectOptions::from_str(&database_url).unwrap().create_if_missing(true); // データベースが存在しない場合は自動的に作成

    // SQLiteプールを作成（ファイルが存在しない場合は自動的に作成される）
    let pool = SqlitePool::connect_with(ops)
        .await
        .expect("Failed to connect to database");

    // マイグレーション実行
    println!("マイグレーションを実行しています...");
    if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
        eprintln!("マイグレーション実行エラー: {}", e);
        std::process::exit(1);
    }
    println!("マイグレーションが完了しました。");

    // データベース初期化（テーブル作成など）
    if let Err(e) = database::init_database(&pool).await {
        eprintln!("データベース初期化エラー: {}", e);
        std::process::exit(1);
    }

    if !db_exists {
        println!("新しいデータベースが作成され、初期化が完了しました。");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .manage(pool)
        .setup(|_app| Ok(()))
        .invoke_handler(tauri::generate_handler![
            file_manager::add_directory,
            file_manager::remove_directory,
            file_manager::get_files,
            file_manager::get_directories,
            file_manager::get_files_by_directory,
            file_manager::get_file_info,
            file_manager::update_file_tags,
            file_manager::delete_file,
            file_manager::move_file,
            search::search_files,
            search::get_tags,
            search::create_tag,
            search::delete_tag,
            watcher::start_watching,
            watcher::stop_watching,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

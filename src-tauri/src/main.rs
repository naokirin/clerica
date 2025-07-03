// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::env;
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use watcher::FileWatcher;


mod database;
mod file_manager;
mod search;
mod watcher;
mod custom_metadata;
mod exif_config;
mod exif_constants;
mod thumbnail;

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
        let db_path = format!("{home}/clerica.db");
        (format!("sqlite:{db_path}"), db_path)
    };

    // データベースファイルが存在しない場合の処理
    let db_exists = Path::new(&db_file_path).exists();
    if !db_exists {
        println!(
            "データベースファイルが存在しません。新規作成します: {db_file_path}"
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
        eprintln!("マイグレーション実行エラー: {e}");
        std::process::exit(1);
    }
    println!("マイグレーションが完了しました。");

    // EXIF設定の初期化
    if let Err(e) = exif_config::initialize_exif_config() {
        eprintln!("EXIF設定の初期化エラー: {e}");
        std::process::exit(1);
    }
    println!("EXIF設定が初期化されました。");

    // データベース初期化（テーブル作成など）
    let db = database::Database;
    if let Err(e) = database::DatabaseTrait::init_database(&db, &pool).await {
        eprintln!("データベース初期化エラー: {e}");
        std::process::exit(1);
    }

    if !db_exists {
        println!("新しいデータベースが作成され、初期化が完了しました。");
    }

    // ファイル監視の初期化
    let file_watcher = match FileWatcher::new(Arc::new(pool.clone())) {
        Ok(watcher) => Arc::new(Mutex::new(watcher)),
        Err(e) => {
            eprintln!("ファイル監視の初期化エラー: {e}");
            std::process::exit(1);
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .manage(pool)
        .manage(file_watcher)
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
            file_manager::rescan_directory,
            file_manager::open_file,
            file_manager::reveal_in_finder,
            search::search_files,
            search::get_tags,
            search::create_tag,
            search::delete_tag,
            watcher::start_watching,
            watcher::stop_watching,
            custom_metadata::create_custom_metadata_key,
            custom_metadata::get_custom_metadata_keys,
            custom_metadata::update_custom_metadata_key,
            custom_metadata::delete_custom_metadata_key,
            custom_metadata::get_custom_metadata_key_by_name,
            custom_metadata::set_custom_metadata_value,
            custom_metadata::get_custom_metadata_values_by_file,
            custom_metadata::get_custom_metadata_value,
            custom_metadata::delete_custom_metadata_value,
            exif_config::get_exif_config_data,
            thumbnail::generate_video_thumbnail,
            thumbnail::cleanup_thumbnail_cache,
            thumbnail::get_thumbnail_cache_size,
            thumbnail::extract_audio_album_art,
            thumbnail::generate_archive_thumbnail,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

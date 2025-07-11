// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use database::DatabaseTrait;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::env;
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use watcher::FileWatcher;

#[derive(Clone)]
pub struct DbPools {
    pub write: SqlitePool,
    pub read: SqlitePool,
}

mod custom_metadata;
mod database;
mod exif_config;
mod exif_constants;
mod file_manager;
mod search;
mod settings;
mod thumbnail;
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
        let db_path = format!("{home}/clerica.db");
        (format!("sqlite:{db_path}"), db_path)
    };

    // データベースファイルが存在しない場合の処理
    let db_exists = Path::new(&db_file_path).exists();
    if !db_exists {
        println!("データベースファイルが存在しません。新規作成します: {db_file_path}");
    }

    // 書き込み用プールの設定
    let write_ops = SqliteConnectOptions::from_str(&database_url)
        .unwrap()
        .create_if_missing(true) // データベースが存在しない場合は自動的に作成
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal) // Walモードを有効化
        .synchronous(sqlx::sqlite::SqliteSynchronous::Off) // 同期をオフにしてパフォーマンス向上
        .busy_timeout(std::time::Duration::from_secs(60)) // ロックタイムアウトを60秒に設定
        .pragma("cache_size", "10000") // キャッシュサイズを10MB（デフォルトの約5倍）に設定
        .pragma("temp_store", "memory") // 一時テーブルをメモリに保存
        .pragma("foreign_keys", "on"); // 外部キー制約を有効化

    // 読み取り用プールの設定（読み取り専用）
    let read_ops = SqliteConnectOptions::from_str(&database_url)
        .unwrap()
        .create_if_missing(false) // 読み取り専用では作成しない
        .read_only(true) // 読み取り専用モード
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal) // Walモードを有効化
        .synchronous(sqlx::sqlite::SqliteSynchronous::Off) // 同期をオフにしてパフォーマンス向上
        .busy_timeout(std::time::Duration::from_secs(30)) // 読み取り専用は短めのタイムアウト
        .pragma("cache_size", "15000") // 読み取り専用は大きなキャッシュ
        .pragma("temp_store", "memory") // 一時テーブルをメモリに保存
        .pragma("foreign_keys", "on"); // 外部キー制約を有効化

    // 書き込み用プールを作成（最大5接続）
    let write_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .connect_with(write_ops)
        .await
        .expect("Failed to connect to write database");

    // 読み取り用プールを作成（最大20接続）
    let read_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(20)
        .min_connections(3)
        .connect_with(read_ops)
        .await
        .expect("Failed to connect to read database");

    let pools = DbPools {
        write: write_pool,
        read: read_pool,
    };

    // マイグレーション実行（書き込み用プールを使用）
    println!("マイグレーションを実行しています...");
    if let Err(e) = sqlx::migrate!("./migrations").run(&pools.write).await {
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
    if let Err(e) = database::DatabaseTrait::init_database(&db, &pools.write).await {
        eprintln!("データベース初期化エラー: {e}");
        std::process::exit(1);
    }

    if !db_exists {
        println!("新しいデータベースが作成され、初期化が完了しました。");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .manage(pools.clone())
        .setup(move |app| {
            // ファイル監視の初期化（setup内でAppHandleが取得可能）
            let app_handle = app.handle().clone();
            let file_watcher = match FileWatcher::new(Arc::new(pools.clone()), Some(app_handle)) {
                Ok(watcher) => Arc::new(Mutex::new(watcher)),
                Err(e) => {
                    eprintln!("ファイル監視の初期化エラー: {e}");
                    std::process::exit(1);
                }
            };

            // 既存のディレクトリの監視を開始
            let watcher_clone = Arc::clone(&file_watcher);
            let pools_clone = pools.clone();
            tauri::async_runtime::spawn(async move {
                let db = database::Database;
                match db.get_directories(&pools_clone.read).await {
                    Ok(directories) => {
                        let mut watcher_guard = watcher_clone.lock().unwrap();
                        for directory in directories {
                            if let Err(e) =
                                watcher_guard.watch_directory(&directory.id, &directory.path)
                            {
                                eprintln!("ディレクトリ監視開始エラー: {} ({})", e, directory.path);
                            } else {
                                println!("ディレクトリの監視を開始しました: {}", directory.path);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("既存ディレクトリの取得エラー: {e}");
                    }
                }
            });

            app.manage(file_watcher);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            file_manager::add_directory,
            file_manager::remove_directory,
            file_manager::get_files,
            file_manager::get_files_paginated,
            file_manager::get_files_with_tags,
            file_manager::get_directories,
            file_manager::get_files_by_directory,
            file_manager::get_files_by_directory_paginated,
            file_manager::get_files_by_directory_with_tags,
            file_manager::count_files,
            file_manager::count_files_by_directory,
            file_manager::count_files_by_category,
            file_manager::get_files_paginated_with_category,
            file_manager::get_files_by_directory_paginated_with_category,
            file_manager::count_files_with_category,
            file_manager::count_files_by_directory_with_category,
            file_manager::get_file_info,
            file_manager::get_file_tags,
            file_manager::update_file_tags,
            file_manager::delete_file,
            file_manager::move_file,
            file_manager::rescan_directory,
            file_manager::open_file,
            file_manager::reveal_in_finder,
            search::search_files,
            search::search_files_paginated,
            search::get_tags,
            search::get_top_tags,
            search::search_tags_by_name,
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
            settings::get_settings,
            settings::update_setting_bool_cmd,
            settings::update_setting_int_cmd,
            settings::update_setting_float_cmd,
            settings::update_setting_string_cmd,
            settings::get_language_setting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use database::DatabaseTrait;
use database_manager::DatabaseManager;
// use shelf_manager::ShelfManager; // Imported via pub use below
use std::sync::{Arc, Mutex};
use tauri::Manager;
use watcher::FileWatcher;

mod custom_metadata;
mod database;
mod database_manager;
mod exif_config;
mod exif_constants;
mod exclusion_patterns;
mod file_manager;
mod shelf_commands;
mod shelf_manager;
mod search;
mod settings;
mod thumbnail;
mod watcher;

// Public re-exports
pub use shelf_manager::{Shelf, ShelfManager};

#[tokio::main]
async fn main() {
    // 分割されたデータベースマネージャを初期化
    let db_manager = match DatabaseManager::new().await {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("データベースの初期化に失敗しました: {e}");
            std::process::exit(1);
        }
    };

    // シェルフマネージャを初期化
    let shelf_manager = match ShelfManager::new(db_manager.get_settings_pool().clone()).await {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("シェルフマネージャの初期化に失敗しました: {e}");
            std::process::exit(1);
        }
    };

    // データベース初期化（設定のみ - データベースは各シェルフで管理）
    let db = database::Database;
    match shelf_manager.get_active_data_pool() {
        Ok(active_pool) => {
            if let Err(e) = db.init_database(&active_pool, db_manager.get_settings_pool()).await {
                eprintln!("データベース初期化エラー: {e}");
                std::process::exit(1);
            }
            println!("マイグレーションが完了しました。");
        }
        Err(e) => {
            eprintln!("アクティブデータプールの取得に失敗しました: {e}");
            eprintln!("データベースが正常に初期化されていない可能性があります。");
            std::process::exit(1);
        }
    }

    // EXIF設定の初期化
    if let Err(e) = exif_config::initialize_exif_config() {
        eprintln!("EXIF設定の初期化エラー: {e}");
        std::process::exit(1);
    }
    println!("EXIF設定が初期化されました。");

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .manage(db_manager.clone())
        .manage(shelf_manager.clone())
        .setup(move |app| {
            // ファイル監視の初期化（setup内でAppHandleが取得可能）
            let app_handle = app.handle().clone();
            let file_watcher = match FileWatcher::new(Arc::new(shelf_manager.clone()), Some(app_handle)) {
                Ok(watcher) => Arc::new(Mutex::new(watcher)),
                Err(e) => {
                    eprintln!("ファイル監視の初期化エラー: {e}");
                    std::process::exit(1);
                }
            };

            // 既存のディレクトリの監視を開始
            let watcher_clone = Arc::clone(&file_watcher);
            let shelf_manager_clone = shelf_manager.clone();
            tauri::async_runtime::spawn(async move {
                let db = database::Database;
                match shelf_manager_clone.get_active_data_pool() {
                    Ok(active_pool) => {
                        match db.get_directories(&active_pool).await {
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
                    }
                    Err(e) => {
                        eprintln!("アクティブデータプールの取得エラー: {e}");
                    }
                }
            });

            app.manage(file_watcher);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            file_manager::directories::add_directory,
            file_manager::directories::remove_directory,
            file_manager::files::get_files,
            file_manager::files::get_files_paginated,
            file_manager::files::get_files_with_tags,
            file_manager::directories::get_directories,
            file_manager::files::get_files_by_directory,
            file_manager::files::get_files_by_directory_paginated,
            file_manager::files::get_files_by_directory_with_tags,
            file_manager::files::count_files,
            file_manager::files::count_files_by_directory,
            file_manager::files::count_files_by_category,
            file_manager::files::get_files_paginated_with_category,
            file_manager::files::get_files_by_directory_paginated_with_category,
            file_manager::files::count_files_with_category,
            file_manager::files::count_files_by_directory_with_category,
            file_manager::files::get_file_info,
            file_manager::tags::get_file_tags,
            file_manager::tags::update_file_tags,
            file_manager::files::delete_file,
            file_manager::files::delete_files,
            file_manager::files::batch_rename_files,
            file_manager::files::move_file,
            file_manager::directories::rescan_directory,
            file_manager::files::open_file,
            file_manager::files::reveal_in_finder,
            file_manager::files::preview_rename,
            file_manager::files::execute_rename,
            file_manager::files::preview_advanced_batch_rename,
            file_manager::files::execute_advanced_batch_rename,
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
            shelf_commands::get_shelves,
            shelf_commands::get_active_shelf_id,
            shelf_commands::create_shelf,
            shelf_commands::switch_shelf,
            shelf_commands::delete_shelf,
            shelf_commands::update_shelf_name,
            exclusion_patterns::get_exclusion_patterns,
            exclusion_patterns::add_exclusion_pattern,
            exclusion_patterns::delete_exclusion_pattern,
            exclusion_patterns::test_exclusion_pattern,
            exclusion_patterns::validate_exclusion_pattern,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

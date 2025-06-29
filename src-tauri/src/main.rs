// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use sqlx::SqlitePool;

mod database;
mod file_manager;
mod search;
mod watcher;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .setup(|app| {
            let app_handle = app.handle().clone();
            
            // データベース初期化を遅延実行
            tauri::async_runtime::spawn(async move {
                // プラグインが完全に初期化されるまで少し待つ
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                
                if let Some(pool) = app_handle.try_state::<SqlitePool>() {
                    database::init_database(&pool).await;
                }
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            file_manager::add_directory,
            file_manager::remove_directory,
            file_manager::get_files,
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
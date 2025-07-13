use regex::{Regex, RegexSet};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::{Arc, RwLock};
use tauri::State;

use crate::shelf_manager::ShelfManager;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExclusionPattern {
    pub id: i64,
    pub pattern: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExclusionPatternRequest {
    pub pattern: String,
}

/// 除外パターンマネージャー（高速マッチング用）
pub struct ExclusionPatternManager {
    regex_set: Arc<RwLock<Option<RegexSet>>>,
    patterns: Arc<RwLock<Vec<String>>>,
}

impl ExclusionPatternManager {
    pub fn new() -> Self {
        Self {
            regex_set: Arc::new(RwLock::new(None)),
            patterns: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// データベースからパターンを読み込んでキャッシュを更新
    pub async fn refresh_patterns(&self, pool: &SqlitePool) -> Result<(), String> {
        let patterns = get_exclusion_patterns_from_db(pool).await?;
        let pattern_strings: Vec<String> = patterns.into_iter().map(|p| p.pattern).collect();

        // 正規表現セットを作成
        let regex_set = if pattern_strings.is_empty() {
            None
        } else {
            match RegexSet::new(&pattern_strings) {
                Ok(set) => Some(set),
                Err(e) => {
                    eprintln!("正規表現セットの作成エラー: {}", e);
                    return Err(format!("正規表現セットの作成に失敗しました: {}", e));
                }
            }
        };

        // キャッシュを更新
        *self.regex_set.write().unwrap() = regex_set;
        *self.patterns.write().unwrap() = pattern_strings;

        Ok(())
    }

    /// ファイルパスが除外パターンにマッチするかチェック
    pub fn should_exclude(&self, file_path: &str) -> bool {
        let regex_set_guard = self.regex_set.read().unwrap();
        if let Some(ref set) = *regex_set_guard {
            set.is_match(file_path)
        } else {
            false
        }
    }

    /// 現在のパターン数を取得
    pub fn pattern_count(&self) -> usize {
        self.patterns.read().unwrap().len()
    }
}

/// データベースから除外パターンを取得
async fn get_exclusion_patterns_from_db(pool: &SqlitePool) -> Result<Vec<ExclusionPattern>, String> {
    let patterns = sqlx::query_as::<_, ExclusionPattern>(
        "SELECT id, pattern, created_at FROM exclusion_patterns ORDER BY id"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("除外パターンの取得に失敗しました: {}", e))?;

    Ok(patterns)
}

/// パターンの正規表現が有効かテスト
fn validate_pattern(pattern: &str) -> Result<(), String> {
    match Regex::new(pattern) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("無効な正規表現です: {}", e)),
    }
}

#[tauri::command]
pub async fn get_exclusion_patterns(
    shelf_manager: State<'_, ShelfManager>,
) -> Result<Vec<ExclusionPattern>, String> {
    let pool = shelf_manager.get_settings_pool();
    get_exclusion_patterns_from_db(pool).await
}

#[tauri::command]
pub async fn add_exclusion_pattern(
    pattern: String,
    shelf_manager: State<'_, ShelfManager>,
) -> Result<(), String> {
    // パターンの妥当性チェック
    validate_pattern(&pattern)?;

    let pool = shelf_manager.get_settings_pool();
    
    // データベースに追加
    sqlx::query("INSERT INTO exclusion_patterns (pattern) VALUES (?)")
        .bind(&pattern)
        .execute(pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE constraint failed") {
                "このパターンは既に存在します".to_string()
            } else {
                format!("除外パターンの追加に失敗しました: {}", e)
            }
        })?;

    Ok(())
}

#[tauri::command]
pub async fn delete_exclusion_pattern(
    id: i64,
    shelf_manager: State<'_, ShelfManager>,
) -> Result<(), String> {
    let pool = shelf_manager.get_settings_pool();
    
    let result = sqlx::query("DELETE FROM exclusion_patterns WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("除外パターンの削除に失敗しました: {}", e))?;

    if result.rows_affected() == 0 {
        return Err("指定されたパターンが見つかりません".to_string());
    }

    Ok(())
}

#[tauri::command]
pub async fn test_exclusion_pattern(
    pattern: String,
    test_path: String,
) -> Result<bool, String> {
    // パターンの妥当性チェック
    validate_pattern(&pattern)?;

    // テストパスにマッチするかチェック
    let regex = Regex::new(&pattern)
        .map_err(|e| format!("正規表現の作成に失敗しました: {}", e))?;
    
    Ok(regex.is_match(&test_path))
}

#[tauri::command]
pub async fn validate_exclusion_pattern(pattern: String) -> Result<bool, String> {
    match validate_pattern(&pattern) {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}
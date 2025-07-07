-- パフォーマンス改善のための追加インデックス

-- ファイル属性用のインデックス
CREATE INDEX IF NOT EXISTS idx_files_size ON files(size);
CREATE INDEX IF NOT EXISTS idx_files_file_type ON files(file_type);
CREATE INDEX IF NOT EXISTS idx_files_is_directory ON files(is_directory);
CREATE INDEX IF NOT EXISTS idx_files_created_at ON files(created_at);
CREATE INDEX IF NOT EXISTS idx_files_last_accessed ON files(last_accessed);

-- ファイル検索用の複合インデックス
CREATE INDEX IF NOT EXISTS idx_files_directory_type ON files(directory_id, is_directory);
CREATE INDEX IF NOT EXISTS idx_files_directory_modified ON files(directory_id, modified_at DESC);
CREATE INDEX IF NOT EXISTS idx_files_type_modified ON files(file_type, modified_at DESC);

-- 検索クエリ最適化用の複合インデックス
CREATE INDEX IF NOT EXISTS idx_files_name_directory ON files(name, directory_id);
CREATE INDEX IF NOT EXISTS idx_files_search_composite ON files(directory_id, is_directory, modified_at DESC);

-- カスタムメタデータ検索用のインデックス
CREATE INDEX IF NOT EXISTS idx_custom_metadata_values_file_key ON custom_metadata_values(file_id, key_id);
CREATE INDEX IF NOT EXISTS idx_custom_metadata_values_key_value ON custom_metadata_values(key_id, value);

-- タグ検索の最適化
CREATE INDEX IF NOT EXISTS idx_tags_name_lower ON tags(LOWER(name));

-- ファイル統計用のインデックス（よく使われるクエリ）
CREATE INDEX IF NOT EXISTS idx_files_stats ON files(directory_id, is_directory, size);
-- ファイルのメタデータ（EXIF、音声情報等）をJSON形式で保存するカラムを追加
ALTER TABLE files ADD COLUMN metadata TEXT DEFAULT '{}';

-- メタデータ検索用のインデックスを作成（JSON_EXTRACT使用）
CREATE INDEX IF NOT EXISTS idx_files_metadata ON files(metadata);
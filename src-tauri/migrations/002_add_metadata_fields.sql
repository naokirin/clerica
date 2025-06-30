-- ファイルメタデータの拡張
-- 既存のfilesテーブルにメタデータフィールドを追加
-- 注意: 既存のテーブルには既にこれらのフィールドが存在するため、
-- このマイグレーションは冪等性を保つために IF NOT EXISTS を使用

-- ファイルサイズ（bytes）
ALTER TABLE files ADD COLUMN file_size INTEGER;

-- MIME タイプ
ALTER TABLE files ADD COLUMN mime_type TEXT;

-- ファイルパーミッション（8進数）
ALTER TABLE files ADD COLUMN permissions TEXT;

-- ファイルオーナー（UID）
ALTER TABLE files ADD COLUMN owner_uid INTEGER;

-- ファイルグループ（GID）
ALTER TABLE files ADD COLUMN group_gid INTEGER;

-- ハードリンク数
ALTER TABLE files ADD COLUMN hard_links INTEGER;

-- デバイスID
ALTER TABLE files ADD COLUMN device_id INTEGER;

-- 既存のフィールドのインデックスを追加
CREATE INDEX IF NOT EXISTS idx_files_file_size ON files(file_size);
CREATE INDEX IF NOT EXISTS idx_files_mime_type ON files(mime_type);
CREATE INDEX IF NOT EXISTS idx_files_created_at ON files(created_at);
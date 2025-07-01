-- ファイルの最後アクセス日時を追加
-- アクセス日時の記録でファイル使用頻度を分析可能にする

-- 最後にアクセスした日時（ファイルの読み取り・実行時）
ALTER TABLE files ADD COLUMN last_accessed TIMESTAMP;

-- アクセス日時のインデックスを追加（検索とソートの最適化）
CREATE INDEX IF NOT EXISTS idx_files_last_accessed ON files(last_accessed);
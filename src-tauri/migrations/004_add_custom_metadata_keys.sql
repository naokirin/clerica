-- カスタムメタデータキーを管理するテーブルを作成
CREATE TABLE IF NOT EXISTS custom_metadata_keys (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    data_type TEXT NOT NULL CHECK (data_type IN ('text', 'number', 'date', 'boolean', 'json')),
    description TEXT,
    is_required BOOLEAN NOT NULL DEFAULT FALSE,
    default_value TEXT,
    validation_pattern TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- カスタムメタデータの値を管理するテーブルを作成
CREATE TABLE IF NOT EXISTS custom_metadata_values (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    key_id TEXT NOT NULL,
    value TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (key_id) REFERENCES custom_metadata_keys(id) ON DELETE CASCADE,
    UNIQUE(file_id, key_id)
);

-- インデックスを追加してパフォーマンスを向上
CREATE INDEX IF NOT EXISTS idx_custom_metadata_keys_name ON custom_metadata_keys(name);
CREATE INDEX IF NOT EXISTS idx_custom_metadata_values_file_id ON custom_metadata_values(file_id);
CREATE INDEX IF NOT EXISTS idx_custom_metadata_values_key_id ON custom_metadata_values(key_id);
CREATE INDEX IF NOT EXISTS idx_custom_metadata_values_value ON custom_metadata_values(value);

-- updated_at自動更新のトリガーを作成
CREATE TRIGGER IF NOT EXISTS update_custom_metadata_keys_updated_at
    AFTER UPDATE ON custom_metadata_keys
    FOR EACH ROW
BEGIN
    UPDATE custom_metadata_keys SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_custom_metadata_values_updated_at
    AFTER UPDATE ON custom_metadata_values
    FOR EACH ROW
BEGIN
    UPDATE custom_metadata_values SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
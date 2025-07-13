-- シェルフ管理テーブルの作成

-- シェルフテーブル
CREATE TABLE IF NOT EXISTS shelves (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- アクティブシェルフ設定テーブル
CREATE TABLE IF NOT EXISTS active_shelf (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    shelf_id TEXT NOT NULL
);
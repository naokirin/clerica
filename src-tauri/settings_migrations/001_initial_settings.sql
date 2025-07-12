-- Settings database initial migration
-- This database contains application configuration and metadata definitions

-- Settings table for application configuration
CREATE TABLE settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Custom metadata keys define the structure of user-defined metadata
CREATE TABLE custom_metadata_keys (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    data_type TEXT NOT NULL,
    description TEXT,
    is_required BOOLEAN NOT NULL DEFAULT 0,
    default_value TEXT,
    validation_pattern TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Insert default settings
INSERT INTO settings (key, value) VALUES ('show_hidden_files', 'false');
INSERT INTO settings (key, value) VALUES ('files_per_page', '50');
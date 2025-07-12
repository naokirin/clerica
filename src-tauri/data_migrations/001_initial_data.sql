-- Data database initial migration
-- This database contains user data: files, directories, tags, and custom metadata values

-- Directories table
CREATE TABLE directories (
    id TEXT PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Files table
CREATE TABLE files (
    id TEXT PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    directory_id TEXT NOT NULL,
    size INTEGER NOT NULL,
    file_type TEXT,
    created_at TIMESTAMP,
    modified_at TIMESTAMP,
    birth_time TIMESTAMP,
    inode INTEGER,
    is_directory BOOLEAN NOT NULL DEFAULT 0,
    created_at_db TIMESTAMP NOT NULL,
    updated_at_db TIMESTAMP NOT NULL,
    file_size INTEGER,
    mime_type TEXT,
    permissions TEXT,
    owner_uid INTEGER,
    group_gid INTEGER,
    hard_links INTEGER,
    device_id INTEGER,
    last_accessed TIMESTAMP,
    metadata TEXT DEFAULT '{}',
    FOREIGN KEY (directory_id) REFERENCES directories (id) ON DELETE CASCADE
);

-- Tags table
CREATE TABLE tags (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    color TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL
);

-- File-tags relationship table
CREATE TABLE file_tags (
    file_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    PRIMARY KEY (file_id, tag_id),
    FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);

-- Custom metadata values (links to keys in settings database)
CREATE TABLE custom_metadata_values (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    key_id TEXT NOT NULL, -- References custom_metadata_keys.id in settings database
    value TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE
);
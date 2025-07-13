-- Create exclusion patterns table for regex-based file filtering
CREATE TABLE exclusion_patterns (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pattern TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Add some default exclusion patterns for common files to ignore
INSERT INTO exclusion_patterns (pattern) VALUES 
    ('\.DS_Store$'),
    ('\.log$'),
    ('\.tmp$'),
    ('/node_modules/'),
    ('/\.git/'),
    ('/target/'),
    ('/build/'),
    ('/dist/');
-- Performance indexes for data database

-- Indexes for files table
CREATE INDEX idx_files_directory_id ON files(directory_id);
CREATE INDEX idx_files_name ON files(name);
CREATE INDEX idx_files_path ON files(path);
CREATE INDEX idx_files_modified_at ON files(modified_at);
CREATE INDEX idx_files_created_at ON files(created_at);
CREATE INDEX idx_files_size ON files(size);
CREATE INDEX idx_files_file_type ON files(file_type);
CREATE INDEX idx_files_is_directory ON files(is_directory);
CREATE INDEX idx_files_inode ON files(inode);
CREATE INDEX idx_files_device_id ON files(device_id);

-- Indexes for directories table
CREATE INDEX idx_directories_path ON directories(path);
CREATE INDEX idx_directories_name ON directories(name);

-- Indexes for tags table
CREATE INDEX idx_tags_name ON tags(name);

-- Indexes for file_tags relationship table
CREATE INDEX idx_file_tags_file_id ON file_tags(file_id);
CREATE INDEX idx_file_tags_tag_id ON file_tags(tag_id);

-- Indexes for custom_metadata_values table
CREATE INDEX idx_custom_metadata_values_file_id ON custom_metadata_values(file_id);
CREATE INDEX idx_custom_metadata_values_key_id ON custom_metadata_values(key_id);
CREATE INDEX idx_custom_metadata_values_file_key ON custom_metadata_values(file_id, key_id);
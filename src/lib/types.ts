export interface Directory {
  id: string;
  path: string;
  name: string;
  created_at: string;
  updated_at: string;
}

export interface File {
  id: string;
  path: string;
  name: string;
  directory_id: string;
  size: number;
  file_type: string | null;
  created_at: string | null;
  modified_at: string | null;
  birth_time: string | null;
  inode: number | null;
  is_directory: boolean;
  created_at_db: string;
  updated_at_db: string;
  file_size: number | null;
  mime_type: string | null;
  permissions: string | null;
  owner_uid: number | null;
  group_gid: number | null;
  hard_links: number | null;
  device_id: number | null;
  last_accessed: string | null;
}

export interface Tag {
  id: string;
  name: string;
  color: string;
  created_at: string;
}

export interface SearchResult {
  file: File;
  tags: Tag[];
  score: number;
}

export type FileCategory = "all" | "image" | "audio" | "video" | "document" | "archive" | "other";

export interface FileCategoryInfo {
  key: FileCategory;
  label: string;
  icon: string;
  mimeTypes: string[];
  extensions: string[];
}

export interface LoadingSteps {
  directories: boolean;
  tags: boolean;
  files: boolean;
}
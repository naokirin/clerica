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
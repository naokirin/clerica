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
  metadata: string | null;
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

export interface PaginatedSearchResult {
  results: SearchResult[];
  total_count: number;
  category_counts: Record<string, number>;
  total_category_counts: Record<string, number>;
}

export interface FileWithTags {
  file: File;
  tags: Tag[];
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

export type CustomMetadataDataType = "text" | "number" | "date" | "boolean" | "json";

export interface CustomMetadataKey {
  id: string;
  name: string;
  display_name: string;
  data_type: CustomMetadataDataType;
  description: string | null;
  is_required: boolean;
  default_value: string | null;
  validation_pattern: string | null;
  created_at: string;
  updated_at: string;
}

export interface CustomMetadataValue {
  id: string;
  file_id: string;
  key_id: string;
  value: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateCustomMetadataKeyRequest {
  name: string;
  display_name: string;
  data_type: CustomMetadataDataType;
  description: string | null;
  is_required: boolean;
  default_value: string | null;
  validation_pattern: string | null;
}

export interface UpdateCustomMetadataKeyRequest {
  id: string;
  display_name: string;
  data_type: CustomMetadataDataType;
  description: string | null;
  is_required: boolean;
  default_value: string | null;
  validation_pattern: string | null;
}

export interface SetCustomMetadataValueRequest {
  file_id: string;
  key_id: string;
  value: string | null;
}

export interface CustomMetadataValueWithKey {
  value: CustomMetadataValue;
  key: CustomMetadataKey;
}

export interface MetadataSearchFilter {
  keyId: string;
  keyName: string;
  displayName: string;
  dataType: CustomMetadataDataType;
  value: string;
  operator: 'equals' | 'contains' | 'greater_than' | 'less_than' | 'not_equals';
}

export type MetadataSearchLogic = 'AND' | 'OR';

export interface MetadataSearchOptions {
  filters: MetadataSearchFilter[];
  logic: MetadataSearchLogic;
}

export type SortField = "name" | "size" | "created_at" | "modified_at" | "last_accessed" | "file_type";

export type SortOrder = "asc" | "desc";

export interface SortOptions {
  field: SortField;
  order: SortOrder;
}
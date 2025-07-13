import { invoke } from "@tauri-apps/api/core";
import type { File, FileWithTags, SortOptions, Tag, FileCategory } from "../types";

export async function getFiles(sortOptions?: SortOptions): Promise<File[]> {
  return await invoke("get_files", {
    sortField: sortOptions?.field || "modified_at",
    sortOrder: sortOptions?.order || "desc"
  });
}

export async function getFilesWithTags(sortOptions?: SortOptions): Promise<FileWithTags[]> {
  return await invoke("get_files_with_tags", {
    sortField: sortOptions?.field || "modified_at",
    sortOrder: sortOptions?.order || "desc"
  });
}

export async function getFilesByDirectory(directoryId: string, sortOptions?: SortOptions): Promise<File[]> {
  return await invoke("get_files_by_directory", {
    directoryId,
    sortField: sortOptions?.field || "modified_at",
    sortOrder: sortOptions?.order || "desc"
  });
}

export async function getFilesByDirectoryWithTags(directoryId: string, sortOptions?: SortOptions): Promise<FileWithTags[]> {
  return await invoke("get_files_by_directory_with_tags", {
    directoryId,
    sortField: sortOptions?.field || "modified_at",
    sortOrder: sortOptions?.order || "desc"
  });
}

// ページネーション対応API
export async function getFilesPaginated(
  sortOptions?: SortOptions,
  limit: number = 20,
  offset: number = 0
): Promise<File[]> {
  return await invoke("get_files_paginated", {
    sortField: sortOptions?.field || "modified_at",
    sortOrder: sortOptions?.order || "desc",
    limit,
    offset
  });
}

export async function getFilesByDirectoryPaginated(
  directoryId: string,
  sortOptions?: SortOptions,
  limit: number = 20,
  offset: number = 0
): Promise<File[]> {
  return await invoke("get_files_by_directory_paginated", {
    directoryId,
    sortField: sortOptions?.field || "modified_at",
    sortOrder: sortOptions?.order || "desc",
    limit,
    offset
  });
}

export async function countFiles(): Promise<number> {
  return await invoke("count_files");
}

export async function countFilesByDirectory(directoryId: string): Promise<number> {
  return await invoke("count_files_by_directory", { directoryId });
}

export async function openFile(filePath: string): Promise<void> {
  return await invoke("open_file", { filePath });
}

export async function revealInFinder(filePath: string): Promise<void> {
  return await invoke("reveal_in_finder", { filePath });
}

export async function deleteFile(filePath: string): Promise<void> {
  return await invoke("delete_file", { filePath });
}

export interface DeleteResult {
  successful_files: string[];
  failed_files: Array<[string, string]>; // [file_path, error_message]
}

export async function deleteFiles(fileIds: number[]): Promise<DeleteResult> {
  return await invoke("delete_files", { fileIds });
}

export interface BatchRenameOperation {
  old_path: string;
  new_name: string;
}

export interface BatchRenameResult {
  successful_files: string[];
  failed_files: Array<[string, string]>; // [file_path, error_message]
}

export async function batchRenameFiles(operations: BatchRenameOperation[]): Promise<BatchRenameResult> {
  return await invoke("batch_rename_files", { operations });
}

export async function generateVideoThumbnail(filePath: string): Promise<string> {
  return await invoke("generate_video_thumbnail", { filePath });
}

export async function extractAudioAlbumArt(filePath: string): Promise<string> {
  return await invoke("extract_audio_album_art", { filePath });
}

export async function generateArchiveThumbnail(filePath: string): Promise<string> {
  return await invoke("generate_archive_thumbnail", { filePath });
}

export async function updateFileTags(fileId: string, tagIds: string[]): Promise<void> {
  return await invoke("update_file_tags", { fileId: fileId, tagIds: tagIds });
}

export async function getFileTags(fileId: string): Promise<Tag[]> {
  return await invoke("get_file_tags", { fileId: fileId });
}

export async function countFilesByCategory(directoryId: string = "all"): Promise<Record<FileCategory, number>> {
  return await invoke("count_files_by_category", { directoryId });
}

export async function getFilesPaginatedWithCategory(
  category: FileCategory,
  sortOptions?: SortOptions,
  limit: number = 20,
  offset: number = 0
): Promise<File[]> {
  return await invoke("get_files_paginated_with_category", {
    category,
    sortField: sortOptions?.field || "modified_at",
    sortOrder: sortOptions?.order || "desc",
    limit,
    offset
  });
}

export async function getFilesByDirectoryPaginatedWithCategory(
  directoryId: string,
  category: FileCategory,
  sortOptions?: SortOptions,
  limit: number = 20,
  offset: number = 0
): Promise<File[]> {
  return await invoke("get_files_by_directory_paginated_with_category", {
    directoryId,
    category,
    sortField: sortOptions?.field || "modified_at",
    sortOrder: sortOptions?.order || "desc",
    limit,
    offset
  });
}

export async function countFilesWithCategory(category: FileCategory): Promise<number> {
  return await invoke("count_files_with_category", { category });
}

export async function countFilesByDirectoryWithCategory(directoryId: string, category: FileCategory): Promise<number> {
  return await invoke("count_files_by_directory_with_category", { directoryId, category });
}

export async function previewRename(
  fileId: string,
  regexPattern: string,
  formatTemplate: string
): Promise<string> {
  return await invoke("preview_rename", { fileId, regexPattern, formatTemplate });
}

export async function executeRename(
  fileId: string,
  regexPattern: string,
  formatTemplate: string
): Promise<string> {
  return await invoke("execute_rename", { fileId, regexPattern, formatTemplate });
}

export async function previewSimpleRename(
  fileId: string,
  newName: string
): Promise<string> {
  // シンプルリネームの場合は、拡張子を保持して新しい名前にする
  // 正規表現APIを使って実装: 全体をキャプチャして新しい名前に置換
  return await invoke("preview_rename", { 
    fileId, 
    regexPattern: "^(.+?)(\\.([^.]+))?$", 
    formatTemplate: `${newName}$2` 
  });
}

export async function executeSimpleRename(
  fileId: string,
  newName: string
): Promise<string> {
  // シンプルリネームの場合は、拡張子を保持して新しい名前にする
  // 正規表現APIを使って実装: 全体をキャプチャして新しい名前に置換
  return await invoke("execute_rename", { 
    fileId, 
    regexPattern: "^(.+?)(\\.([^.]+))?$", 
    formatTemplate: `${newName}$2` 
  });
}

// バッチリネーム用の高度なプレビュー・実行機能
export interface AdvancedBatchRenameOperation {
  file_id: string;
  find_pattern: string;
  replace_pattern: string;
  use_regex: boolean;
  use_template: boolean;
}

export interface AdvancedBatchRenamePreview {
  file_id: string;
  old_name: string;
  new_name: string;
  error?: string;
}

export async function previewAdvancedBatchRename(
  operations: AdvancedBatchRenameOperation[]
): Promise<AdvancedBatchRenamePreview[]> {
  return await invoke("preview_advanced_batch_rename", { operations });
}

export async function executeAdvancedBatchRename(
  operations: AdvancedBatchRenameOperation[]
): Promise<BatchRenameResult> {
  return await invoke("execute_advanced_batch_rename", { operations });
}
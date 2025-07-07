import { invoke } from "@tauri-apps/api/core";
import type { File, FileWithTags, SortOptions, Tag } from "../types";

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
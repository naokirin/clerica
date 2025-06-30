import { invoke } from "@tauri-apps/api/core";
import type { Directory, File, Tag, SearchResult } from "./types";

export class ClericaAPI {
  // ディレクトリ関連
  static async addDirectory(path: string, name: string): Promise<Directory> {
    return invoke('add_directory', { path, name });
  }

  static async removeDirectory(id: string): Promise<void> {
    return invoke('remove_directory', { id });
  }

  static async getDirectories(): Promise<Directory[]> {
    return invoke('get_directories');
  }

  // ファイル関連
  static async getFiles(): Promise<File[]> {
    return invoke('get_files');
  }

  static async getFilesByDirectory(directoryId: string): Promise<File[]> {
    return invoke('get_files_by_directory', { directoryId });
  }

  static async getFileInfo(fileId: string): Promise<File> {
    return invoke('get_file_info', { fileId });
  }

  static async updateFileTags(fileId: string, tagIds: string[]): Promise<void> {
    return invoke('update_file_tags', { fileId, tagIds });
  }

  static async deleteFile(fileId: string): Promise<void> {
    return invoke('delete_file', { fileId });
  }

  static async moveFile(fileId: string, newPath: string): Promise<void> {
    return invoke('move_file', { fileId, newPath });
  }

  // 検索関連
  static async searchFiles(
    query: string, 
    tagIds?: string[],
    sortBy?: string
  ): Promise<SearchResult[]> {
    return invoke('search_files', { 
      query, 
      tag_ids: tagIds,
      sort_by: sortBy 
    });
  }

  // タグ関連
  static async getTags(): Promise<Tag[]> {
    return invoke('get_tags');
  }

  static async createTag(name: string, color: string): Promise<Tag> {
    return invoke('create_tag', { name, color });
  }

  static async deleteTag(id: string): Promise<void> {
    return invoke('delete_tag', { id });
  }

  // ファイル監視関連
  static async startWatching(directoryId: string): Promise<void> {
    return invoke('start_watching', { directoryId });
  }

  static async stopWatching(directoryId: string): Promise<void> {
    return invoke('stop_watching', { directoryId });
  }
}
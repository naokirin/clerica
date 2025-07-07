import { writable, type Writable } from 'svelte/store';
import { BaseViewModel } from './BaseViewModel';
import { getDirectories, addDirectory, removeDirectory, rescanDirectory, type DirectoryRemovalResult } from '../api/directories';
import type { Directory } from '../types';

export class DirectoryViewModel extends BaseViewModel {
  private _directories: Writable<Directory[]> = writable([]);
  private _selectedDirectoryId: Writable<string | "all"> = writable("all");

  public readonly directories = this._directories;
  public readonly selectedDirectoryId = this._selectedDirectoryId;

  constructor() {
    super();
    this.loadDirectories();
  }

  public async loadDirectories(): Promise<void> {
    const result = await this.executeAsync(async () => {
      return await getDirectories();
    });

    if (result) {
      this._directories.set(result);
    }
  }

  public async addNewDirectory(path: string, name: string, tagViewModel?: any): Promise<boolean> {
    const result = await this.executeAsync(async () => {
      await addDirectory(path, name);
      return true;
    });

    if (result) {
      await this.loadDirectories();
      
      // 自動タグ付けによって新しいタグが作成された可能性があるため、タグ情報を更新
      if (tagViewModel) {
        await tagViewModel.refreshAllTags();
      }
      
      return true;
    }
    return false;
  }

  public async removeExistingDirectory(id: string, tagViewModel?: any): Promise<DirectoryRemovalResult | null> {
    const result = await this.executeAsync(async () => {
      return await removeDirectory(id);
    });

    if (result) {
      await this.loadDirectories();
      
      // タグが削除された場合、タグ情報を更新
      if (result.deleted_tags_count > 0 && tagViewModel) {
        console.log(`${result.deleted_tags_count}個のタグが削除されました: ${result.deleted_tag_ids.join(', ')}`);
        await tagViewModel.refreshAllTags();
      }
      
      return result;
    }
    return null;
  }

  public async rescanExistingDirectory(directoryId: string, tagViewModel?: any): Promise<boolean> {
    const result = await this.executeAsync(async () => {
      await rescanDirectory(directoryId);
      return true;
    });

    if (result) {
      // 再スキャンによって新しいタグが作成された可能性があるため、タグ情報を更新
      if (tagViewModel) {
        await tagViewModel.refreshAllTags();
      }
      return true;
    }
    return false;
  }

  public selectDirectory(directoryId: string | "all"): void {
    this._selectedDirectoryId.set(directoryId);
  }
}
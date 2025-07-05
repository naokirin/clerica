import { writable, derived, type Writable, type Readable } from 'svelte/store';
import { BaseViewModel } from './BaseViewModel.js';
import { getFiles, getFilesWithTags, getFilesByDirectory, getFilesByDirectoryWithTags, openFile, revealInFinder, deleteFile } from '../api/files.js';
import { getSettings } from '../api/settings.js';
import type { File, FileWithTags, FileCategory, SortOptions } from '../types.js';
import { getFileCategory } from '../utils.js';

export class FileViewModel extends BaseViewModel {
  private _filesWithTags: Writable<FileWithTags[]> = writable([]);
  private _selectedFile: Writable<File | null> = writable(null);
  private _selectedCategory: Writable<FileCategory> = writable("all");
  private _currentPage: Writable<number> = writable(1);
  private _itemsPerPage: Writable<number> = writable(20);
  private _isDeleting: Writable<boolean> = writable(false);
  private _selectedDirectoryId: Writable<string | "all"> = writable("all");
  private _sortOptions: Writable<SortOptions> = writable({ field: "modified_at", order: "desc" });

  // 互換性のためのファイルのみのストア
  public readonly files: Readable<File[]> = derived(
    this._filesWithTags,
    (filesWithTags) => filesWithTags.map(f => f.file)
  );
  
  public readonly filesWithTags = this._filesWithTags;
  public readonly selectedFile = this._selectedFile;
  public readonly selectedCategory = this._selectedCategory;
  public readonly currentPage = this._currentPage;
  public readonly itemsPerPage = this._itemsPerPage;
  public readonly isDeleting = this._isDeleting;
  public readonly selectedDirectoryId = this._selectedDirectoryId;
  public readonly sortOptions = this._sortOptions;

  // 派生ストア
  public readonly categoryCounts: Readable<Record<FileCategory, number>> = derived(
    [this.files, this._selectedCategory],
    ([files, selectedCategory]) => {
      const counts: Record<FileCategory, number> = {
        all: files.length,
        image: 0,
        audio: 0,
        video: 0,
        document: 0,
        archive: 0,
        other: 0,
      };

      files.forEach((file) => {
        const category = getFileCategory(file);
        counts[category]++;
      });

      return counts;
    }
  );

  public readonly filteredFiles: Readable<File[]> = derived(
    [this.files, this._selectedCategory],
    ([files, selectedCategory]) => {
      if (selectedCategory === "all") {
        return files;
      }
      return files.filter((file) => getFileCategory(file) === selectedCategory);
    }
  );

  public readonly filteredFilesWithTags: Readable<FileWithTags[]> = derived(
    [this._filesWithTags, this._selectedCategory],
    ([filesWithTags, selectedCategory]) => {
      if (selectedCategory === "all") {
        return filesWithTags;
      }
      return filesWithTags.filter((fileWithTags) => getFileCategory(fileWithTags.file) === selectedCategory);
    }
  );

  public readonly totalPages: Readable<number> = derived(
    [this.filteredFiles, this._itemsPerPage],
    ([filteredFiles, itemsPerPage]) => Math.ceil(filteredFiles.length / itemsPerPage)
  );

  public readonly paginatedFiles: Readable<File[]> = derived(
    [this.filteredFiles, this._currentPage, this._itemsPerPage],
    ([filteredFiles, currentPage, itemsPerPage]) => {
      const startIndex = (currentPage - 1) * itemsPerPage;
      const endIndex = startIndex + itemsPerPage;
      return filteredFiles.slice(startIndex, endIndex);
    }
  );

  public readonly paginatedFilesWithTags: Readable<FileWithTags[]> = derived(
    [this.filteredFilesWithTags, this._currentPage, this._itemsPerPage],
    ([filteredFilesWithTags, currentPage, itemsPerPage]) => {
      const startIndex = (currentPage - 1) * itemsPerPage;
      const endIndex = startIndex + itemsPerPage;
      return filteredFilesWithTags.slice(startIndex, endIndex);
    }
  );

  constructor() {
    super();
    this.loadSettings();
    this.loadFiles();
  }

  private async loadSettings(): Promise<void> {
    try {
      const settings = await getSettings();
      this._itemsPerPage.set(settings.files_per_page);
    } catch (error) {
      console.error('設定の読み込みに失敗しました:', error);
    }
  }

  public async updateItemsPerPage(): Promise<void> {
    await this.loadSettings();
    this._currentPage.set(1); // ページをリセット
  }

  public async loadFiles(directoryId?: string | "all"): Promise<void> {
    const targetDirectoryId = directoryId || "all";
    
    const result = await this.executeAsync(async () => {
      const currentSortOptions = this.getCurrentSortOptions();
      if (targetDirectoryId === "all") {
        return await getFilesWithTags(currentSortOptions);
      } else {
        return await getFilesByDirectoryWithTags(targetDirectoryId, currentSortOptions);
      }
    });

    if (result) {
      this._filesWithTags.set(result);
    }
  }

  private getCurrentSortOptions(): SortOptions {
    let currentOptions: SortOptions = { field: "modified_at", order: "desc" };
    this._sortOptions.subscribe(options => currentOptions = options)();
    return currentOptions;
  }

  public setSelectedDirectoryId(directoryId: string | "all"): void {
    this._selectedDirectoryId.set(directoryId);
    this.loadFiles(directoryId); // ディレクトリ変更時にファイルを再読み込み
    this._currentPage.set(1); // ページをリセット
  }

  public selectFile(file: File): void {
    this._selectedFile.set(file);
  }

  public closeFileDetails(): void {
    this._selectedFile.set(null);
  }

  public selectCategory(category: FileCategory): void {
    this._selectedCategory.set(category);
    this._currentPage.set(1); // カテゴリ変更時はページをリセット
  }

  public goToPage(page: number): void {
    this._currentPage.set(page);
  }

  public goToPreviousPage(): void {
    this._currentPage.update(page => Math.max(1, page - 1));
  }

  public goToNextPage(): void {
    this._currentPage.update(page => page + 1);
  }

  public goToFirstPage(): void {
    this._currentPage.set(1);
  }

  public goToLastPage(totalPages: number): void {
    this._currentPage.set(totalPages);
  }

  public async openSelectedFile(filePath: string): Promise<boolean> {
    const result = await this.executeAsync(async () => {
      await openFile(filePath);
      return true;
    });

    if (result) {
      await this.loadFiles(); // ファイル情報を再読み込み
      return true;
    }
    return false;
  }

  public async revealFileInFinder(filePath: string): Promise<boolean> {
    const result = await this.executeAsync(async () => {
      await revealInFinder(filePath);
      return true;
    });

    return result !== null;
  }

  public async deleteSelectedFile(filePath: string): Promise<boolean> {
    this._isDeleting.set(true);
    
    const result = await this.executeAsync(async () => {
      await deleteFile(filePath);
      return true;
    });

    if (result) {
      await this.loadFiles();
      this.closeFileDetails();
    }
    
    this._isDeleting.set(false);
    return result !== null;
  }

  public setSortOptions(options: SortOptions): void {
    this._sortOptions.set(options);
    this.loadFiles(); // ソート変更時にファイルを再読み込み
  }
}
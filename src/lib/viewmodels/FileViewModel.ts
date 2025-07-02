import { writable, derived, type Writable, type Readable } from 'svelte/store';
import { BaseViewModel } from './BaseViewModel.js';
import { getFiles, getFilesByDirectory, openFile, revealInFinder, deleteFile } from '../api/files.js';
import type { File, FileCategory } from '../types.js';
import { getFileCategory } from '../utils.js';

export class FileViewModel extends BaseViewModel {
  private _files: Writable<File[]> = writable([]);
  private _selectedFile: Writable<File | null> = writable(null);
  private _selectedCategory: Writable<FileCategory> = writable("all");
  private _currentPage: Writable<number> = writable(1);
  private _itemsPerPage = 25;
  private _isDeleting: Writable<boolean> = writable(false);
  private _selectedDirectoryId: Writable<string | "all"> = writable("all");

  public readonly files = this._files;
  public readonly selectedFile = this._selectedFile;
  public readonly selectedCategory = this._selectedCategory;
  public readonly currentPage = this._currentPage;
  public readonly isDeleting = this._isDeleting;
  public readonly selectedDirectoryId = this._selectedDirectoryId;

  // 派生ストア
  public readonly categoryCounts: Readable<Record<FileCategory, number>> = derived(
    [this._files, this._selectedCategory],
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
    [this._files, this._selectedCategory],
    ([files, selectedCategory]) => {
      if (selectedCategory === "all") {
        return files;
      }
      return files.filter((file) => getFileCategory(file) === selectedCategory);
    }
  );

  public readonly totalPages: Readable<number> = derived(
    this.filteredFiles,
    (filteredFiles) => Math.ceil(filteredFiles.length / this._itemsPerPage)
  );

  public readonly paginatedFiles: Readable<File[]> = derived(
    [this.filteredFiles, this._currentPage],
    ([filteredFiles, currentPage]) => {
      const startIndex = (currentPage - 1) * this._itemsPerPage;
      const endIndex = startIndex + this._itemsPerPage;
      return filteredFiles.slice(startIndex, endIndex);
    }
  );

  constructor() {
    super();
    this.loadFiles();
  }

  public async loadFiles(directoryId?: string | "all"): Promise<void> {
    const targetDirectoryId = directoryId || "all";
    
    const result = await this.executeAsync(async () => {
      if (targetDirectoryId === "all") {
        return await getFiles();
      } else {
        return await getFilesByDirectory(targetDirectoryId);
      }
    });

    if (result) {
      this._files.set(result);
    }
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
}
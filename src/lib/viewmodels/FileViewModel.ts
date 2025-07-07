import { writable, derived, type Writable, type Readable } from 'svelte/store';
import { BaseViewModel } from './BaseViewModel';
import { 
  getFiles, 
  getFilesWithTags, 
  getFilesByDirectory, 
  getFilesByDirectoryWithTags, 
  getFilesPaginated,
  getFilesByDirectoryPaginated,
  getFilesPaginatedWithCategory,
  getFilesByDirectoryPaginatedWithCategory,
  countFiles,
  countFilesByDirectory,
  countFilesByCategory,
  countFilesWithCategory,
  countFilesByDirectoryWithCategory,
  getFileTags,
  openFile, 
  revealInFinder, 
  deleteFile 
} from '../api/files';
import { getSettings } from '../api/settings';
import type { File, FileWithTags, FileCategory, SortOptions } from '../types';
import { getFileCategory } from '../utils';

export class FileViewModel extends BaseViewModel {
  private _filesWithTags: Writable<FileWithTags[]> = writable([]);
  private _selectedFile: Writable<File | null> = writable(null);
  private _selectedCategory: Writable<FileCategory> = writable("all");
  private _currentPage: Writable<number> = writable(1);
  private _itemsPerPage: Writable<number> = writable(20);
  private _totalFiles: Writable<number> = writable(0);
  private _isDeleting: Writable<boolean> = writable(false);
  private _selectedDirectoryId: Writable<string | "all"> = writable("all");
  private _sortOptions: Writable<SortOptions> = writable({ field: "modified_at", order: "desc" });
  private _useServerSidePagination: boolean = true; // サーバーサイドページネーションを有効化
  private _categoryCounts: Writable<Record<FileCategory, number>> = writable({
    all: 0,
    image: 0,
    audio: 0,
    video: 0,
    document: 0,
    archive: 0,
    other: 0,
  });

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
  public readonly totalFiles = this._totalFiles;
  public readonly isDeleting = this._isDeleting;
  public readonly selectedDirectoryId = this._selectedDirectoryId;
  public readonly sortOptions = this._sortOptions;

  public readonly categoryCounts: Readable<Record<FileCategory, number>> = this._categoryCounts;

  // サーバーサイドページネーションでは、サーバーですでにフィルタリング済み
  public readonly filteredFiles: Readable<File[]> = derived(
    [this.files],
    ([files]) => files
  );

  public readonly filteredFilesWithTags: Readable<FileWithTags[]> = derived(
    [this._filesWithTags],
    ([filesWithTags]) => filesWithTags
  );

  public readonly totalPages: Readable<number> = derived(
    [this._totalFiles, this._itemsPerPage],
    ([totalFiles, itemsPerPage]) => Math.ceil(totalFiles / itemsPerPage)
  );

  // サーバーサイドページネーションでは、返されたデータがそのままページングされたデータ
  public readonly paginatedFiles: Readable<File[]> = derived(
    [this.files],
    ([files]) => files
  );

  public readonly paginatedFilesWithTags: Readable<FileWithTags[]> = derived(
    [this._filesWithTags],
    ([filesWithTags]) => filesWithTags
  );

  constructor() {
    super();
    this.loadSettings();
    this.loadFiles();
    this.loadCategoryCounts();
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
    
    if (this._useServerSidePagination) {
      await this.loadFilesPaginated();
    } else {
      // 従来の方法（後方互換性のため残す）
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
        this._totalFiles.set(result.length);
      }
    }
  }

  private async loadFilesPaginated(): Promise<void> {
    const result = await this.executeAsync(async () => {
      const currentSortOptions = this.getCurrentSortOptions();
      const currentPage = this.getCurrentPage();
      const itemsPerPage = this.getCurrentItemsPerPage();
      const offset = (currentPage - 1) * itemsPerPage;
      const selectedDirectoryId = this.getCurrentSelectedDirectoryId();
      const selectedCategory = this.getCurrentSelectedCategory();

      let files: File[];
      let totalCount: number;

      if (selectedCategory === "all") {
        // カテゴリフィルタリングなしの場合
        if (selectedDirectoryId === "all") {
          [files, totalCount] = await Promise.all([
            getFilesPaginated(currentSortOptions, itemsPerPage, offset),
            countFiles()
          ]);
        } else {
          [files, totalCount] = await Promise.all([
            getFilesByDirectoryPaginated(selectedDirectoryId, currentSortOptions, itemsPerPage, offset),
            countFilesByDirectory(selectedDirectoryId)
          ]);
        }
      } else {
        // カテゴリフィルタリングありの場合
        if (selectedDirectoryId === "all") {
          [files, totalCount] = await Promise.all([
            getFilesPaginatedWithCategory(selectedCategory, currentSortOptions, itemsPerPage, offset),
            countFilesWithCategory(selectedCategory)
          ]);
        } else {
          [files, totalCount] = await Promise.all([
            getFilesByDirectoryPaginatedWithCategory(selectedDirectoryId, selectedCategory, currentSortOptions, itemsPerPage, offset),
            countFilesByDirectoryWithCategory(selectedDirectoryId, selectedCategory)
          ]);
        }
      }

      // ファイルにタグ情報を追加
      const filesWithTags: FileWithTags[] = await Promise.all(
        files.map(async (file) => {
          try {
            const tags = await getFileTags(file.id);
            return { file, tags };
          } catch (error) {
            console.warn(`Failed to get tags for file ${file.id}:`, error);
            return { file, tags: [] };
          }
        })
      );

      return { filesWithTags, totalCount };
    });

    if (result) {
      this._filesWithTags.set(result.filesWithTags);
      this._totalFiles.set(result.totalCount);
      // ファイル読み込み後にカテゴリカウントも更新
      this.loadCategoryCounts();
    }
  }

  private getCurrentPage(): number {
    let currentPage = 1;
    this._currentPage.subscribe(page => currentPage = page)();
    return currentPage;
  }

  private getCurrentItemsPerPage(): number {
    let itemsPerPage = 20;
    this._itemsPerPage.subscribe(items => itemsPerPage = items)();
    return itemsPerPage;
  }

  private getCurrentSelectedDirectoryId(): string | "all" {
    let directoryId: string | "all" = "all";
    this._selectedDirectoryId.subscribe(id => directoryId = id)();
    return directoryId;
  }

  private getCurrentSortOptions(): SortOptions {
    let currentOptions: SortOptions = { field: "modified_at", order: "desc" };
    this._sortOptions.subscribe(options => currentOptions = options)();
    return currentOptions;
  }

  private getCurrentSelectedCategory(): FileCategory {
    let selectedCategory: FileCategory = "all";
    this._selectedCategory.subscribe(category => selectedCategory = category)();
    return selectedCategory;
  }

  public setSelectedDirectoryId(directoryId: string | "all"): void {
    this._selectedDirectoryId.set(directoryId);
    this.loadFiles(directoryId); // ディレクトリ変更時にファイルを再読み込み
    this._currentPage.set(1); // ページをリセット
    this.loadCategoryCounts(directoryId); // カテゴリカウントも更新
  }

  public selectFile(file: File): void {
    this._selectedFile.set(file);
  }

  public closeFileDetails(): void {
    this._selectedFile.set(null);
  }

  public async selectCategory(category: FileCategory): Promise<void> {
    this._selectedCategory.set(category);
    this._currentPage.set(1); // カテゴリ変更時はページをリセット
    
    if (this._useServerSidePagination) {
      await this.loadFilesPaginated();
    } else {
      await this.loadFiles();
    }
  }

  public async goToPage(page: number): Promise<void> {
    this._currentPage.set(page);
    
    if (this._useServerSidePagination) {
      await this.loadFilesPaginated();
    }
  }

  public async goToPreviousPage(): Promise<void> {
    const currentPage = this.getCurrentPage();
    if (currentPage > 1) {
      await this.goToPage(currentPage - 1);
    }
  }

  public async goToNextPage(): Promise<void> {
    const currentPage = this.getCurrentPage();
    const totalPages = this.getCurrentTotalPages();
    if (currentPage < totalPages) {
      await this.goToPage(currentPage + 1);
    }
  }

  public async goToFirstPage(): Promise<void> {
    await this.goToPage(1);
  }

  public async goToLastPage(): Promise<void> {
    const totalPages = this.getCurrentTotalPages();
    await this.goToPage(totalPages);
  }

  private getCurrentTotalPages(): number {
    let totalPages = 1;
    this.totalPages.subscribe(pages => totalPages = pages)();
    return totalPages;
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

  public async setSortOptions(options: SortOptions): Promise<void> {
    this._sortOptions.set(options);
    this._currentPage.set(1); // ソート変更時はページをリセット
    if (this._useServerSidePagination) {
      await this.loadFilesPaginated();
    } else {
      await this.loadFiles();
    }
  }

  // カテゴリカウントを読み込む新しいメソッド
  public async loadCategoryCounts(directoryId?: string | "all"): Promise<void> {
    const targetDirectoryId = directoryId || this.getCurrentSelectedDirectoryId();
    const counts = await this.executeAsync(() => countFilesByCategory(targetDirectoryId));
    if (counts) {
      this._categoryCounts.set(counts);
    }
  }

}
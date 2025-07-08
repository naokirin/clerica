import { writable, derived, type Writable, type Readable } from 'svelte/store';
import { BaseViewModel } from './BaseViewModel';
import { searchFiles, searchFilesPaginated } from '../api/search';
import { getSettings } from '../api/settings';
import type { SearchResult, PaginatedSearchResult, FileCategory, MetadataSearchFilter, MetadataSearchLogic, SortOptions } from '../types';
import { getFileCategory } from '../utils';

export class SearchViewModel extends BaseViewModel {
  private _searchQuery: Writable<string> = writable("");
  private _selectedTags: Writable<string[]> = writable([]);
  private _metadataSearchFilters: Writable<MetadataSearchFilter[]> = writable([]);
  private _metadataLogic: Writable<MetadataSearchLogic> = writable("AND");
  private _searchResults: Writable<SearchResult[]> = writable([]);
  private _totalSearchResults: Writable<number> = writable(0);
  private _useServerSidePagination: boolean = true; // サーバーサイドページネーションを有効化
  private _selectedCategory: Writable<FileCategory> = writable("all");
  private _currentPage: Writable<number> = writable(1);
  private _itemsPerPage: Writable<number> = writable(20);
  private _selectedDirectoryId: Writable<string | "all"> = writable("all");
  private _sortOptions: Writable<SortOptions> = writable({ field: "modified_at", order: "desc" });
  private _searchCategoryCounts: Writable<Record<FileCategory, number>> = writable({
    all: 0,
    image: 0,
    audio: 0,
    video: 0,
    document: 0,
    archive: 0,
    other: 0,
  });
  private _totalSearchCategoryCounts: Writable<Record<FileCategory, number>> = writable({
    all: 0,
    image: 0,
    audio: 0,
    video: 0,
    document: 0,
    archive: 0,
    other: 0,
  });

  public readonly searchQuery = this._searchQuery;
  public readonly selectedTags = this._selectedTags;
  public readonly metadataSearchFilters = this._metadataSearchFilters;
  public readonly metadataLogic = this._metadataLogic;
  public readonly searchResults = this._searchResults;
  public readonly totalSearchResults = this._totalSearchResults;
  public readonly selectedCategory = this._selectedCategory;
  public readonly currentPage = this._currentPage;
  public readonly itemsPerPage = this._itemsPerPage;
  public readonly selectedDirectoryId = this._selectedDirectoryId;
  public readonly sortOptions = this._sortOptions;
  public readonly searchCategoryCounts = this._searchCategoryCounts;
  public readonly totalSearchCategoryCounts = this._totalSearchCategoryCounts;

  // サーバーサイドページネーションでは、返されたデータがそのまま表示データ
  public readonly filteredSearchResults: Readable<SearchResult[]> = derived(
    [this._searchResults],
    ([searchResults]) => searchResults
  );

  public readonly searchTotalPages: Readable<number> = derived(
    [this._totalSearchResults, this._itemsPerPage],
    ([totalResults, itemsPerPage]) => Math.ceil(totalResults / itemsPerPage)
  );

  public readonly paginatedSearchResults: Readable<SearchResult[]> = derived(
    [this._searchResults],
    ([searchResults]) => searchResults
  );

  constructor() {
    super();
    this.loadSettings();
    this.loadInitialFiles();
  }

  private async loadSettings(): Promise<void> {
    try {
      const settings = await getSettings();
      this._itemsPerPage.set(settings.files_per_page);
    } catch (error) {
      console.error('設定の読み込みに失敗しました:', error);
    }
  }

  private async loadInitialFiles(): Promise<void> {
    try {
      // 初期状態では空の検索クエリで全ファイルを取得
      // この時点でsearchQueryは空文字列なので、全ファイルが返される
      if (this._useServerSidePagination) {
        const currentPage = this.getCurrentPage();
        const itemsPerPage = this.getCurrentItemsPerPage();
        const offset = (currentPage - 1) * itemsPerPage;
        
        const result = await searchFilesPaginated(
          "", // 空のクエリ
          [], // 空のタグ
          [], // 空のメタデータフィルタ
          "AND",
          "all", // 全ディレクトリ
          { field: "modified_at", order: "desc" },
          itemsPerPage,
          offset,
          "all" // 全カテゴリ
        );
        
        this._searchResults.set(result.results);
        this._totalSearchResults.set(result.total_count);
        
        // カテゴリ別件数を更新（フィルタ適用後）
        const categoryCounts: Record<FileCategory, number> = {
          all: result.category_counts["all"] || 0,
          image: result.category_counts["image"] || 0,
          audio: result.category_counts["audio"] || 0,
          video: result.category_counts["video"] || 0,
          document: result.category_counts["document"] || 0,
          archive: result.category_counts["archive"] || 0,
          other: result.category_counts["other"] || 0,
        };
        this._searchCategoryCounts.set(categoryCounts);

        // 総カテゴリ別件数を更新（フィルタ適用前）
        const totalCategoryCounts: Record<FileCategory, number> = {
          all: result.total_category_counts["all"] || 0,
          image: result.total_category_counts["image"] || 0,
          audio: result.total_category_counts["audio"] || 0,
          video: result.total_category_counts["video"] || 0,
          document: result.total_category_counts["document"] || 0,
          archive: result.total_category_counts["archive"] || 0,
          other: result.total_category_counts["other"] || 0,
        };
        this._totalSearchCategoryCounts.set(totalCategoryCounts);
      }
    } catch (error) {
      console.error('初期ファイル読み込みに失敗しました:', error);
    }
  }

  public async updateItemsPerPage(): Promise<void> {
    await this.loadSettings();
    this._currentPage.set(1); // ページをリセット
  }

  public setSearchQuery(query: string): void {
    this._searchQuery.set(query);
  }

  public setSelectedTags(tags: string[]): void {
    this._selectedTags.set(tags);
  }

  public setMetadataSearchFilters(filters: MetadataSearchFilter[]): void {
    this._metadataSearchFilters.set(filters);
  }

  public setMetadataLogic(logic: MetadataSearchLogic): void {
    this._metadataLogic.set(logic);
  }

  public async selectCategory(category: FileCategory): Promise<void> {
    this._selectedCategory.set(category);
    this._currentPage.set(1);
    
    if (this._useServerSidePagination) {
      await this.performSearch();
    }
  }

  public async goToPage(page: number): Promise<void> {
    this._currentPage.set(page);
    
    if (this._useServerSidePagination) {
      await this.performSearch();
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

  public async setSelectedDirectoryId(directoryId: string | "all"): Promise<void> {
    this._selectedDirectoryId.set(directoryId);
    this._currentPage.set(1); // ディレクトリ変更時はページをリセット
    
    // ディレクトリ変更時に検索を再実行（検索クエリがある場合のみ）
    let query: string;
    const queryUnsub = this._searchQuery.subscribe(q => query = q);
    queryUnsub();

    if (query && query.trim()) {
      await this.performSearch();
    }
  }

  public async performSearch(): Promise<void> {
    const result = await this.executeAsync(async () => {
      let query: string;
      let tags: string[];
      let filters: MetadataSearchFilter[];
      let logic: MetadataSearchLogic;
      let directoryId: string | "all";
      let category: FileCategory;

      // 現在の値を取得
      const queryUnsub = this._searchQuery.subscribe(q => query = q);
      const tagsUnsub = this._selectedTags.subscribe(t => tags = t);
      const filtersUnsub = this._metadataSearchFilters.subscribe(f => filters = f);
      const logicUnsub = this._metadataLogic.subscribe(l => logic = l);
      const dirUnsub = this._selectedDirectoryId.subscribe(d => directoryId = d);
      const categoryUnsub = this._selectedCategory.subscribe(c => category = c);

      // 購読を即座に解除
      queryUnsub();
      tagsUnsub();
      filtersUnsub();
      logicUnsub();
      dirUnsub();
      categoryUnsub();

      const currentSortOptions = this.getCurrentSortOptions();
      
      if (this._useServerSidePagination) {
        const currentPage = this.getCurrentPage();
        const itemsPerPage = this.getCurrentItemsPerPage();
        const offset = (currentPage - 1) * itemsPerPage;
        
        return await searchFilesPaginated(
          query!,
          tags!,
          filters!,
          logic!,
          directoryId!,
          currentSortOptions,
          itemsPerPage,
          offset,
          category!
        );
      } else {
        // 従来の方法（後方互換性のため残す）
        const results = await searchFiles(query!, tags!, filters!, logic!, directoryId!, currentSortOptions, category!);
        return { results, total_count: results.length };
      }
    });

    if (result) {
      this._searchResults.set(result.results);
      this._totalSearchResults.set(result.total_count);
      
      // カテゴリ別件数を更新（フィルタ適用後）
      const categoryCounts: Record<FileCategory, number> = {
        all: result.category_counts["all"] || 0,
        image: result.category_counts["image"] || 0,
        audio: result.category_counts["audio"] || 0,
        video: result.category_counts["video"] || 0,
        document: result.category_counts["document"] || 0,
        archive: result.category_counts["archive"] || 0,
        other: result.category_counts["other"] || 0,
      };
      this._searchCategoryCounts.set(categoryCounts);

      // 総カテゴリ別件数を更新（フィルタ適用前）
      const totalCategoryCounts: Record<FileCategory, number> = {
        all: result.total_category_counts["all"] || 0,
        image: result.total_category_counts["image"] || 0,
        audio: result.total_category_counts["audio"] || 0,
        video: result.total_category_counts["video"] || 0,
        document: result.total_category_counts["document"] || 0,
        archive: result.total_category_counts["archive"] || 0,
        other: result.total_category_counts["other"] || 0,
      };
      this._totalSearchCategoryCounts.set(totalCategoryCounts);
      
      // 新しい検索の場合のみページをリセット（ページ移動の場合はリセットしない）
      if (!this._useServerSidePagination) {
        this._currentPage.set(1);
      }
    }
  }

  public clearSearch(): void {
    this._searchQuery.set("");
    this._selectedTags.set([]);
    this._metadataSearchFilters.set([]);
    this._metadataLogic.set("AND");
    this._searchResults.set([]);
    this._selectedCategory.set("all");
    this._currentPage.set(1);
    this._searchCategoryCounts.set({
      all: 0,
      image: 0,
      audio: 0,
      video: 0,
      document: 0,
      archive: 0,
      other: 0,
    });
    this._totalSearchCategoryCounts.set({
      all: 0,
      image: 0,
      audio: 0,
      video: 0,
      document: 0,
      archive: 0,
      other: 0,
    });
  }

  private getCurrentSortOptions(): SortOptions {
    let currentOptions: SortOptions = { field: "modified_at", order: "desc" };
    this._sortOptions.subscribe(options => currentOptions = options)();
    return currentOptions;
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
  
  private getCurrentTotalPages(): number {
    let totalPages = 1;
    this.searchTotalPages.subscribe(pages => totalPages = pages)();
    return totalPages;
  }

  public async setSortOptions(options: SortOptions): Promise<void> {
    this._sortOptions.set(options);
    this._currentPage.set(1); // ソート変更時はページをリセット
    await this.performSearch(); // ソート変更時に検索を再実行
  }

  // タグが更新された時の再検索メソッド
  public async refreshSearchResults(): Promise<void> {
    // 既に検索結果がある場合のみ再検索を実行
    let currentResults: SearchResult[];
    const resultUnsub = this._searchResults.subscribe(results => currentResults = results);
    resultUnsub();
    
    if (currentResults && currentResults.length > 0) {
      await this.performSearch();
    }
  }
}
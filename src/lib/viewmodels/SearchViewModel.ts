import { writable, derived, type Writable, type Readable } from 'svelte/store';
import { BaseViewModel } from './BaseViewModel.js';
import { searchFiles } from '../api/search.js';
import type { SearchResult, FileCategory, MetadataSearchFilter, MetadataSearchLogic, SortOptions } from '../types.js';
import { getFileCategory } from '../utils.js';

export class SearchViewModel extends BaseViewModel {
  private _searchQuery: Writable<string> = writable("");
  private _selectedTags: Writable<string[]> = writable([]);
  private _metadataSearchFilters: Writable<MetadataSearchFilter[]> = writable([]);
  private _metadataLogic: Writable<MetadataSearchLogic> = writable("AND");
  private _searchResults: Writable<SearchResult[]> = writable([]);
  private _selectedCategory: Writable<FileCategory> = writable("all");
  private _currentPage: Writable<number> = writable(1);
  private _itemsPerPage = 25;
  private _selectedDirectoryId: Writable<string | "all"> = writable("all");
  private _sortOptions: Writable<SortOptions> = writable({ field: "modified_at", order: "desc" });

  public readonly searchQuery = this._searchQuery;
  public readonly selectedTags = this._selectedTags;
  public readonly metadataSearchFilters = this._metadataSearchFilters;
  public readonly metadataLogic = this._metadataLogic;
  public readonly searchResults = this._searchResults;
  public readonly selectedCategory = this._selectedCategory;
  public readonly currentPage = this._currentPage;
  public readonly selectedDirectoryId = this._selectedDirectoryId;
  public readonly sortOptions = this._sortOptions;

  // 派生ストア
  public readonly searchCategoryCounts: Readable<Record<FileCategory, number>> = derived(
    this._searchResults,
    (searchResults) => {
      const counts: Record<FileCategory, number> = {
        all: searchResults.length,
        image: 0,
        audio: 0,
        video: 0,
        document: 0,
        archive: 0,
        other: 0,
      };

      searchResults.forEach((result) => {
        const category = getFileCategory(result.file);
        counts[category]++;
      });

      return counts;
    }
  );

  public readonly filteredSearchResults: Readable<SearchResult[]> = derived(
    [this._searchResults, this._selectedCategory],
    ([searchResults, selectedCategory]) => {
      if (selectedCategory === "all") {
        return searchResults;
      }
      return searchResults.filter((result) => getFileCategory(result.file) === selectedCategory);
    }
  );

  public readonly searchTotalPages: Readable<number> = derived(
    this.filteredSearchResults,
    (filteredResults) => Math.ceil(filteredResults.length / this._itemsPerPage)
  );

  public readonly paginatedSearchResults: Readable<SearchResult[]> = derived(
    [this.filteredSearchResults, this._currentPage],
    ([filteredResults, currentPage]) => {
      const startIndex = (currentPage - 1) * this._itemsPerPage;
      const endIndex = startIndex + this._itemsPerPage;
      return filteredResults.slice(startIndex, endIndex);
    }
  );

  constructor() {
    super();
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

  public selectCategory(category: FileCategory): void {
    this._selectedCategory.set(category);
    this._currentPage.set(1);
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

  public setSelectedDirectoryId(directoryId: string | "all"): void {
    this._selectedDirectoryId.set(directoryId);
    // ディレクトリ変更時に検索を再実行（検索クエリがある場合のみ）
    let query: string;
    const queryUnsub = this._searchQuery.subscribe(q => query = q);
    queryUnsub();

    this.performSearch();
  }

  public async performSearch(): Promise<void> {
    const result = await this.executeAsync(async () => {
      let query: string;
      let tags: string[];
      let filters: MetadataSearchFilter[];
      let logic: MetadataSearchLogic;
      let directoryId: string | "all";

      // 現在の値を取得
      const queryUnsub = this._searchQuery.subscribe(q => query = q);
      const tagsUnsub = this._selectedTags.subscribe(t => tags = t);
      const filtersUnsub = this._metadataSearchFilters.subscribe(f => filters = f);
      const logicUnsub = this._metadataLogic.subscribe(l => logic = l);
      const dirUnsub = this._selectedDirectoryId.subscribe(d => directoryId = d);

      // 購読を即座に解除
      queryUnsub();
      tagsUnsub();
      filtersUnsub();
      logicUnsub();
      dirUnsub();

      const currentSortOptions = this.getCurrentSortOptions();
      return await searchFiles(query!, tags!, filters!, logic!, directoryId!, currentSortOptions);
    });

    if (result) {
      this._searchResults.set(result);
      this._currentPage.set(1); // 検索後はページをリセット
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
  }

  private getCurrentSortOptions(): SortOptions {
    let currentOptions: SortOptions = { field: "modified_at", order: "desc" };
    this._sortOptions.subscribe(options => currentOptions = options)();
    return currentOptions;
  }

  public setSortOptions(options: SortOptions): void {
    this._sortOptions.set(options);
    this.performSearch(); // ソート変更時に検索を再実行
  }
}
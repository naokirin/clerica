/**
 * 共通UI状態管理ストア
 * ページネーション、ソート、フィルタリングなどの汎用的なUI状態を管理
 */

import { writable, derived, type Readable, type Writable } from 'svelte/store';

export interface PaginationState {
  currentPage: number;
  itemsPerPage: number;
  totalItems: number;
}

export interface SortState {
  key: string;
  order: 'asc' | 'desc';
}

export interface FilterState {
  selectedCategory: string | null;
  searchQuery: string;
}

export interface PaginationStore extends Readable<PaginationState> {
  setCurrentPage: (page: number) => void;
  setItemsPerPage: (items: number) => void;
  setTotalItems: (total: number) => void;
  goToPage: (page: number) => void;
  goToPreviousPage: () => void;
  goToNextPage: () => void;
  goToFirstPage: () => void;
  goToLastPage: () => void;
  getTotalPages: () => number;
}

export interface SortStore extends Readable<SortState> {
  setSort: (key: string, order: 'asc' | 'desc') => void;
  toggleSort: (key: string) => void;
}

export interface FilterStore extends Readable<FilterState> {
  setCategory: (category: string | null) => void;
  setSearchQuery: (query: string) => void;
  clearFilters: () => void;
}

/**
 * ページネーション用ストア作成関数
 */
export function createPaginationStore(initialItemsPerPage: number = 20): PaginationStore {
  const { subscribe, set, update } = writable<PaginationState>({
    currentPage: 1,
    itemsPerPage: initialItemsPerPage,
    totalItems: 0
  });

  const getTotalPages = (): number => {
    let totalPages = 1;
    const unsubscribe = subscribe(state => {
      totalPages = Math.ceil(state.totalItems / state.itemsPerPage) || 1;
    });
    unsubscribe();
    return totalPages;
  };

  return {
    subscribe,
    setCurrentPage: (page: number) => update(state => ({ ...state, currentPage: page })),
    setItemsPerPage: (items: number) => update(state => ({ 
      ...state, 
      itemsPerPage: items, 
      currentPage: 1 // Reset to first page when changing items per page
    })),
    setTotalItems: (total: number) => update(state => ({ ...state, totalItems: total })),
    goToPage: (page: number) => {
      const totalPages = getTotalPages();
      const validPage = Math.max(1, Math.min(page, totalPages));
      update(state => ({ ...state, currentPage: validPage }));
    },
    goToPreviousPage: () => update(state => ({ 
      ...state, 
      currentPage: Math.max(1, state.currentPage - 1) 
    })),
    goToNextPage: () => {
      const totalPages = getTotalPages();
      update(state => ({ 
        ...state, 
        currentPage: Math.min(totalPages, state.currentPage + 1) 
      }));
    },
    goToFirstPage: () => update(state => ({ ...state, currentPage: 1 })),
    goToLastPage: () => {
      const totalPages = getTotalPages();
      update(state => ({ ...state, currentPage: totalPages }));
    },
    getTotalPages
  };
}

/**
 * ソート用ストア作成関数
 */
export function createSortStore(initialKey: string = 'name', initialOrder: 'asc' | 'desc' = 'asc'): SortStore {
  const { subscribe, set, update } = writable<SortState>({
    key: initialKey,
    order: initialOrder
  });

  return {
    subscribe,
    setSort: (key: string, order: 'asc' | 'desc') => set({ key, order }),
    toggleSort: (key: string) => update(state => ({
      key,
      order: state.key === key && state.order === 'asc' ? 'desc' : 'asc'
    }))
  };
}

/**
 * フィルター用ストア作成関数
 */
export function createFilterStore(): FilterStore {
  const { subscribe, set, update } = writable<FilterState>({
    selectedCategory: null,
    searchQuery: ''
  });

  return {
    subscribe,
    setCategory: (category: string | null) => update(state => ({ ...state, selectedCategory: category })),
    setSearchQuery: (query: string) => update(state => ({ ...state, searchQuery: query })),
    clearFilters: () => set({ selectedCategory: null, searchQuery: '' })
  };
}

/**
 * 汎用コレクション管理ストア
 * ページネーション、ソート、フィルタリングを統合
 */
export interface CollectionStore<T> {
  items: Readable<T[]>;
  pagination: PaginationStore;
  sort: SortStore;
  filter: FilterStore;
  paginatedItems: Readable<T[]>;
  setItems: (items: T[]) => void;
  refresh: () => void;
}

export function createCollectionStore<T>(
  initialItemsPerPage: number = 20,
  sortKeyExtractor: (item: T) => any = (item) => item,
  filterPredicate?: (item: T, filter: FilterState) => boolean
): CollectionStore<T> {
  const itemsStore = writable<T[]>([]);
  const pagination = createPaginationStore(initialItemsPerPage);
  const sort = createSortStore();
  const filter = createFilterStore();

  // フィルタリング済みアイテム
  const filteredItems = derived(
    [itemsStore, filter],
    ([$items, $filter]) => {
      if (!filterPredicate) return $items;
      return $items.filter(item => filterPredicate(item, $filter));
    }
  );

  // ソート済みアイテム
  const sortedItems = derived(
    [filteredItems, sort],
    ([$filteredItems, $sort]) => {
      const sorted = [...$filteredItems];
      sorted.sort((a, b) => {
        const aValue = sortKeyExtractor(a);
        const bValue = sortKeyExtractor(b);
        
        let comparison = 0;
        if (aValue < bValue) comparison = -1;
        else if (aValue > bValue) comparison = 1;
        
        return $sort.order === 'asc' ? comparison : -comparison;
      });
      return sorted;
    }
  );

  // ページネーション済みアイテム
  const paginatedItems = derived(
    [sortedItems, pagination],
    ([$sortedItems, $pagination]) => {
      const startIndex = ($pagination.currentPage - 1) * $pagination.itemsPerPage;
      const endIndex = startIndex + $pagination.itemsPerPage;
      return $sortedItems.slice(startIndex, endIndex);
    }
  );

  // フィルタ済みアイテム数の変更時にページネーション状態を更新
  filteredItems.subscribe($filteredItems => {
    pagination.setTotalItems($filteredItems.length);
  });

  return {
    items: { subscribe: itemsStore.subscribe },
    pagination,
    sort,
    filter,
    paginatedItems,
    setItems: (items: T[]) => {
      itemsStore.set(items);
      pagination.setTotalItems(items.length);
      pagination.goToFirstPage();
    },
    refresh: () => {
      // サブクラスでオーバーライドして実装
    }
  };
}
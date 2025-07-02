import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { get } from 'svelte/store';
import { SearchViewModel } from '../../lib/viewmodels/SearchViewModel.js';
import type { SearchResult, MetadataSearchFilter, File } from '../../lib/types.js';

// Mock APIs
vi.mock('../../lib/api/search.js', () => ({
  searchFiles: vi.fn()
}));

// Mock utils
vi.mock('../../lib/utils.js', () => ({
  getFileCategory: vi.fn()
}));

const { searchFiles: mockSearchFiles } = vi.mocked(await import('../../lib/api/search.js'));
const { getFileCategory: mockGetFileCategory } = vi.mocked(await import('../../lib/utils.js'));

describe('SearchViewModel', () => {
  let searchViewModel: SearchViewModel;
  
  const mockFile1: File = {
    id: '1',
    name: 'test.jpg',
    path: '/test/test.jpg',
    size: 1024,
    created_at: '2024-01-01',
    modified_at: '2024-01-01'
  };

  const mockFile2: File = {
    id: '2',
    name: 'document.pdf',
    path: '/test/document.pdf',
    size: 2048,
    created_at: '2024-01-02',
    modified_at: '2024-01-02'
  };

  const mockSearchResults: SearchResult[] = [
    {
      file: mockFile1,
      score: 0.9,
      matches: ['test']
    },
    {
      file: mockFile2,
      score: 0.8,
      matches: ['document']
    }
  ];

  beforeEach(() => {
    vi.clearAllMocks();
    mockSearchFiles.mockResolvedValue(mockSearchResults);
    mockGetFileCategory.mockImplementation((file: File) => {
      if (file.name.endsWith('.jpg')) return 'image';
      if (file.name.endsWith('.pdf')) return 'document';
      return 'other';
    });
    
    searchViewModel = new SearchViewModel();
  });

  afterEach(() => {
    searchViewModel?.dispose();
  });

  describe('constructor', () => {
    it('should initialize with default values', () => {
      expect(get(searchViewModel.searchQuery)).toBe('');
      expect(get(searchViewModel.selectedTags)).toEqual([]);
      expect(get(searchViewModel.metadataSearchFilters)).toEqual([]);
      expect(get(searchViewModel.searchResults)).toEqual([]);
      expect(get(searchViewModel.selectedCategory)).toBe('all');
      expect(get(searchViewModel.currentPage)).toBe(1);
    });
  });

  describe('search parameters', () => {
    it('should set search query', () => {
      searchViewModel.setSearchQuery('test query');
      expect(get(searchViewModel.searchQuery)).toBe('test query');
    });

    it('should set selected tags', () => {
      const tags = ['tag1', 'tag2'];
      searchViewModel.setSelectedTags(tags);
      expect(get(searchViewModel.selectedTags)).toEqual(tags);
    });

    it('should set metadata search filters', () => {
      const filters: MetadataSearchFilter[] = [
        { field: 'size', operator: 'greater_than', value: '1000' }
      ];
      searchViewModel.setMetadataSearchFilters(filters);
      expect(get(searchViewModel.metadataSearchFilters)).toEqual(filters);
    });
  });

  describe('category filtering', () => {
    beforeEach(async () => {
      await searchViewModel.performSearch();
    });

    it('should show all results by default', () => {
      const filtered = get(searchViewModel.filteredSearchResults);
      expect(filtered).toEqual(mockSearchResults);
    });

    it('should filter by category', () => {
      searchViewModel.selectCategory('image');
      
      const filtered = get(searchViewModel.filteredSearchResults);
      expect(filtered).toHaveLength(1);
      expect(filtered[0].file.name).toBe('test.jpg');
    });

    it('should reset page when category changes', () => {
      searchViewModel.goToPage(2);
      searchViewModel.selectCategory('image');
      
      expect(get(searchViewModel.currentPage)).toBe(1);
    });

    it('should calculate category counts', () => {
      const counts = get(searchViewModel.searchCategoryCounts);
      
      expect(counts.all).toBe(2);
      expect(counts.image).toBe(1);
      expect(counts.document).toBe(1);
      expect(counts.other).toBe(0);
    });
  });

  describe('pagination', () => {
    beforeEach(async () => {
      await searchViewModel.performSearch();
    });

    it('should calculate total pages', () => {
      const totalPages = get(searchViewModel.searchTotalPages);
      expect(totalPages).toBe(1); // 2 results, 25 per page
    });

    it('should paginate results', () => {
      const paginated = get(searchViewModel.paginatedSearchResults);
      expect(paginated).toEqual(mockSearchResults);
    });

    it('should navigate pages', () => {
      searchViewModel.goToPage(2);
      expect(get(searchViewModel.currentPage)).toBe(2);
      
      searchViewModel.goToPreviousPage();
      expect(get(searchViewModel.currentPage)).toBe(1);
      
      searchViewModel.goToNextPage();
      expect(get(searchViewModel.currentPage)).toBe(2);
      
      searchViewModel.goToFirstPage();
      expect(get(searchViewModel.currentPage)).toBe(1);
      
      searchViewModel.goToLastPage(5);
      expect(get(searchViewModel.currentPage)).toBe(5);
    });

    it('should not go below page 1', () => {
      searchViewModel.goToPage(1);
      searchViewModel.goToPreviousPage();
      
      expect(get(searchViewModel.currentPage)).toBe(1);
    });
  });

  describe('performSearch', () => {
    it('should search with current parameters', async () => {
      searchViewModel.setSearchQuery('test');
      searchViewModel.setSelectedTags(['tag1']);
      searchViewModel.setMetadataSearchFilters([
        { field: 'size', operator: 'greater_than', value: '1000' }
      ]);

      await searchViewModel.performSearch();

      expect(mockSearchFiles).toHaveBeenCalledWith(
        'test',
        ['tag1'],
        [{ field: 'size', operator: 'greater_than', value: '1000' }]
      );
      expect(get(searchViewModel.searchResults)).toEqual(mockSearchResults);
      expect(get(searchViewModel.currentPage)).toBe(1);
    });

    it('should handle search errors', async () => {
      const error = new Error('Search failed');
      mockSearchFiles.mockRejectedValue(error);

      await searchViewModel.performSearch();

      expect(get(searchViewModel.error)).toBeTruthy();
    });

    it('should reset page after search', async () => {
      searchViewModel.goToPage(3);
      
      await searchViewModel.performSearch();
      
      expect(get(searchViewModel.currentPage)).toBe(1);
    });
  });

  describe('clearSearch', () => {
    it('should reset all search parameters', () => {
      searchViewModel.setSearchQuery('test');
      searchViewModel.setSelectedTags(['tag1']);
      searchViewModel.setMetadataSearchFilters([
        { field: 'size', operator: 'greater_than', value: '1000' }
      ]);
      searchViewModel.selectCategory('image');
      searchViewModel.goToPage(3);

      searchViewModel.clearSearch();

      expect(get(searchViewModel.searchQuery)).toBe('');
      expect(get(searchViewModel.selectedTags)).toEqual([]);
      expect(get(searchViewModel.metadataSearchFilters)).toEqual([]);
      expect(get(searchViewModel.searchResults)).toEqual([]);
      expect(get(searchViewModel.selectedCategory)).toBe('all');
      expect(get(searchViewModel.currentPage)).toBe(1);
    });
  });

  describe('store reactivity', () => {
    it('should notify subscribers when search query changes', () => {
      const values: string[] = [];
      searchViewModel.searchQuery.subscribe(value => values.push(value));

      searchViewModel.setSearchQuery('test');
      searchViewModel.setSearchQuery('another test');

      expect(values).toEqual(['', 'test', 'another test']);
    });

    it('should notify subscribers when search results change', () => {
      const values: SearchResult[][] = [];
      searchViewModel.searchResults.subscribe(value => values.push([...value]));

      searchViewModel['_searchResults'].set(mockSearchResults);

      expect(values).toHaveLength(2);
      expect(values[1]).toEqual(mockSearchResults);
    });

    it('should update derived stores when results change', () => {
      searchViewModel['_searchResults'].set(mockSearchResults);

      const counts = get(searchViewModel.searchCategoryCounts);
      const filtered = get(searchViewModel.filteredSearchResults);
      const paginated = get(searchViewModel.paginatedSearchResults);

      expect(counts.all).toBe(2);
      expect(filtered).toEqual(mockSearchResults);
      expect(paginated).toEqual(mockSearchResults);
    });
  });
});
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { get } from 'svelte/store';
import { SearchViewModel } from '../../lib/viewmodels/SearchViewModel';
import type { SearchResult, MetadataSearchFilter, File } from '../../lib/types';

// Mock APIs
vi.mock('../../lib/api/search', () => ({
  searchFiles: vi.fn(),
  searchFilesPaginated: vi.fn()
}));

vi.mock('../../lib/api/settings', () => ({
  getSettings: vi.fn()
}));

// Mock utils
vi.mock('../../lib/utils', () => ({
  getFileCategory: vi.fn()
}));

// Mock error store
vi.mock('../../lib/stores/error', () => ({
  errorStore: {
    showError: vi.fn()
  }
}));

const { searchFiles: mockSearchFiles, searchFilesPaginated: mockSearchFilesPaginated } = vi.mocked(await import('../../lib/api/search'));
const { getSettings: mockGetSettings } = vi.mocked(await import('../../lib/api/settings'));
const { getFileCategory: mockGetFileCategory } = vi.mocked(await import('../../lib/utils'));

describe('SearchViewModel', () => {
  let searchViewModel: SearchViewModel;
  
  const mockFile1: File = {
    id: '1',
    name: 'test.jpg',
    path: '/test/test.jpg',
    directory_id: 'dir1',
    size: 1024,
    file_type: 'jpg',
    created_at: '2024-01-01',
    modified_at: '2024-01-01',
    birth_time: null,
    inode: 12345,
    is_directory: false,
    created_at_db: '2024-01-01T00:00:00Z',
    updated_at_db: '2024-01-01T00:00:00Z',
    file_size: 1024,
    mime_type: 'image/jpeg',
    permissions: 'rw-r--r--',
    owner_uid: 1000,
    group_gid: 1000,
    hard_links: 1,
    device_id: 123,
    last_accessed: '2024-01-01',
    metadata: null
  };

  const mockFile2: File = {
    id: '2',
    name: 'document.pdf',
    path: '/test/document.pdf',
    directory_id: 'dir1',
    size: 2048,
    file_type: 'pdf',
    created_at: '2024-01-02',
    modified_at: '2024-01-02',
    birth_time: null,
    inode: 54321,
    is_directory: false,
    created_at_db: '2024-01-02T00:00:00Z',
    updated_at_db: '2024-01-02T00:00:00Z',
    file_size: 2048,
    mime_type: 'application/pdf',
    permissions: 'rw-r--r--',
    owner_uid: 1000,
    group_gid: 1000,
    hard_links: 1,
    device_id: 123,
    last_accessed: '2024-01-02',
    metadata: null
  };

  const mockSearchResults: SearchResult[] = [
    {
      file: mockFile1,
      tags: [],
      score: 0.9
    },
    {
      file: mockFile2,
      tags: [],
      score: 0.8
    }
  ];

  beforeEach(async () => {
    vi.clearAllMocks();
    mockSearchFiles.mockResolvedValue(mockSearchResults);
    mockSearchFilesPaginated.mockResolvedValue({
      results: mockSearchResults,
      total_count: 40, // More than 20 items for pagination
      category_counts: {
        all: 2,
        image: 1,
        audio: 0,
        video: 0,
        document: 1,
        archive: 0,
        other: 0,
      },
      total_category_counts: {
        all: 2,
        image: 1,
        audio: 0,
        video: 0,
        document: 1,
        archive: 0,
        other: 0,
      }
    });
    mockGetSettings.mockResolvedValue({ show_hidden_files: false, files_per_page: 20 });
    mockGetFileCategory.mockImplementation((file: File) => {
      if (file.name.endsWith('.jpg')) return 'image';
      if (file.name.endsWith('.pdf')) return 'document';
      return 'other';
    });
    
    searchViewModel = new SearchViewModel();
    // Wait for constructor async operations to complete
    await new Promise(resolve => setTimeout(resolve, 10));
  });

  afterEach(() => {
    searchViewModel?.dispose();
  });

  describe('constructor', () => {
    it('should initialize with default values', () => {
      expect(get(searchViewModel.searchQuery)).toBe('');
      expect(get(searchViewModel.selectedTags)).toEqual([]);
      expect(get(searchViewModel.metadataSearchFilters)).toEqual([]);
      expect(get(searchViewModel.metadataLogic)).toBe('AND');
      expect(get(searchViewModel.selectedCategory)).toBe('all');
      expect(get(searchViewModel.currentPage)).toBe(1);
    });

    it('should load initial files on construction', () => {
      // After constructor async operations, searchResults should contain data
      expect(get(searchViewModel.searchResults)).toEqual(mockSearchResults);
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
        { keyId: 'size', keyName: 'size', displayName: 'Size', dataType: 'number', operator: 'greater_than', value: '1000' }
      ];
      searchViewModel.setMetadataSearchFilters(filters);
      expect(get(searchViewModel.metadataSearchFilters)).toEqual(filters);
    });

    it('should set metadata logic', () => {
      searchViewModel.setMetadataLogic('OR');
      expect(get(searchViewModel.metadataLogic)).toBe('OR');
    });
  });

  describe('category filtering', () => {
    it('should show all results by default', () => {
      const filtered = get(searchViewModel.filteredSearchResults);
      expect(filtered).toEqual(mockSearchResults);
    });

    it('should filter by category', async () => {
      // Setup mock for image category filter
      const imageResults = [mockSearchResults[0]]; // Only the JPG file
      mockSearchFilesPaginated.mockResolvedValueOnce({
        results: imageResults,
        total_count: 1,
        category_counts: {
          all: 1,
          image: 1,
          audio: 0,
          video: 0,
          document: 0,
          archive: 0,
          other: 0,
        },
        total_category_counts: {
          all: 2,
          image: 1,
          audio: 0,
          video: 0,
          document: 1,
          archive: 0,
          other: 0,
        }
      });
      
      await searchViewModel.selectCategory('image');
      
      const filtered = get(searchViewModel.filteredSearchResults);
      expect(filtered).toHaveLength(1);
      expect(filtered[0].file.name).toBe('test.jpg');
    });

    it('should reset page when category changes', async () => {
      await searchViewModel.goToPage(2);
      await searchViewModel.selectCategory('image');
      
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

    it('should calculate total pages', () => {
      const totalPages = get(searchViewModel.searchTotalPages);
      expect(totalPages).toBe(2); // 40 results, 20 per page
    });

    it('should paginate results', () => {
      const paginated = get(searchViewModel.paginatedSearchResults);
      expect(paginated).toEqual(mockSearchResults);
    });

    it('should navigate pages', async () => {
      await searchViewModel.goToPage(2);
      expect(get(searchViewModel.currentPage)).toBe(2);
      
      await searchViewModel.goToPreviousPage();
      expect(get(searchViewModel.currentPage)).toBe(1);
      
      await searchViewModel.goToNextPage();
      expect(get(searchViewModel.currentPage)).toBe(2);
      
      await searchViewModel.goToFirstPage();
      expect(get(searchViewModel.currentPage)).toBe(1);
      
      await searchViewModel.goToLastPage();
      expect(get(searchViewModel.currentPage)).toBe(2);
    });

    it('should not go below page 1', async () => {
      await searchViewModel.goToPage(1);
      await searchViewModel.goToPreviousPage();
      
      expect(get(searchViewModel.currentPage)).toBe(1);
    });
  });

  describe('performSearch', () => {
    it('should search with current parameters', async () => {
      searchViewModel.setSearchQuery('test');
      searchViewModel.setSelectedTags(['tag1']);
      searchViewModel.setMetadataSearchFilters([
        { keyId: 'size', keyName: 'size', displayName: 'Size', dataType: 'number', operator: 'greater_than', value: '1000' }
      ]);

      await searchViewModel.performSearch();

      expect(mockSearchFilesPaginated).toHaveBeenCalledWith(
        'test',
        ['tag1'],
        [{ field: 'size', operator: 'greater_than', value: '1000' }],
        'AND',
        'all',
        { field: 'modified_at', order: 'desc' },
        20, // itemsPerPage
        0,  // offset
        'all' // category
      );
      expect(get(searchViewModel.searchResults)).toEqual(mockSearchResults);
      expect(get(searchViewModel.currentPage)).toBe(1);
    });

    it('should handle search errors', async () => {
      const error = new Error('Search failed');
      mockSearchFilesPaginated.mockRejectedValueOnce(error);

      await searchViewModel.performSearch();

      expect(get(searchViewModel.error)).toBeTruthy();
    });

    it('should maintain current page after search', async () => {
      await searchViewModel.goToPage(2);
      
      await searchViewModel.performSearch();
      
      expect(get(searchViewModel.currentPage)).toBe(2);
    });
  });

  describe('clearSearch', () => {
    it('should reset all search parameters', () => {
      searchViewModel.setSearchQuery('test');
      searchViewModel.setSelectedTags(['tag1']);
      searchViewModel.setMetadataSearchFilters([
        { keyId: 'size', keyName: 'size', displayName: 'Size', dataType: 'number', operator: 'greater_than', value: '1000' }
      ]);
      searchViewModel.setMetadataLogic('OR');
      searchViewModel.selectCategory('image');
      searchViewModel.goToPage(3);

      searchViewModel.clearSearch();

      expect(get(searchViewModel.searchQuery)).toBe('');
      expect(get(searchViewModel.selectedTags)).toEqual([]);
      expect(get(searchViewModel.metadataSearchFilters)).toEqual([]);
      expect(get(searchViewModel.metadataLogic)).toBe('AND');
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
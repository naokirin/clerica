import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { get } from 'svelte/store';
import { FileViewModel } from '../../lib/viewmodels/FileViewModel';
import type { File } from '../../lib/types';

// Mock APIs
vi.mock('../../lib/api/files', () => ({
  getFiles: vi.fn(),
  getFilesWithTags: vi.fn(),
  getFilesByDirectory: vi.fn(),
  getFilesByDirectoryWithTags: vi.fn(),
  getFilesPaginated: vi.fn(),
  getFilesByDirectoryPaginated: vi.fn(),
  getFilesPaginatedWithCategory: vi.fn(),
  getFilesByDirectoryPaginatedWithCategory: vi.fn(),
  countFiles: vi.fn(),
  countFilesByDirectory: vi.fn(),
  countFilesByCategory: vi.fn(),
  countFilesWithCategory: vi.fn(),
  countFilesByDirectoryWithCategory: vi.fn(),
  getFileTags: vi.fn(),
  openFile: vi.fn(),
  revealInFinder: vi.fn(),
  deleteFile: vi.fn()
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

const { 
  getFiles: mockGetFiles, 
  getFilesWithTags: mockGetFilesWithTags,
  getFilesByDirectory: mockGetFilesByDirectory,
  getFilesByDirectoryWithTags: mockGetFilesByDirectoryWithTags,
  getFilesPaginated: mockGetFilesPaginated,
  getFilesByDirectoryPaginated: mockGetFilesByDirectoryPaginated,
  getFilesPaginatedWithCategory: mockGetFilesPaginatedWithCategory,
  getFilesByDirectoryPaginatedWithCategory: mockGetFilesByDirectoryPaginatedWithCategory,
  countFiles: mockCountFiles,
  countFilesByDirectory: mockCountFilesByDirectory,
  countFilesByCategory: mockCountFilesByCategory,
  countFilesWithCategory: mockCountFilesWithCategory,
  countFilesByDirectoryWithCategory: mockCountFilesByDirectoryWithCategory,
  getFileTags: mockGetFileTags,
  openFile: mockOpenFile, 
  revealInFinder: mockRevealInFinder, 
  deleteFile: mockDeleteFile 
} = vi.mocked(await import('../../lib/api/files'));
const { getSettings: mockGetSettings } = vi.mocked(await import('../../lib/api/settings'));
const { getFileCategory: mockGetFileCategory } = vi.mocked(await import('../../lib/utils'));

describe('FileViewModel', () => {
  let fileViewModel: FileViewModel;
  
  const mockFiles: File[] = [
    {
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
    },
    {
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
    }
  ];

  const mockFilesWithTags = [
    { file: mockFiles[0], tags: [] },
    { file: mockFiles[1], tags: [] }
  ];

  beforeEach(() => {
    vi.clearAllMocks();
    mockGetFiles.mockResolvedValue(mockFiles);
    mockGetFilesWithTags.mockResolvedValue(mockFilesWithTags);
    mockGetFilesByDirectory.mockResolvedValue(mockFiles);
    mockGetFilesByDirectoryWithTags.mockResolvedValue(mockFilesWithTags);
    mockGetFilesPaginated.mockResolvedValue(mockFiles);
    mockGetFilesByDirectoryPaginated.mockResolvedValue(mockFiles);
    mockGetFilesPaginatedWithCategory.mockResolvedValue(mockFiles);
    mockGetFilesByDirectoryPaginatedWithCategory.mockResolvedValue(mockFiles);
    mockCountFiles.mockResolvedValue(mockFiles.length);
    mockCountFilesByDirectory.mockResolvedValue(mockFiles.length);
    mockCountFilesByCategory.mockResolvedValue({
      all: 0,
      image: 0,
      audio: 0,
      video: 0,
      document: 0,
      archive: 0,
      other: 0,
    });
    mockCountFilesWithCategory.mockResolvedValue(1);
    mockCountFilesByDirectoryWithCategory.mockResolvedValue(1);
    mockGetFileTags.mockResolvedValue([]);
    mockGetSettings.mockResolvedValue({ show_hidden_files: false, files_per_page: 20 });
    mockGetFileCategory.mockImplementation((file: File) => {
      if (file.name.endsWith('.jpg')) return 'image';
      if (file.name.endsWith('.pdf')) return 'document';
      return 'other';
    });
  });

  afterEach(() => {
    fileViewModel?.dispose();
  });

  describe('constructor', () => {
    it('should initialize with default values', async () => {
      fileViewModel = new FileViewModel();
      
      expect(get(fileViewModel.selectedFile)).toBeNull();
      expect(get(fileViewModel.selectedCategory)).toBe('all');
      expect(get(fileViewModel.currentPage)).toBe(1);
      expect(get(fileViewModel.isDeleting)).toBe(false);
    });

    it('should not load files automatically on initialization', async () => {
      fileViewModel = new FileViewModel();
      
      // Wait for potential async initialization
      await new Promise(resolve => setTimeout(resolve, 0));
      
      expect(mockGetFilesWithTags).not.toHaveBeenCalled();
    });
  });

  describe('loadFiles', () => {
    beforeEach(() => {
      fileViewModel = new FileViewModel();
    });

    it('should load files successfully', async () => {
      await fileViewModel.loadFiles();
      
      expect(mockGetFilesPaginated).toHaveBeenCalled();
      expect(mockCountFiles).toHaveBeenCalled();
      expect(get(fileViewModel.files)).toEqual(mockFiles);
    });

    it('should handle load errors gracefully', async () => {
      const error = new Error('Load failed');
      mockGetFilesPaginated.mockRejectedValueOnce(error);
      
      // loadFiles should not throw even when API fails  
      await expect(fileViewModel.loadFiles()).resolves.not.toThrow();
      
      // Verify that the function completed without throwing
      expect(mockGetFilesPaginated).toHaveBeenCalled();
    });
  });

  describe('file selection', () => {
    beforeEach(() => {
      fileViewModel = new FileViewModel();
    });

    it('should select file', () => {
      fileViewModel.selectFile(mockFiles[0]);
      
      expect(get(fileViewModel.selectedFile)).toEqual(mockFiles[0]);
    });

    it('should close file details', () => {
      fileViewModel.selectFile(mockFiles[0]);
      fileViewModel.closeFileDetails();
      
      expect(get(fileViewModel.selectedFile)).toBeNull();
    });
  });

  describe('category filtering', () => {
    beforeEach(async () => {
      fileViewModel = new FileViewModel();
      // Wait for constructor async operations to complete
      await new Promise(resolve => setTimeout(resolve, 10));
      await fileViewModel.loadFiles();
    });

    it('should show all files by default', () => {
      const filtered = get(fileViewModel.filteredFiles);
      expect(filtered).toEqual(mockFiles);
    });

    it('should filter by category', async () => {
      const imageFiles = [mockFiles[0]]; // JPGファイルのみ
      mockGetFilesPaginatedWithCategory.mockResolvedValue(imageFiles);
      
      await fileViewModel.selectCategory('image');
      
      expect(mockGetFilesPaginatedWithCategory).toHaveBeenCalledWith(
        'image',
        expect.any(Object), // sortOptions
        expect.any(Number), // itemsPerPage
        expect.any(Number)  // offset
      );
      expect(get(fileViewModel.files)).toEqual(imageFiles);
    });

    it('should reset page when category changes', async () => {
      fileViewModel.goToPage(2);
      await fileViewModel.selectCategory('image');
      
      expect(get(fileViewModel.currentPage)).toBe(1);
    });

    it('should initialize category counts', async () => {
      // Wait for async loadCategoryCounts to complete
      await new Promise(resolve => setTimeout(resolve, 10));
      
      const counts = get(fileViewModel.categoryCounts);
      
      expect(counts).toBeDefined();
      expect(counts).toEqual({
        all: 0,
        image: 0,
        audio: 0,
        video: 0,
        document: 0,
        archive: 0,
        other: 0
      });
    });
  });

  describe('pagination', () => {
    beforeEach(async () => {
      fileViewModel = new FileViewModel();
      // Set up mock to return a total count that allows pagination
      mockCountFiles.mockResolvedValue(40); // More than 20 items for pagination
      await fileViewModel.loadFiles();
    });

    it('should calculate total pages', () => {
      const totalPages = get(fileViewModel.totalPages);
      expect(totalPages).toBe(2); // 40 files, 20 per page
    });

    it('should paginate files', () => {
      const paginated = get(fileViewModel.paginatedFiles);
      expect(paginated).toEqual(mockFiles);
    });

    it('should navigate pages', async () => {
      await fileViewModel.goToPage(2);
      expect(get(fileViewModel.currentPage)).toBe(2);
      
      await fileViewModel.goToPreviousPage();
      expect(get(fileViewModel.currentPage)).toBe(1);
      
      await fileViewModel.goToNextPage();
      expect(get(fileViewModel.currentPage)).toBe(2);
      
      await fileViewModel.goToFirstPage();
      expect(get(fileViewModel.currentPage)).toBe(1);
      
      await fileViewModel.goToLastPage();
      expect(get(fileViewModel.currentPage)).toBe(2);
    });

    it('should not go below page 1', () => {
      fileViewModel.goToPage(1);
      fileViewModel.goToPreviousPage();
      
      expect(get(fileViewModel.currentPage)).toBe(1);
    });
  });

  describe('file operations', () => {
    beforeEach(async () => {
      fileViewModel = new FileViewModel();
      await fileViewModel.loadFiles();
    });

    it('should open file successfully', async () => {
      mockOpenFile.mockResolvedValue(undefined);
      
      const result = await fileViewModel.openSelectedFile('/test/file.txt');
      
      expect(mockOpenFile).toHaveBeenCalledWith('/test/file.txt');
      expect(mockGetFilesPaginated).toHaveBeenCalled(); // reload after operation
      expect(result).toBe(true);
    });

    it('should handle open file error', async () => {
      mockOpenFile.mockRejectedValue(new Error('Open failed'));
      
      const result = await fileViewModel.openSelectedFile('/test/file.txt');
      
      expect(result).toBe(false);
    });

    it('should reveal file in finder', async () => {
      mockRevealInFinder.mockResolvedValue(undefined);
      
      const result = await fileViewModel.revealFileInFinder('/test/file.txt');
      
      expect(mockRevealInFinder).toHaveBeenCalledWith('/test/file.txt');
      expect(result).toBe(true);
    });

    it('should handle reveal in finder error', async () => {
      mockRevealInFinder.mockRejectedValue(new Error('Reveal failed'));
      
      const result = await fileViewModel.revealFileInFinder('/test/file.txt');
      
      expect(result).toBe(false);
    });

    it('should delete file successfully', async () => {
      mockDeleteFile.mockResolvedValue(undefined);
      fileViewModel.selectFile(mockFiles[0]);
      
      const result = await fileViewModel.deleteSelectedFile('/test/file.txt');
      
      expect(get(fileViewModel.isDeleting)).toBe(false);
      expect(mockDeleteFile).toHaveBeenCalledWith('/test/file.txt');
      expect(mockGetFilesPaginated).toHaveBeenCalled(); // reload after operation
      expect(get(fileViewModel.selectedFile)).toBeNull();
      expect(result).toBe(true);
    });

    it('should handle delete file error', async () => {
      mockDeleteFile.mockRejectedValue(new Error('Delete failed'));
      
      const result = await fileViewModel.deleteSelectedFile('/test/file.txt');
      
      expect(get(fileViewModel.isDeleting)).toBe(false);
      expect(result).toBe(false);
    });

    it('should manage deleting state', async () => {
      mockDeleteFile.mockImplementation(() => new Promise(resolve => setTimeout(resolve, 100)));
      
      const deletePromise = fileViewModel.deleteSelectedFile('/test/file.txt');
      
      // Check deleting state is true during operation
      expect(get(fileViewModel.isDeleting)).toBe(true);
      
      await deletePromise;
      
      // Check deleting state is false after completion
      expect(get(fileViewModel.isDeleting)).toBe(false);
    });
  });

  describe('store reactivity', () => {
    beforeEach(() => {
      fileViewModel = new FileViewModel();
    });

    it('should notify subscribers when files change', () => {
      const values: File[][] = [];
      fileViewModel.files.subscribe(value => values.push([...value]));
      
      fileViewModel['_filesWithTags'].set(mockFilesWithTags);
      
      expect(values).toHaveLength(2);
      expect(values[1]).toEqual(mockFiles);
    });

    it('should notify subscribers when category changes', () => {
      const values: string[] = [];
      fileViewModel.selectedCategory.subscribe(value => values.push(value));
      
      fileViewModel.selectCategory('image');
      
      expect(values).toEqual(['all', 'image']);
    });
  });
});
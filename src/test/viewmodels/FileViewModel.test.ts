import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { get } from 'svelte/store';
import { FileViewModel } from '../../lib/viewmodels/FileViewModel.js';
import type { File } from '../../lib/types.js';

// Mock APIs
vi.mock('../../lib/api/files.js', () => ({
  getFiles: vi.fn(),
  openFile: vi.fn(),
  revealInFinder: vi.fn(),
  deleteFile: vi.fn()
}));

// Mock utils
vi.mock('../../lib/utils.js', () => ({
  getFileCategory: vi.fn()
}));

const { getFiles: mockGetFiles, openFile: mockOpenFile, revealInFinder: mockRevealInFinder, deleteFile: mockDeleteFile } = vi.mocked(await import('../../lib/api/files.js'));
const { getFileCategory: mockGetFileCategory } = vi.mocked(await import('../../lib/utils.js'));

describe('FileViewModel', () => {
  let fileViewModel: FileViewModel;
  
  const mockFiles: File[] = [
    {
      id: '1',
      name: 'test.jpg',
      path: '/test/test.jpg',
      size: 1024,
      created_at: '2024-01-01',
      modified_at: '2024-01-01'
    },
    {
      id: '2',
      name: 'document.pdf',
      path: '/test/document.pdf',
      size: 2048,
      created_at: '2024-01-02',
      modified_at: '2024-01-02'
    }
  ];

  beforeEach(() => {
    vi.clearAllMocks();
    mockGetFiles.mockResolvedValue(mockFiles);
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

    it('should load files on initialization', async () => {
      fileViewModel = new FileViewModel();
      
      // Wait for async initialization
      await new Promise(resolve => setTimeout(resolve, 0));
      
      expect(mockGetFiles).toHaveBeenCalled();
    });
  });

  describe('loadFiles', () => {
    beforeEach(() => {
      fileViewModel = new FileViewModel();
    });

    it('should load files successfully', async () => {
      await fileViewModel.loadFiles();
      
      expect(mockGetFiles).toHaveBeenCalled();
      expect(get(fileViewModel.files)).toEqual(mockFiles);
    });

    it('should handle load errors', async () => {
      const error = new Error('Load failed');
      mockGetFiles.mockRejectedValue(error);
      
      await fileViewModel.loadFiles();
      
      expect(get(fileViewModel.error)).toBeTruthy();
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
      await fileViewModel.loadFiles();
    });

    it('should show all files by default', () => {
      const filtered = get(fileViewModel.filteredFiles);
      expect(filtered).toEqual(mockFiles);
    });

    it('should filter by category', () => {
      fileViewModel.selectCategory('image');
      
      const filtered = get(fileViewModel.filteredFiles);
      expect(filtered).toHaveLength(1);
      expect(filtered[0].name).toBe('test.jpg');
    });

    it('should reset page when category changes', () => {
      fileViewModel.goToPage(2);
      fileViewModel.selectCategory('image');
      
      expect(get(fileViewModel.currentPage)).toBe(1);
    });

    it('should calculate category counts', () => {
      const counts = get(fileViewModel.categoryCounts);
      
      expect(counts.all).toBe(2);
      expect(counts.image).toBe(1);
      expect(counts.document).toBe(1);
      expect(counts.other).toBe(0);
    });
  });

  describe('pagination', () => {
    beforeEach(async () => {
      fileViewModel = new FileViewModel();
      await fileViewModel.loadFiles();
    });

    it('should calculate total pages', () => {
      const totalPages = get(fileViewModel.totalPages);
      expect(totalPages).toBe(1); // 2 files, 25 per page
    });

    it('should paginate files', () => {
      const paginated = get(fileViewModel.paginatedFiles);
      expect(paginated).toEqual(mockFiles);
    });

    it('should navigate pages', () => {
      fileViewModel.goToPage(2);
      expect(get(fileViewModel.currentPage)).toBe(2);
      
      fileViewModel.goToPreviousPage();
      expect(get(fileViewModel.currentPage)).toBe(1);
      
      fileViewModel.goToNextPage();
      expect(get(fileViewModel.currentPage)).toBe(2);
      
      fileViewModel.goToFirstPage();
      expect(get(fileViewModel.currentPage)).toBe(1);
      
      fileViewModel.goToLastPage(5);
      expect(get(fileViewModel.currentPage)).toBe(5);
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
      expect(mockGetFiles).toHaveBeenCalledTimes(3); // constructor load + explicit load + reload
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
      expect(mockGetFiles).toHaveBeenCalledTimes(3); // constructor load + explicit load + reload
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
      
      fileViewModel['_files'].set(mockFiles);
      
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
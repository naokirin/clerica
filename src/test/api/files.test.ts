import { describe, it, expect, vi, beforeEach } from 'vitest';
import { getFiles, openFile, revealInFinder, deleteFile } from '../../lib/api/files.js';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

const { invoke: mockInvoke } = vi.mocked(await import('@tauri-apps/api/core'));

describe('files API', () => {
  beforeEach(() => {
    mockInvoke.mockClear();
  });

  describe('getFiles', () => {
    it('should call invoke with correct command', async () => {
      const mockFiles = [
        { 
          id: '1', 
          name: 'test.txt', 
          path: '/test/test.txt', 
          size: 1024,
          created_at: '2024-01-01',
          modified_at: '2024-01-01'
        }
      ];
      mockInvoke.mockResolvedValue(mockFiles);

      const result = await getFiles();

      expect(mockInvoke).toHaveBeenCalledWith('get_files', {
        sortField: 'modified_at',
        sortOrder: 'desc'
      });
      expect(result).toEqual(mockFiles);
    });

    it('should handle empty files list', async () => {
      mockInvoke.mockResolvedValue([]);

      const result = await getFiles();

      expect(mockInvoke).toHaveBeenCalledWith('get_files', {
        sortField: 'modified_at',
        sortOrder: 'desc'
      });
      expect(result).toEqual([]);
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to get files');
      mockInvoke.mockRejectedValue(error);

      await expect(getFiles()).rejects.toThrow('Failed to get files');
      expect(mockInvoke).toHaveBeenCalledWith('get_files', {
        sortField: 'modified_at',
        sortOrder: 'desc'
      });
    });
  });

  describe('openFile', () => {
    it('should call invoke with correct command and parameters', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await openFile('/test/file.txt');

      expect(mockInvoke).toHaveBeenCalledWith('open_file', {
        filePath: '/test/file.txt'
      });
    });

    it('should handle special characters in file path', async () => {
      mockInvoke.mockResolvedValue(undefined);
      const pathWithSpaces = '/test/file with spaces.txt';

      await openFile(pathWithSpaces);

      expect(mockInvoke).toHaveBeenCalledWith('open_file', {
        filePath: pathWithSpaces
      });
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to open file');
      mockInvoke.mockRejectedValue(error);

      await expect(openFile('/test/file.txt')).rejects.toThrow('Failed to open file');
    });
  });

  describe('revealInFinder', () => {
    it('should call invoke with correct command and parameters', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await revealInFinder('/test/file.txt');

      expect(mockInvoke).toHaveBeenCalledWith('reveal_in_finder', {
        filePath: '/test/file.txt'
      });
    });

    it('should handle directory paths', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await revealInFinder('/test/directory/');

      expect(mockInvoke).toHaveBeenCalledWith('reveal_in_finder', {
        filePath: '/test/directory/'
      });
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to reveal in finder');
      mockInvoke.mockRejectedValue(error);

      await expect(revealInFinder('/test/file.txt')).rejects.toThrow('Failed to reveal in finder');
    });
  });

  describe('deleteFile', () => {
    it('should call invoke with correct command and parameters', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await deleteFile('/test/file.txt');

      expect(mockInvoke).toHaveBeenCalledWith('delete_file', {
        filePath: '/test/file.txt'
      });
    });

    it('should handle non-existent file paths', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await deleteFile('/non/existent/file.txt');

      expect(mockInvoke).toHaveBeenCalledWith('delete_file', {
        filePath: '/non/existent/file.txt'
      });
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to delete file');
      mockInvoke.mockRejectedValue(error);

      await expect(deleteFile('/test/file.txt')).rejects.toThrow('Failed to delete file');
    });
  });
});
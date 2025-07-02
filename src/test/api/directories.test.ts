import { describe, it, expect, vi, beforeEach } from 'vitest';
import { getDirectories, addDirectory, removeDirectory, rescanDirectory } from '../../lib/api/directories.js';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

const { invoke: mockInvoke } = vi.mocked(await import('@tauri-apps/api/core'));

describe('directories API', () => {
  beforeEach(() => {
    mockInvoke.mockClear();
  });

  describe('getDirectories', () => {
    it('should call invoke with correct command', async () => {
      const mockDirectories = [
        { id: '1', name: 'Test Dir', path: '/test/path', createdAt: '2024-01-01' }
      ];
      mockInvoke.mockResolvedValue(mockDirectories);

      const result = await getDirectories();

      expect(mockInvoke).toHaveBeenCalledWith('get_directories');
      expect(result).toEqual(mockDirectories);
    });

    it('should handle empty directories list', async () => {
      mockInvoke.mockResolvedValue([]);

      const result = await getDirectories();

      expect(result).toEqual([]);
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to get directories');
      mockInvoke.mockRejectedValue(error);

      await expect(getDirectories()).rejects.toThrow('Failed to get directories');
    });
  });

  describe('addDirectory', () => {
    it('should call invoke with correct command and parameters', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await addDirectory('/test/path', 'Test Directory');

      expect(mockInvoke).toHaveBeenCalledWith('add_directory', {
        path: '/test/path',
        name: 'Test Directory'
      });
    });

    it('should handle empty name', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await addDirectory('/test/path', '');

      expect(mockInvoke).toHaveBeenCalledWith('add_directory', {
        path: '/test/path',
        name: ''
      });
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to add directory');
      mockInvoke.mockRejectedValue(error);

      await expect(addDirectory('/test/path', 'Test')).rejects.toThrow('Failed to add directory');
    });
  });

  describe('removeDirectory', () => {
    it('should call invoke with correct command and parameters', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await removeDirectory('test-id');

      expect(mockInvoke).toHaveBeenCalledWith('remove_directory', {
        id: 'test-id'
      });
    });

    it('should handle empty id', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await removeDirectory('');

      expect(mockInvoke).toHaveBeenCalledWith('remove_directory', {
        id: ''
      });
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to remove directory');
      mockInvoke.mockRejectedValue(error);

      await expect(removeDirectory('test-id')).rejects.toThrow('Failed to remove directory');
    });
  });

  describe('rescanDirectory', () => {
    it('should call invoke with correct command and parameters', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await rescanDirectory('dir-id');

      expect(mockInvoke).toHaveBeenCalledWith('rescan_directory', {
        directoryId: 'dir-id'
      });
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to rescan directory');
      mockInvoke.mockRejectedValue(error);

      await expect(rescanDirectory('dir-id')).rejects.toThrow('Failed to rescan directory');
    });
  });
});
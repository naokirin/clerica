import { describe, it, expect, vi, beforeEach } from 'vitest';
import { ClericaAPI } from './api';

// Tauri API をモック
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

const mockInvoke = vi.mocked(invoke);

describe('ClericaAPI', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Directory operations', () => {
    it('should add directory', async () => {
      const mockDirectory = {
        id: '1',
        path: '/test/path',
        name: 'test',
        created_at: '2023-01-01',
        updated_at: '2023-01-01'
      };

      mockInvoke.mockResolvedValue(mockDirectory);

      const result = await ClericaAPI.addDirectory('/test/path', 'test');

      expect(mockInvoke).toHaveBeenCalledWith('add_directory', {
        path: '/test/path',
        name: 'test'
      });
      expect(result).toEqual(mockDirectory);
    });

    it('should remove directory', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await ClericaAPI.removeDirectory('test-id');

      expect(mockInvoke).toHaveBeenCalledWith('remove_directory', {
        id: 'test-id'
      });
    });

    it('should get directories', async () => {
      const mockDirectories = [
        { id: '1', path: '/path1', name: 'dir1', created_at: '2023-01-01', updated_at: '2023-01-01' },
        { id: '2', path: '/path2', name: 'dir2', created_at: '2023-01-01', updated_at: '2023-01-01' }
      ];

      mockInvoke.mockResolvedValue(mockDirectories);

      const result = await ClericaAPI.getDirectories();

      expect(mockInvoke).toHaveBeenCalledWith('get_directories');
      expect(result).toEqual(mockDirectories);
    });
  });

  describe('File operations', () => {
    it('should get files', async () => {
      const mockFiles = [
        {
          id: '1',
          path: '/test/file.txt',
          name: 'file.txt',
          directory_id: 'dir1',
          size: 1024,
          file_type: 'txt',
          created_at: '2023-01-01',
          modified_at: '2023-01-01',
          birth_time: null,
          inode: 12345,
          is_directory: false,
          created_at_db: '2023-01-01',
          updated_at_db: '2023-01-01'
        }
      ];

      mockInvoke.mockResolvedValue(mockFiles);

      const result = await ClericaAPI.getFiles();

      expect(mockInvoke).toHaveBeenCalledWith('get_files');
      expect(result).toEqual(mockFiles);
    });

    it('should get files by directory', async () => {
      const mockFiles = [];
      mockInvoke.mockResolvedValue(mockFiles);

      const result = await ClericaAPI.getFilesByDirectory('dir1');

      expect(mockInvoke).toHaveBeenCalledWith('get_files_by_directory', {
        directoryId: 'dir1'
      });
      expect(result).toEqual(mockFiles);
    });

    it('should update file tags', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await ClericaAPI.updateFileTags('file1', ['tag1', 'tag2']);

      expect(mockInvoke).toHaveBeenCalledWith('update_file_tags', {
        fileId: 'file1',
        tagIds: ['tag1', 'tag2']
      });
    });

    it('should get file info', async () => {
      const mockFile = {
        id: 'file1',
        path: '/test/file.txt',
        name: 'file.txt',
        directory_id: 'dir1',
        size: 1024,
        file_type: 'txt',
        created_at: '2023-01-01',
        modified_at: '2023-01-01',
        birth_time: null,
        inode: 12345,
        is_directory: false,
        created_at_db: '2023-01-01',
        updated_at_db: '2023-01-01'
      };

      mockInvoke.mockResolvedValue(mockFile);

      const result = await ClericaAPI.getFileInfo('file1');

      expect(mockInvoke).toHaveBeenCalledWith('get_file_info', {
        fileId: 'file1'
      });
      expect(result).toEqual(mockFile);
    });

    it('should delete file', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await ClericaAPI.deleteFile('file1');

      expect(mockInvoke).toHaveBeenCalledWith('delete_file', {
        fileId: 'file1'
      });
    });

    it('should move file', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await ClericaAPI.moveFile('file1', '/new/path');

      expect(mockInvoke).toHaveBeenCalledWith('move_file', {
        fileId: 'file1',
        newPath: '/new/path'
      });
    });
  });

  describe('Search operations', () => {
    it('should search files', async () => {
      const mockResults = [
        {
          file: {
            id: 'file1',
            path: '/test/file.txt',
            name: 'file.txt',
            directory_id: 'dir1',
            size: 1024,
            file_type: 'txt',
            created_at: '2023-01-01',
            modified_at: '2023-01-01',
            birth_time: null,
            inode: 12345,
            is_directory: false,
            created_at_db: '2023-01-01',
            updated_at_db: '2023-01-01'
          },
          tags: [],
          score: 1.0
        }
      ];

      mockInvoke.mockResolvedValue(mockResults);

      const result = await ClericaAPI.searchFiles('test', ['tag1'], 'name');

      expect(mockInvoke).toHaveBeenCalledWith('search_files', {
        query: 'test',
        tag_ids: ['tag1'],
        sort_by: 'name'
      });
      expect(result).toEqual(mockResults);
    });

    it('should search files with minimal parameters', async () => {
      const mockResults = [];
      mockInvoke.mockResolvedValue(mockResults);

      const result = await ClericaAPI.searchFiles('test');

      expect(mockInvoke).toHaveBeenCalledWith('search_files', {
        query: 'test',
        tag_ids: undefined,
        sort_by: undefined
      });
      expect(result).toEqual(mockResults);
    });
  });

  describe('Tag operations', () => {
    it('should get tags', async () => {
      const mockTags = [
        { id: '1', name: 'important', color: '#ff0000', created_at: '2023-01-01' },
        { id: '2', name: 'work', color: '#00ff00', created_at: '2023-01-01' }
      ];

      mockInvoke.mockResolvedValue(mockTags);

      const result = await ClericaAPI.getTags();

      expect(mockInvoke).toHaveBeenCalledWith('get_tags');
      expect(result).toEqual(mockTags);
    });

    it('should create tag', async () => {
      const mockTag = { id: '1', name: 'test', color: '#ff0000', created_at: '2023-01-01' };
      mockInvoke.mockResolvedValue(mockTag);

      const result = await ClericaAPI.createTag('test', '#ff0000');

      expect(mockInvoke).toHaveBeenCalledWith('create_tag', {
        name: 'test',
        color: '#ff0000'
      });
      expect(result).toEqual(mockTag);
    });

    it('should delete tag', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await ClericaAPI.deleteTag('tag1');

      expect(mockInvoke).toHaveBeenCalledWith('delete_tag', {
        id: 'tag1'
      });
    });
  });

  describe('File watching operations', () => {
    it('should start watching', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await ClericaAPI.startWatching('dir1');

      expect(mockInvoke).toHaveBeenCalledWith('start_watching', {
        directoryId: 'dir1'
      });
    });

    it('should stop watching', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await ClericaAPI.stopWatching('dir1');

      expect(mockInvoke).toHaveBeenCalledWith('stop_watching', {
        directoryId: 'dir1'
      });
    });
  });

  describe('Error handling', () => {
    it('should propagate errors from invoke', async () => {
      const error = new Error('Backend error');
      mockInvoke.mockRejectedValue(error);

      await expect(ClericaAPI.getDirectories()).rejects.toThrow('Backend error');
    });

    it('should handle network errors', async () => {
      mockInvoke.mockRejectedValue(new Error('Network error'));

      await expect(ClericaAPI.searchFiles('test')).rejects.toThrow('Network error');
    });
  });
});
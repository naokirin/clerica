import { describe, it, expect, vi, beforeEach } from 'vitest';
import { searchFiles } from '../../lib/api/search.js';
import type { MetadataSearchFilter } from '../../lib/types.js';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

const { invoke: mockInvoke } = vi.mocked(await import('@tauri-apps/api/core'));

describe('search API', () => {
  beforeEach(() => {
    mockInvoke.mockClear();
  });

  describe('searchFiles', () => {
    it('should call invoke with correct command and parameters', async () => {
      const mockResults = [
        {
          file: {
            id: '1',
            name: 'test.txt',
            path: '/test/test.txt',
            size: 1024,
            created_at: '2024-01-01',
            modified_at: '2024-01-01'
          },
          score: 0.9,
          matches: ['test']
        }
      ];
      mockInvoke.mockResolvedValue(mockResults);

      const query = 'test query';
      const tagIds = ['tag1', 'tag2'];
      const metadataFilters: MetadataSearchFilter[] = [
        { field: 'size', operator: 'greater_than', value: '1000' }
      ];

      const result = await searchFiles(query, tagIds, metadataFilters);

      expect(mockInvoke).toHaveBeenCalledWith('search_files', {
        query,
        tagIds,
        metadataFilters,
        metadataLogic: 'AND',
        directoryId: undefined
      });
      expect(result).toEqual(mockResults);
    });

    it('should handle empty query', async () => {
      mockInvoke.mockResolvedValue([]);

      const result = await searchFiles('', [], []);

      expect(mockInvoke).toHaveBeenCalledWith('search_files', {
        query: '',
        tagIds: [],
        metadataFilters: [],
        metadataLogic: 'AND',
        directoryId: undefined
      });
      expect(result).toEqual([]);
    });

    it('should handle search with only tags', async () => {
      const mockResults = [
        {
          file: {
            id: '1',
            name: 'tagged.txt',
            path: '/test/tagged.txt',
            size: 2048,
            created_at: '2024-01-01',
            modified_at: '2024-01-01'
          },
          score: 1.0,
          matches: []
        }
      ];
      mockInvoke.mockResolvedValue(mockResults);

      const result = await searchFiles('', ['important'], []);

      expect(mockInvoke).toHaveBeenCalledWith('search_files', {
        query: '',
        tagIds: ['important'],
        metadataFilters: [],
        metadataLogic: 'AND',
        directoryId: undefined
      });
      expect(result).toEqual(mockResults);
    });

    it('should handle search with metadata filters only', async () => {
      mockInvoke.mockResolvedValue([]);
      
      const metadataFilters: MetadataSearchFilter[] = [
        { field: 'created_at', operator: 'after', value: '2024-01-01' },
        { field: 'size', operator: 'less_than', value: '5000' }
      ];

      await searchFiles('', [], metadataFilters);

      expect(mockInvoke).toHaveBeenCalledWith('search_files', {
        query: '',
        tagIds: [],
        metadataFilters,
        metadataLogic: 'AND',
        directoryId: undefined
      });
    });

    it('should handle complex search with all parameters', async () => {
      const mockResults = [
        {
          file: {
            id: '1',
            name: 'complex.pdf',
            path: '/docs/complex.pdf',
            size: 15000,
            created_at: '2024-01-15',
            modified_at: '2024-01-20'
          },
          score: 0.8,
          matches: ['complex', 'document']
        }
      ];
      mockInvoke.mockResolvedValue(mockResults);

      const query = 'complex document';
      const tagIds = ['documents', 'important'];
      const metadataFilters: MetadataSearchFilter[] = [
        { field: 'size', operator: 'greater_than', value: '10000' },
        { field: 'created_at', operator: 'after', value: '2024-01-01' }
      ];

      const result = await searchFiles(query, tagIds, metadataFilters);

      expect(mockInvoke).toHaveBeenCalledWith('search_files', {
        query,
        tagIds,
        metadataFilters,
        metadataLogic: 'AND',
        directoryId: undefined
      });
      expect(result).toEqual(mockResults);
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Search failed');
      mockInvoke.mockRejectedValue(error);

      await expect(searchFiles('test', [], [])).rejects.toThrow('Search failed');
    });

    it('should handle special characters in query', async () => {
      mockInvoke.mockResolvedValue([]);
      
      const specialQuery = 'test-file_name.txt @#$%';
      
      await searchFiles(specialQuery, [], []);

      expect(mockInvoke).toHaveBeenCalledWith('search_files', {
        query: specialQuery,
        tagIds: [],
        metadataFilters: [],
        metadataLogic: 'AND',
        directoryId: undefined
      });
    });
  });
});
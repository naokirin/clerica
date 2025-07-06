import { describe, it, expect, vi, beforeEach } from 'vitest';
import { getTags, createTag, getCustomMetadataKeys } from '../../lib/api/tags';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

const { invoke: mockInvoke } = vi.mocked(await import('@tauri-apps/api/core'));

describe('tags API', () => {
  beforeEach(() => {
    mockInvoke.mockClear();
  });

  describe('getTags', () => {
    it('should call invoke with correct command', async () => {
      const mockTags = [
        { id: '1', name: 'Important', color: '#ff0000', createdAt: '2024-01-01' },
        { id: '2', name: 'Work', color: '#00ff00', createdAt: '2024-01-02' }
      ];
      mockInvoke.mockResolvedValue(mockTags);

      const result = await getTags();

      expect(mockInvoke).toHaveBeenCalledWith('get_tags');
      expect(result).toEqual(mockTags);
    });

    it('should handle empty tags list', async () => {
      mockInvoke.mockResolvedValue([]);

      const result = await getTags();

      expect(result).toEqual([]);
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to get tags');
      mockInvoke.mockRejectedValue(error);

      await expect(getTags()).rejects.toThrow('Failed to get tags');
    });
  });

  describe('createTag', () => {
    it('should call invoke with correct command and parameters', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await createTag('New Tag', '#0066cc');

      expect(mockInvoke).toHaveBeenCalledWith('create_tag', {
        name: 'New Tag',
        color: '#0066cc'
      });
    });

    it('should handle tag with special characters', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await createTag('Project #1 - Main', '#ff6600');

      expect(mockInvoke).toHaveBeenCalledWith('create_tag', {
        name: 'Project #1 - Main',
        color: '#ff6600'
      });
    });

    it('should handle empty name', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await createTag('', '#000000');

      expect(mockInvoke).toHaveBeenCalledWith('create_tag', {
        name: '',
        color: '#000000'
      });
    });

    it('should handle different color formats', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await createTag('Test', 'red');

      expect(mockInvoke).toHaveBeenCalledWith('create_tag', {
        name: 'Test',
        color: 'red'
      });
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to create tag');
      mockInvoke.mockRejectedValue(error);

      await expect(createTag('Test', '#ff0000')).rejects.toThrow('Failed to create tag');
    });
  });

  describe('getCustomMetadataKeys', () => {
    it('should call invoke with correct command', async () => {
      const mockKeys = [
        { id: '1', name: 'priority', dataType: 'string', createdAt: '2024-01-01' },
        { id: '2', name: 'rating', dataType: 'number', createdAt: '2024-01-02' }
      ];
      mockInvoke.mockResolvedValue(mockKeys);

      const result = await getCustomMetadataKeys();

      expect(mockInvoke).toHaveBeenCalledWith('get_custom_metadata_keys');
      expect(result).toEqual(mockKeys);
    });

    it('should handle empty metadata keys list', async () => {
      mockInvoke.mockResolvedValue([]);

      const result = await getCustomMetadataKeys();

      expect(result).toEqual([]);
    });

    it('should handle different data types', async () => {
      const mockKeys = [
        { id: '1', name: 'text_field', dataType: 'string', createdAt: '2024-01-01' },
        { id: '2', name: 'number_field', dataType: 'number', createdAt: '2024-01-01' },
        { id: '3', name: 'date_field', dataType: 'date', createdAt: '2024-01-01' },
        { id: '4', name: 'boolean_field', dataType: 'boolean', createdAt: '2024-01-01' }
      ];
      mockInvoke.mockResolvedValue(mockKeys);

      const result = await getCustomMetadataKeys();

      expect(result).toEqual(mockKeys);
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to get metadata keys');
      mockInvoke.mockRejectedValue(error);

      await expect(getCustomMetadataKeys()).rejects.toThrow('Failed to get metadata keys');
    });
  });
});
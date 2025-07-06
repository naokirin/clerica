import { describe, it, expect, vi, beforeEach } from 'vitest';
import {
  getCustomMetadataValuesByFile,
  setCustomMetadataValue,
  deleteCustomMetadataValue,
  createCustomMetadataKey,
  updateCustomMetadataKey,
  deleteCustomMetadataKey
} from '../../lib/api/metadata';
import type {
  SetCustomMetadataValueRequest,
  CreateCustomMetadataKeyRequest,
  UpdateCustomMetadataKeyRequest
} from '../../lib/types';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

const { invoke: mockInvoke } = vi.mocked(await import('@tauri-apps/api/core'));

describe('metadata API', () => {
  beforeEach(() => {
    mockInvoke.mockClear();
  });

  describe('getCustomMetadataValuesByFile', () => {
    it('should call invoke with correct command and parameters', async () => {
      const mockValues = [
        { id: '1', fileId: 'file1', keyId: 'key1', value: 'test value', createdAt: '2024-01-01' }
      ];
      mockInvoke.mockResolvedValue(mockValues);

      const result = await getCustomMetadataValuesByFile('file1');

      expect(mockInvoke).toHaveBeenCalledWith('get_custom_metadata_values_by_file', {
        fileId: 'file1'
      });
      expect(result).toEqual(mockValues);
    });

    it('should handle empty metadata values', async () => {
      mockInvoke.mockResolvedValue([]);

      const result = await getCustomMetadataValuesByFile('file1');

      expect(result).toEqual([]);
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to get metadata values');
      mockInvoke.mockRejectedValue(error);

      await expect(getCustomMetadataValuesByFile('file1')).rejects.toThrow('Failed to get metadata values');
    });
  });

  describe('setCustomMetadataValue', () => {
    it('should call invoke with correct command and parameters', async () => {
      mockInvoke.mockResolvedValue(undefined);

      const request: SetCustomMetadataValueRequest = {
        fileId: 'file1',
        keyId: 'key1',
        value: 'new value'
      };

      await setCustomMetadataValue(request);

      expect(mockInvoke).toHaveBeenCalledWith('set_custom_metadata_value', { request });
    });

    it('should handle different value types', async () => {
      mockInvoke.mockResolvedValue(undefined);

      const requests = [
        { fileId: 'file1', keyId: 'text_key', value: 'string value' },
        { fileId: 'file1', keyId: 'number_key', value: '42' },
        { fileId: 'file1', keyId: 'date_key', value: '2024-01-01' },
        { fileId: 'file1', keyId: 'boolean_key', value: 'true' }
      ];

      for (const request of requests) {
        await setCustomMetadataValue(request);
        expect(mockInvoke).toHaveBeenCalledWith('set_custom_metadata_value', { request });
      }
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to set metadata value');
      mockInvoke.mockRejectedValue(error);

      const request: SetCustomMetadataValueRequest = {
        fileId: 'file1',
        keyId: 'key1',
        value: 'value'
      };

      await expect(setCustomMetadataValue(request)).rejects.toThrow('Failed to set metadata value');
    });
  });

  describe('deleteCustomMetadataValue', () => {
    it('should call invoke with correct command and parameters', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await deleteCustomMetadataValue('file1', 'key1');

      expect(mockInvoke).toHaveBeenCalledWith('delete_custom_metadata_value', {
        fileId: 'file1',
        keyId: 'key1'
      });
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to delete metadata value');
      mockInvoke.mockRejectedValue(error);

      await expect(deleteCustomMetadataValue('file1', 'key1')).rejects.toThrow('Failed to delete metadata value');
    });
  });

  describe('createCustomMetadataKey', () => {
    it('should call invoke with correct command and parameters', async () => {
      mockInvoke.mockResolvedValue(undefined);

      const request: CreateCustomMetadataKeyRequest = {
        name: 'priority',
        dataType: 'string'
      };

      await createCustomMetadataKey(request);

      expect(mockInvoke).toHaveBeenCalledWith('create_custom_metadata_key', { request });
    });

    it('should handle different data types', async () => {
      mockInvoke.mockResolvedValue(undefined);

      const requests: CreateCustomMetadataKeyRequest[] = [
        { name: 'text_field', dataType: 'string' },
        { name: 'number_field', dataType: 'number' },
        { name: 'date_field', dataType: 'date' },
        { name: 'boolean_field', dataType: 'boolean' }
      ];

      for (const request of requests) {
        await createCustomMetadataKey(request);
        expect(mockInvoke).toHaveBeenCalledWith('create_custom_metadata_key', { request });
      }
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to create metadata key');
      mockInvoke.mockRejectedValue(error);

      const request: CreateCustomMetadataKeyRequest = {
        name: 'test_key',
        dataType: 'string'
      };

      await expect(createCustomMetadataKey(request)).rejects.toThrow('Failed to create metadata key');
    });
  });

  describe('updateCustomMetadataKey', () => {
    it('should call invoke with correct command and parameters', async () => {
      mockInvoke.mockResolvedValue(undefined);

      const request: UpdateCustomMetadataKeyRequest = {
        name: 'updated_priority'
      };

      await updateCustomMetadataKey(request);

      expect(mockInvoke).toHaveBeenCalledWith('update_custom_metadata_key', { request });
    });

    it('should handle partial updates', async () => {
      mockInvoke.mockResolvedValue(undefined);

      const request: UpdateCustomMetadataKeyRequest = {
        display_name: 'new_name'
      };

      await updateCustomMetadataKey(request);

      expect(mockInvoke).toHaveBeenCalledWith('update_custom_metadata_key', { request });
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to update metadata key');
      mockInvoke.mockRejectedValue(error);

      const request: UpdateCustomMetadataKeyRequest = {
        display_name: 'updated_name'
      };

      await expect(updateCustomMetadataKey(request)).rejects.toThrow('Failed to update metadata key');
    });
  });

  describe('deleteCustomMetadataKey', () => {
    it('should call invoke with correct command and parameters', async () => {
      mockInvoke.mockResolvedValue(undefined);

      await deleteCustomMetadataKey('key1');

      expect(mockInvoke).toHaveBeenCalledWith('delete_custom_metadata_key', {
        keyId: 'key1'
      });
    });

    it('should propagate errors from invoke', async () => {
      const error = new Error('Failed to delete metadata key');
      mockInvoke.mockRejectedValue(error);

      await expect(deleteCustomMetadataKey('key1')).rejects.toThrow('Failed to delete metadata key');
    });
  });
});
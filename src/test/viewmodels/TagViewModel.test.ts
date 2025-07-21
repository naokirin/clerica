import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { get } from 'svelte/store';
import { TagViewModel } from '../../lib/viewmodels/TagViewModel';
import type { 
  Tag, 
  CustomMetadataKey, 
  CreateCustomMetadataKeyRequest, 
  UpdateCustomMetadataKeyRequest 
} from '../../lib/types';

// Mock APIs
vi.mock('../../lib/api/tags', () => ({
  getTags: vi.fn(),
  createTag: vi.fn(),
  getCustomMetadataKeys: vi.fn()
}));

vi.mock('../../lib/api/metadata', () => ({
  createCustomMetadataKey: vi.fn(),
  updateCustomMetadataKey: vi.fn(),
  deleteCustomMetadataKey: vi.fn()
}));

const { getTags: mockGetTags, createTag: mockCreateTag, getCustomMetadataKeys: mockGetCustomMetadataKeys } = vi.mocked(await import('../../lib/api/tags'));
const { createCustomMetadataKey: mockCreateCustomMetadataKey, updateCustomMetadataKey: mockUpdateCustomMetadataKey, deleteCustomMetadataKey: mockDeleteCustomMetadataKey } = vi.mocked(await import('../../lib/api/metadata'));

describe('TagViewModel', () => {
  let tagViewModel: TagViewModel;
  
  const mockTags: Tag[] = [
    {
      id: '1',
      name: 'Important',
      color: '#ff0000',
      created_at: '2024-01-01'
    },
    {
      id: '2',
      name: 'Work',
      color: '#00ff00',
      created_at: '2024-01-02'
    }
  ];

  const mockMetadataKeys: CustomMetadataKey[] = [
    {
      id: '1',
      name: 'priority',
      display_name: 'Priority',
      data_type: 'text',
      description: null,
      is_required: false,
      default_value: null,
      validation_pattern: null,
      created_at: '2024-01-01',
      updated_at: '2024-01-01'
    },
    {
      id: '2',
      name: 'rating',
      display_name: 'Rating',
      data_type: 'number',
      description: null,
      is_required: false,
      default_value: null,
      validation_pattern: null,
      created_at: '2024-01-02',
      updated_at: '2024-01-02'
    }
  ];

  beforeEach(() => {
    vi.clearAllMocks();
    mockGetTags.mockResolvedValue(mockTags);
    mockGetCustomMetadataKeys.mockResolvedValue(mockMetadataKeys);
  });

  afterEach(() => {
    tagViewModel?.dispose();
  });

  describe('constructor', () => {
    it('should initialize and load data', async () => {
      tagViewModel = new TagViewModel();
      
      // Wait for async initialization
      await new Promise(resolve => setTimeout(resolve, 0));
      
      expect(mockGetTags).toHaveBeenCalled();
      expect(mockGetCustomMetadataKeys).toHaveBeenCalled();
    });
  });

  describe('loadTags', () => {
    beforeEach(() => {
      tagViewModel = new TagViewModel();
    });

    it('should load tags successfully', async () => {
      await tagViewModel.loadTags();
      
      expect(mockGetTags).toHaveBeenCalled();
      expect(get(tagViewModel.tags)).toEqual(mockTags);
    });

    it('should handle load tags error', async () => {
      const error = new Error('Load tags failed');
      mockGetTags.mockRejectedValue(error);
      
      await tagViewModel.loadTags();
      
      expect(get(tagViewModel.error)).toBeTruthy();
    });
  });

  describe('loadCustomMetadataKeys', () => {
    beforeEach(() => {
      tagViewModel = new TagViewModel();
    });

    it('should load metadata keys successfully', async () => {
      await tagViewModel.loadCustomMetadataKeys();
      
      expect(mockGetCustomMetadataKeys).toHaveBeenCalled();
      expect(get(tagViewModel.customMetadataKeys)).toEqual(mockMetadataKeys);
    });

    it('should handle load metadata keys error', async () => {
      const error = new Error('Load metadata keys failed');
      mockGetCustomMetadataKeys.mockRejectedValue(error);
      
      await tagViewModel.loadCustomMetadataKeys();
      
      expect(get(tagViewModel.error)).toBeTruthy();
    });
  });

  describe('createNewTag', () => {
    beforeEach(() => {
      tagViewModel = new TagViewModel();
    });

    it('should create tag with default color', async () => {
      const newTag = { id: '3', name: 'New Tag', color: '#3B82F6', created_at: '2024-01-03' };
      mockCreateTag.mockResolvedValue(newTag);
      
      const result = await tagViewModel.createNewTag('New Tag');
      
      expect(mockCreateTag).toHaveBeenCalledWith('New Tag', '#3B82F6');
      expect(mockGetTags).toHaveBeenCalledTimes(2); // initial load + reload
      expect(result).toBe(true);
    });

    it('should create tag with custom color', async () => {
      const newTag = { id: '3', name: 'New Tag', color: '#3B82F6', created_at: '2024-01-03' };
      mockCreateTag.mockResolvedValue(newTag);
      
      const result = await tagViewModel.createNewTag('Colored Tag', '#ff6600');
      
      expect(mockCreateTag).toHaveBeenCalledWith('Colored Tag', '#ff6600');
      expect(result).toBe(true);
    });

    it('should handle create tag error', async () => {
      const error = new Error('Create tag failed');
      mockCreateTag.mockRejectedValue(error);
      
      const result = await tagViewModel.createNewTag('Failed Tag');
      
      expect(result).toBe(false);
      expect(get(tagViewModel.error)).toBeTruthy();
    });
  });

  describe('createNewCustomMetadataKey', () => {
    beforeEach(() => {
      tagViewModel = new TagViewModel();
    });

    it('should create metadata key successfully', async () => {
      mockCreateCustomMetadataKey.mockResolvedValue(undefined);
      
      const request: CreateCustomMetadataKeyRequest = {
        name: 'priority',
        display_name: 'Priority',
        data_type: 'text',
        description: null,
        is_required: false,
        default_value: null,
        validation_pattern: null
      };
      
      const result = await tagViewModel.createNewCustomMetadataKey(request);
      
      expect(mockCreateCustomMetadataKey).toHaveBeenCalledWith(request);
      expect(mockGetCustomMetadataKeys).toHaveBeenCalledTimes(2); // initial load + reload
      expect(result).toBe(true);
    });

    it('should handle different data types', async () => {
      mockCreateCustomMetadataKey.mockResolvedValue(undefined);
      
      const requests: CreateCustomMetadataKeyRequest[] = [
        { name: 'text_field', display_name: 'Text Field', data_type: 'text', description: null, is_required: false, default_value: null, validation_pattern: null },
        { name: 'number_field', display_name: 'Number Field', data_type: 'number', description: null, is_required: false, default_value: null, validation_pattern: null },
        { name: 'date_field', display_name: 'Date Field', data_type: 'date', description: null, is_required: false, default_value: null, validation_pattern: null },
        { name: 'boolean_field', display_name: 'Boolean Field', data_type: 'boolean', description: null, is_required: false, default_value: null, validation_pattern: null }
      ];
      
      for (const request of requests) {
        const result = await tagViewModel.createNewCustomMetadataKey(request);
        expect(result).toBe(true);
        expect(mockCreateCustomMetadataKey).toHaveBeenCalledWith(request);
      }
    });

    it('should handle create metadata key error', async () => {
      const error = new Error('Create metadata key failed');
      mockCreateCustomMetadataKey.mockRejectedValue(error);
      
      const request: CreateCustomMetadataKeyRequest = {
        name: 'failed_key',
        display_name: 'Failed Key',
        data_type: 'text',
        description: null,
        is_required: false,
        default_value: null,
        validation_pattern: null
      };
      
      const result = await tagViewModel.createNewCustomMetadataKey(request);
      
      expect(result).toBe(false);
      expect(get(tagViewModel.error)).toBeTruthy();
    });
  });

  describe('updateExistingCustomMetadataKey', () => {
    beforeEach(() => {
      tagViewModel = new TagViewModel();
    });

    it('should update metadata key successfully', async () => {
      mockUpdateCustomMetadataKey.mockResolvedValue(undefined);
      
      const request: UpdateCustomMetadataKeyRequest = {
        id: '1',
        display_name: 'Updated Priority',
        data_type: 'text',
        description: null,
        is_required: false,
        default_value: null,
        validation_pattern: null
      };
      
      const result = await tagViewModel.updateExistingCustomMetadataKey(request);
      
      expect(mockUpdateCustomMetadataKey).toHaveBeenCalledWith(request);
      expect(mockGetCustomMetadataKeys).toHaveBeenCalledTimes(2); // initial load + reload
      expect(result).toBe(true);
    });

    it('should handle update metadata key error', async () => {
      const error = new Error('Update metadata key failed');
      mockUpdateCustomMetadataKey.mockRejectedValue(error);
      
      const request: UpdateCustomMetadataKeyRequest = {
        id: '1',
        display_name: 'Failed Update',
        data_type: 'text',
        description: null,
        is_required: false,
        default_value: null,
        validation_pattern: null
      };
      
      const result = await tagViewModel.updateExistingCustomMetadataKey(request);
      
      expect(result).toBe(false);
      expect(get(tagViewModel.error)).toBeTruthy();
    });
  });

  describe('deleteExistingCustomMetadataKey', () => {
    beforeEach(() => {
      tagViewModel = new TagViewModel();
    });

    it('should delete metadata key successfully', async () => {
      mockDeleteCustomMetadataKey.mockResolvedValue(undefined);
      
      const result = await tagViewModel.deleteExistingCustomMetadataKey('key1');
      
      expect(mockDeleteCustomMetadataKey).toHaveBeenCalledWith('key1');
      expect(mockGetCustomMetadataKeys).toHaveBeenCalledTimes(2); // initial load + reload
      expect(result).toBe(true);
    });

    it('should handle delete metadata key error', async () => {
      const error = new Error('Delete metadata key failed');
      mockDeleteCustomMetadataKey.mockRejectedValue(error);
      
      const result = await tagViewModel.deleteExistingCustomMetadataKey('key1');
      
      expect(result).toBe(false);
      expect(get(tagViewModel.error)).toBeTruthy();
    });
  });

  describe('store reactivity', () => {
    beforeEach(() => {
      tagViewModel = new TagViewModel();
    });

    it('should notify subscribers when tags change', () => {
      const values: Tag[][] = [];
      tagViewModel.tags.subscribe(value => values.push([...value]));
      
      tagViewModel['_tags'].set(mockTags);
      
      expect(values).toHaveLength(2);
      expect(values[1]).toEqual(mockTags);
    });

    it('should notify subscribers when metadata keys change', () => {
      const values: CustomMetadataKey[][] = [];
      tagViewModel.customMetadataKeys.subscribe(value => values.push([...value]));
      
      tagViewModel['_customMetadataKeys'].set(mockMetadataKeys);
      
      expect(values).toHaveLength(2);
      expect(values[1]).toEqual(mockMetadataKeys);
    });
  });

  describe('integration scenarios', () => {
    beforeEach(() => {
      tagViewModel = new TagViewModel();
    });

    it('should handle concurrent operations', async () => {
      const newTag = { id: '3', name: 'New Tag', color: '#3B82F6', created_at: '2024-01-03' };
      mockCreateTag.mockResolvedValue(newTag);
      mockCreateCustomMetadataKey.mockResolvedValue(undefined);
      
      const tagPromise = tagViewModel.createNewTag('Concurrent Tag');
      const metadataPromise = tagViewModel.createNewCustomMetadataKey({
        name: 'concurrent_key',
        display_name: 'Concurrent Key',
        data_type: 'text',
        description: null,
        is_required: false,
        default_value: null,
        validation_pattern: null
      });
      
      const [tagResult, metadataResult] = await Promise.all([tagPromise, metadataPromise]);
      
      expect(tagResult).toBe(true);
      expect(metadataResult).toBe(true);
    });

    it('should maintain state consistency after operations', async () => {
      const newTag = { id: '3', name: 'New Tag', color: '#3B82F6', created_at: '2024-01-03' };
      mockCreateTag.mockResolvedValue(newTag);
      const updatedTags = [...mockTags, { id: '3', name: 'New Tag', color: '#3B82F6', created_at: '2024-01-03' }];
      mockGetTags.mockResolvedValue(updatedTags);
      
      await tagViewModel.createNewTag('New Tag');
      
      expect(get(tagViewModel.tags)).toEqual(updatedTags);
    });
  });
});
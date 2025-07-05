import { writable, type Writable } from 'svelte/store';
import { BaseViewModel } from './BaseViewModel.js';
import { getTags, createTag, getCustomMetadataKeys, getTopTags, searchTagsByName } from '../api/tags.js';
import {
  createCustomMetadataKey,
  updateCustomMetadataKey,
  deleteCustomMetadataKey
} from '../api/metadata.js';
import type {
  Tag,
  CustomMetadataKey,
  CreateCustomMetadataKeyRequest,
  UpdateCustomMetadataKeyRequest
} from '../types.js';

export class TagViewModel extends BaseViewModel {
  private _tags: Writable<Tag[]> = writable([]);
  private _topTags: Writable<Tag[]> = writable([]);
  private _tagSearchResults: Writable<Tag[]> = writable([]);
  private _customMetadataKeys: Writable<CustomMetadataKey[]> = writable([]);

  public readonly tags = this._tags;
  public readonly topTags = this._topTags;
  public readonly tagSearchResults = this._tagSearchResults;
  public readonly customMetadataKeys = this._customMetadataKeys;

  constructor() {
    super();
    this.loadTags();
    this.loadTopTags();
    this.loadCustomMetadataKeys();
  }

  public async loadTags(): Promise<void> {
    const result = await this.executeAsync(async () => {
      return await getTags();
    });

    if (result) {
      this._tags.set(result);
    }
  }

  public async loadCustomMetadataKeys(): Promise<void> {
    const result = await this.executeAsync(async () => {
      return await getCustomMetadataKeys();
    });

    if (result) {
      this._customMetadataKeys.set(result);
    }
  }

  public async loadTopTags(limit: number = 10): Promise<void> {
    const result = await this.executeAsync(async () => {
      return await getTopTags(limit);
    });

    if (result) {
      this._topTags.set(result);
    }
  }

  public async searchTags(query: string): Promise<void> {
    if (query.trim() === '') {
      this._tagSearchResults.set([]);
      return;
    }

    const result = await this.executeAsync(async () => {
      return await searchTagsByName(query);
    });

    if (result) {
      this._tagSearchResults.set(result);
    }
  }

  public clearTagSearch(): void {
    this._tagSearchResults.set([]);
  }

  public async createNewTag(name: string, color: string = "#3B82F6"): Promise<boolean> {
    const result = await this.executeAsync(async () => {
      await createTag(name, color);
      return true;
    });

    if (result) {
      await this.loadTags();
      await this.loadTopTags(); // 上位タグも更新
      return true;
    }
    return false;
  }

  public async createNewCustomMetadataKey(request: CreateCustomMetadataKeyRequest): Promise<boolean> {
    const result = await this.executeAsync(async () => {
      await createCustomMetadataKey(request);
      return true;
    });

    if (result) {
      await this.loadCustomMetadataKeys();
      return true;
    }
    return false;
  }

  public async updateExistingCustomMetadataKey(
    request: UpdateCustomMetadataKeyRequest
  ): Promise<boolean> {
    const result = await this.executeAsync(async () => {
      await updateCustomMetadataKey(request);
      return true;
    });

    if (result) {
      await this.loadCustomMetadataKeys();
      return true;
    }
    return false;
  }

  public async deleteExistingCustomMetadataKey(keyId: string): Promise<boolean> {
    const result = await this.executeAsync(async () => {
      await deleteCustomMetadataKey(keyId);
      return true;
    });

    if (result) {
      await this.loadCustomMetadataKeys();
      return true;
    }
    return false;
  }
}
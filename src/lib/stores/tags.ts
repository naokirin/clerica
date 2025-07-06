/**
 * タグ管理用ストア
 * タグ一覧の取得、作成、編集、削除を担当
 */

import { writable, derived, type Readable } from 'svelte/store';
import { createCollectionStore, type CollectionStore, type FilterState } from './common';
import { getTags, createTag } from '../api/tags';
import type { Tag } from '../types';

export interface TagFilter extends FilterState {
  colorFilter: string | null;
  usageCount: 'all' | 'used' | 'unused';
}

export interface TagsStore extends CollectionStore<Tag> {
  // タグ専用プロパティ
  tagFilter: Readable<TagFilter>;
  tagsByColor: Readable<Record<string, Tag[]>>;
  recentTags: Readable<Tag[]>;
  
  // タグ専用メソッド
  loadTags(): Promise<void>;
  createTag(name: string, color: string): Promise<Tag | null>;
  updateTag(id: string, name: string, color: string): Promise<void>;
  deleteTag(id: string): Promise<void>;
  setColorFilter(color: string | null): void;
  setUsageFilter(usage: 'all' | 'used' | 'unused'): void;
  findTagsByNames(names: string[]): Tag[];
  getTagById(id: string): Tag | null;
  getMostUsedTags(limit?: number): Tag[];
}

interface TagStoreInternal {
  _tags: Tag[];
  _filter: TagFilter;
  _tagUsageCounts: Record<string, number>;
  _isLoading: boolean;
  _error: string | null;
}

function createTagsStore(): TagsStore {
  // 内部状態
  const internalStore = writable<TagStoreInternal>({
    _tags: [],
    _filter: {
      selectedCategory: null,
      searchQuery: '',
      colorFilter: null,
      usageCount: 'all'
    },
    _tagUsageCounts: {},
    _isLoading: false,
    _error: null
  });

  // タグ専用のフィルタ関数
  const tagFilterPredicate = (tag: Tag, filter: TagFilter): boolean => {
    // 色フィルタ
    if (filter.colorFilter && tag.color !== filter.colorFilter) {
      return false;
    }

    // 使用状況フィルタ
    if (filter.usageCount !== 'all') {
      let usageCount = 0;
      const unsubscribe = internalStore.subscribe($store => {
        usageCount = $store._tagUsageCounts[tag.id] || 0;
      });
      unsubscribe();

      if (filter.usageCount === 'used' && usageCount === 0) {
        return false;
      }
      if (filter.usageCount === 'unused' && usageCount > 0) {
        return false;
      }
    }

    // 検索クエリフィルタ
    if (filter.searchQuery) {
      const query = filter.searchQuery.toLowerCase();
      const nameMatch = tag.name.toLowerCase().includes(query);
      if (!nameMatch) {
        return false;
      }
    }

    return true;
  };

  // タグ専用のソートキー関数
  const tagSortKeyExtractor = (tag: Tag) => {
    // デフォルトは名前順
    return tag.name;
  };

  // 基本のCollectionStoreを作成
  const baseCollection = createCollectionStore<Tag>(
    50, // タグは一度に多く表示する
    tagSortKeyExtractor,
    (tag, filter) => tagFilterPredicate(tag, filter as TagFilter)
  );

  // タグ専用フィルタストア
  const tagFilter = derived(internalStore, $store => $store._filter);

  // 色別タググループ
  const tagsByColor = derived(baseCollection.items, $tags => {
    const grouped: Record<string, Tag[]> = {};
    $tags.forEach(tag => {
      if (!grouped[tag.color]) {
        grouped[tag.color] = [];
      }
      grouped[tag.color].push(tag);
    });
    return grouped;
  });

  // 最近使用されたタグ（使用回数上位）
  const recentTags = derived(
    [baseCollection.items, internalStore],
    ([$tags, $store]) => {
      return $tags
        .map(tag => ({
          tag,
          count: $store._tagUsageCounts[tag.id] || 0
        }))
        .sort((a, b) => b.count - a.count)
        .slice(0, 10)
        .map(item => item.tag);
    }
  );

  return {
    // CollectionStoreのプロパティを継承
    items: baseCollection.items,
    pagination: baseCollection.pagination,
    sort: baseCollection.sort,
    filter: baseCollection.filter,
    paginatedItems: baseCollection.paginatedItems,
    setItems: baseCollection.setItems,

    // タグ専用プロパティ
    tagFilter,
    tagsByColor,
    recentTags,

    // タグ専用メソッド
    async loadTags(): Promise<void> {
      try {
        internalStore.update(store => ({ ...store, _isLoading: true, _error: null }));
        
        const tags = await getTags();
        baseCollection.setItems(tags);
        
        // TODO: タグの使用回数も取得して更新
        // const usageCounts = await getTagUsageCounts();
        // internalStore.update(store => ({ ...store, _tagUsageCounts: usageCounts }));
        
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        internalStore.update(store => ({ ...store, _error: errorMessage }));
      } finally {
        internalStore.update(store => ({ ...store, _isLoading: false }));
      }
    },

    async createTag(name: string, color: string): Promise<Tag | null> {
      try {
        internalStore.update(store => ({ ...store, _isLoading: true, _error: null }));
        
        const newTag = await createTag({ name, color });
        if (newTag) {
          // 楽観的更新：新しいタグをストアに追加
          let currentTags: Tag[] = [];
          const unsubscribe = baseCollection.items.subscribe($tags => {
            currentTags = $tags;
          });
          unsubscribe();
          
          baseCollection.setItems([...currentTags, newTag]);
        }
        
        return newTag;
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        internalStore.update(store => ({ ...store, _error: errorMessage }));
        return null;
      } finally {
        internalStore.update(store => ({ ...store, _isLoading: false }));
      }
    },

    async updateTag(id: string, name: string, color: string): Promise<void> {
      try {
        internalStore.update(store => ({ ...store, _isLoading: true, _error: null }));
        
        await updateTag(id, { name, color });
        
        // 楽観的更新：タグを更新
        let currentTags: Tag[] = [];
        const unsubscribe = baseCollection.items.subscribe($tags => {
          currentTags = $tags;
        });
        unsubscribe();
        
        const updatedTags = currentTags.map(tag => 
          tag.id === id ? { ...tag, name, color } : tag
        );
        baseCollection.setItems(updatedTags);
        
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        internalStore.update(store => ({ ...store, _error: errorMessage }));
        // エラー時はタグ一覧を再読み込み
        await this.loadTags();
      } finally {
        internalStore.update(store => ({ ...store, _isLoading: false }));
      }
    },

    async deleteTag(id: string): Promise<void> {
      try {
        internalStore.update(store => ({ ...store, _isLoading: true, _error: null }));
        
        await deleteTag(id);
        
        // 楽観的更新：タグを削除
        let currentTags: Tag[] = [];
        const unsubscribe = baseCollection.items.subscribe($tags => {
          currentTags = $tags;
        });
        unsubscribe();
        
        const filteredTags = currentTags.filter(tag => tag.id !== id);
        baseCollection.setItems(filteredTags);
        
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        internalStore.update(store => ({ ...store, _error: errorMessage }));
        // エラー時はタグ一覧を再読み込み
        await this.loadTags();
      } finally {
        internalStore.update(store => ({ ...store, _isLoading: false }));
      }
    },

    setColorFilter(color: string | null): void {
      internalStore.update(store => ({
        ...store,
        _filter: { ...store._filter, colorFilter: color }
      }));
    },

    setUsageFilter(usage: 'all' | 'used' | 'unused'): void {
      internalStore.update(store => ({
        ...store,
        _filter: { ...store._filter, usageCount: usage }
      }));
    },

    findTagsByNames(names: string[]): Tag[] {
      let tags: Tag[] = [];
      const unsubscribe = baseCollection.items.subscribe($tags => {
        tags = $tags.filter(tag => names.includes(tag.name));
      });
      unsubscribe();
      return tags;
    },

    getTagById(id: string): Tag | null {
      let foundTag: Tag | null = null;
      const unsubscribe = baseCollection.items.subscribe($tags => {
        foundTag = $tags.find(tag => tag.id === id) || null;
      });
      unsubscribe();
      return foundTag;
    },

    getMostUsedTags(limit: number = 10): Tag[] {
      let mostUsed: Tag[] = [];
      const unsubscribe = recentTags.subscribe($recentTags => {
        mostUsed = $recentTags.slice(0, limit);
      });
      unsubscribe();
      return mostUsed;
    },

    refresh: () => {
      return this.loadTags();
    }
  };
}

// シングルトンストアとしてエクスポート
export const tagsStore = createTagsStore();
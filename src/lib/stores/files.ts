/**
 * ファイル管理用ストア
 * ファイル一覧の取得、フィルタリング、状態管理を担当
 */

import { writable, derived, type Readable } from 'svelte/store';
import { createCollectionStore, type CollectionStore, type FilterState } from './common';
import { settingsService } from '../services/SettingsService';
import { getFiles } from '../api/files';
import type { File, FileWithTags, Tag, FileCategory } from '../types';

// 複数ファイル選択機能の状態管理ストア
export const selectedFileIds = writable<Set<number>>(new Set());

export interface FileFilter extends FilterState {
  selectedDirectory: string | null;
  fileType: FileCategory;
  tagIds: string[];
  hasMetadata: boolean;
}

export interface FilesStore extends CollectionStore<FileWithTags> {
  // 追加のファイル専用プロパティ
  fileFilter: Readable<FileFilter>;
  categoryCounts: Readable<Record<FileCategory, number>>;
  directories: Readable<string[]>;
  
  // ファイル専用メソッド
  loadFiles(directoryId?: string): Promise<void>;
  setDirectory(directoryId: string | null): void;
  setFileType(fileType: FileCategory): void;
  setTagFilter(tagIds: string[]): void;
  setMetadataFilter(hasMetadata: boolean): void;
  updateFileTags(fileId: string, tagIds: string[]): Promise<void>;
  getFilesByCategory(category: FileCategory): FileWithTags[];
}

interface FileStoreInternal {
  _files: FileWithTags[];
  _filter: FileFilter;
  _directories: string[];
  _isLoading: boolean;
  _error: string | null;
}

function createFilesStore(): FilesStore {
  // 内部状態
  const internalStore = writable<FileStoreInternal>({
    _files: [],
    _filter: {
      selectedCategory: null,
      searchQuery: '',
      selectedDirectory: null,
      fileType: 'all',
      tagIds: [],
      hasMetadata: false
    },
    _directories: [],
    _isLoading: false,
    _error: null
  });

  // 設定からitemsPerPageを取得
  const initialItemsPerPage = settingsService.getSetting('itemsPerPage');

  // ファイル専用のフィルタ関数
  const fileFilterPredicate = (file: FileWithTags, filter: FileFilter): boolean => {
    // ディレクトリフィルタ
    if (filter.selectedDirectory && file.file.directory_id !== filter.selectedDirectory) {
      return false;
    }

    // ファイルタイプフィルタ
    if (filter.fileType !== 'all') {
      const category = getFileCategoryFromMimeType(file.file.mime_type || '');
      if (category !== filter.fileType) {
        return false;
      }
    }

    // タグフィルタ
    if (filter.tagIds.length > 0) {
      const fileTagIds = file.tags.map(tag => tag.id);
      const hasAllTags = filter.tagIds.every(tagId => fileTagIds.includes(tagId));
      if (!hasAllTags) {
        return false;
      }
    }

    // メタデータフィルタ
    if (filter.hasMetadata && !file.file.metadata) {
      return false;
    }

    // 検索クエリフィルタ
    if (filter.searchQuery) {
      const query = filter.searchQuery.toLowerCase();
      const nameMatch = file.file.name.toLowerCase().includes(query);
      const tagMatch = file.tags.some(tag => tag.name.toLowerCase().includes(query));
      if (!nameMatch && !tagMatch) {
        return false;
      }
    }

    return true;
  };

  // ファイル専用のソートキー関数
  const fileSortKeyExtractor = (file: FileWithTags) => {
    const sortKey = settingsService.getSetting('defaultSortKey');
    switch (sortKey) {
      case 'name':
        return file.file.name;
      case 'size':
        return file.file.size;
      case 'created_at':
        return file.file.created_at;
      case 'modified_at':
        return file.file.modified_at;
      default:
        return file.file.name;
    }
  };

  // 基本のCollectionStoreを作成
  const baseCollection = createCollectionStore<FileWithTags>(
    initialItemsPerPage,
    fileSortKeyExtractor,
    (file, filter) => fileFilterPredicate(file, filter as FileFilter)
  );

  // ファイル専用フィルタストア
  const fileFilter = derived(internalStore, $store => $store._filter);

  // ディレクトリ一覧
  const directories = derived(internalStore, $store => $store._directories);

  // カテゴリ別ファイル数
  const categoryCounts = derived(baseCollection.items, $files => {
    const counts: Record<FileCategory, number> = {
      all: $files.length,
      image: 0,
      audio: 0,
      video: 0,
      document: 0,
      archive: 0,
      other: 0
    };

    $files.forEach(fileWithTags => {
      const category = getFileCategoryFromMimeType(fileWithTags.file.mime_type || '');
      if (category !== 'all') {
        counts[category]++;
      }
    });

    return counts;
  });

  return {
    // CollectionStoreのプロパティを継承
    items: baseCollection.items,
    pagination: baseCollection.pagination,
    sort: baseCollection.sort,
    filter: baseCollection.filter,
    paginatedItems: baseCollection.paginatedItems,
    setItems: baseCollection.setItems,

    // ファイル専用プロパティ
    fileFilter,
    categoryCounts,
    directories,

    // ファイル専用メソッド
    async loadFiles(directoryId?: string): Promise<void> {
      try {
        internalStore.update(store => ({ ...store, _isLoading: true, _error: null }));
        
        const files = await getFiles(directoryId);
        baseCollection.setItems(files);
        
        // ディレクトリ一覧を更新
        const uniqueDirectories = [...new Set(files.map(f => f.file.directory_id))];
        internalStore.update(store => ({ ...store, _directories: uniqueDirectories }));
        
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        internalStore.update(store => ({ ...store, _error: errorMessage }));
      } finally {
        internalStore.update(store => ({ ...store, _isLoading: false }));
      }
    },

    setDirectory(directoryId: string | null): void {
      internalStore.update(store => ({
        ...store,
        _filter: { ...store._filter, selectedDirectory: directoryId }
      }));
    },

    setFileType(fileType: FileCategory): void {
      internalStore.update(store => ({
        ...store,
        _filter: { ...store._filter, fileType }
      }));
    },

    setTagFilter(tagIds: string[]): void {
      internalStore.update(store => ({
        ...store,
        _filter: { ...store._filter, tagIds }
      }));
    },

    setMetadataFilter(hasMetadata: boolean): void {
      internalStore.update(store => ({
        ...store,
        _filter: { ...store._filter, hasMetadata }
      }));
    },

    async updateFileTags(fileId: string, tagIds: string[]): Promise<void> {
      // TODO: API呼び出しの実装
    },

    getFilesByCategory(category: FileCategory): FileWithTags[] {
      let files: FileWithTags[] = [];
      const unsubscribe = baseCollection.items.subscribe($files => {
        files = $files.filter(fileWithTags => {
          if (category === 'all') return true;
          const fileCategory = getFileCategoryFromMimeType(fileWithTags.file.mime_type || '');
          return fileCategory === category;
        });
      });
      unsubscribe();
      return files;
    },

    refresh: () => {
      // ディレクトリが選択されている場合はそのディレクトリを、そうでなければ全体を再読み込み
      let currentDirectory: string | null = null;
      const unsubscribe = fileFilter.subscribe($filter => {
        currentDirectory = $filter.selectedDirectory;
      });
      unsubscribe();
      
      return baseCollection.refresh(); // TODO: loadFiles(currentDirectory)を呼び出すように修正
    }
  };
}

// ファイルのMIMEタイプからカテゴリを判定する関数
function getFileCategoryFromMimeType(mimeType: string): FileCategory {
  if (!mimeType) return 'other';
  
  if (mimeType.startsWith('image/')) return 'image';
  if (mimeType.startsWith('audio/')) return 'audio';
  if (mimeType.startsWith('video/')) return 'video';
  
  // ドキュメント系
  if (mimeType.includes('pdf') || 
      mimeType.includes('document') ||
      mimeType.includes('text') ||
      mimeType.includes('spreadsheet') ||
      mimeType.includes('presentation')) {
    return 'document';
  }
  
  // アーカイブ系
  if (mimeType.includes('zip') ||
      mimeType.includes('rar') ||
      mimeType.includes('tar') ||
      mimeType.includes('gzip') ||
      mimeType.includes('7z')) {
    return 'archive';
  }
  
  return 'other';
}

// シングルトンストアとしてエクスポート
export const filesStore = createFilesStore();
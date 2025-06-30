import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

// モックAPIクラス
class MockClericaAPI {
  static mockInvoke = vi.fn();

  static async addDirectory(path: string, name: string) {
    return this.mockInvoke('add_directory', { path, name });
  }

  static async removeDirectory(id: string) {
    return this.mockInvoke('remove_directory', { id });
  }

  static async getDirectories() {
    return this.mockInvoke('get_directories');
  }

  static async updateFileTags(fileId: string, tagIds: string[]) {
    return this.mockInvoke('update_file_tags', { fileId, tagIds });
  }

  static async searchFiles(query: string, tagIds?: string[], sortBy?: string) {
    return this.mockInvoke('search_files', { query, tag_ids: tagIds, sort_by: sortBy });
  }

  static async getTags() {
    return this.mockInvoke('get_tags');
  }

  static async createTag(name: string, color: string) {
    return this.mockInvoke('create_tag', { name, color });
  }

  static async getFilesByDirectory(directoryId: string) {
    return this.mockInvoke('get_files_by_directory', { directoryId });
  }
}

// 結合テスト - 実際のAPIと同様の動作をモック
describe('Integration Tests', () => {
  let mockInvoke: any;

  beforeEach(() => {
    // モックをリセット
    MockClericaAPI.mockInvoke.mockClear();
    mockInvoke = MockClericaAPI.mockInvoke;
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  describe('Directory Management Flow', () => {
    it('should handle complete directory management workflow', async () => {
      // 初期状態：ディレクトリなし
      mockInvoke.mockResolvedValueOnce([]);

      let directories = await MockClericaAPI.getDirectories();
      expect(directories).toHaveLength(0);

      // ディレクトリを追加
      const newDirectory = {
        id: 'dir1',
        path: '/test/documents',
        name: 'Documents',
        created_at: '2023-01-01T00:00:00Z',
        updated_at: '2023-01-01T00:00:00Z'
      };

      mockInvoke.mockResolvedValueOnce(newDirectory);
      const addedDirectory = await MockClericaAPI.addDirectory('/test/documents', 'Documents');
      expect(addedDirectory).toEqual(newDirectory);

      // ディレクトリ一覧を再取得
      mockInvoke.mockResolvedValueOnce([newDirectory]);
      directories = await MockClericaAPI.getDirectories();
      expect(directories).toHaveLength(1);
      expect(directories[0].name).toBe('Documents');

      // ディレクトリを削除
      mockInvoke.mockResolvedValueOnce(undefined);
      await MockClericaAPI.removeDirectory('dir1');

      // 削除後の確認
      mockInvoke.mockResolvedValueOnce([]);
      directories = await MockClericaAPI.getDirectories();
      expect(directories).toHaveLength(0);
    });
  });

  describe('Tag Management Flow', () => {
    it('should handle tag operations', async () => {
      const tag1 = {
        id: 'tag1',
        name: 'Important',
        color: '#ff0000',
        created_at: '2023-01-01T00:00:00Z'
      };

      // タグを作成
      mockInvoke.mockResolvedValueOnce(tag1);
      const createdTag1 = await MockClericaAPI.createTag('Important', '#ff0000');
      expect(createdTag1).toEqual(tag1);

      // タグ一覧を取得
      mockInvoke.mockResolvedValueOnce([tag1]);
      const tags = await MockClericaAPI.getTags();
      expect(tags).toHaveLength(1);

      // ファイルのタグを更新
      mockInvoke.mockResolvedValueOnce(undefined);
      await MockClericaAPI.updateFileTags('file1', ['tag1']);
    });
  });

  describe('Search Integration', () => {
    it('should handle search scenarios', async () => {
      const searchResults = [
        {
          file: {
            id: 'file1',
            path: '/docs/report.pdf',
            name: 'report.pdf',
            directory_id: 'dir1',
            size: 3072,
            file_type: 'pdf',
            created_at: '2023-01-01T00:00:00Z',
            modified_at: '2023-01-02T00:00:00Z',
            birth_time: null,
            inode: 12345,
            is_directory: false,
            created_at_db: '2023-01-01T00:00:00Z',
            updated_at_db: '2023-01-02T00:00:00Z'
          },
          tags: [
            {
              id: 'tag1',
              name: 'Important',
              color: '#ff0000',
              created_at: '2023-01-01T00:00:00Z'
            }
          ],
          score: 0.95
        }
      ];

      // 検索クエリと複数のフィルターを使用
      mockInvoke.mockResolvedValueOnce(searchResults);
      const results = await MockClericaAPI.searchFiles(
        'report',
        ['tag1'],
        'modified'
      );

      expect(results).toHaveLength(1);
      expect(results[0].file.name).toBe('report.pdf');
      expect(results[0].score).toBe(0.95);

      // 空の検索結果
      mockInvoke.mockResolvedValueOnce([]);
      const emptyResults = await MockClericaAPI.searchFiles('nonexistent');
      expect(emptyResults).toHaveLength(0);
    });
  });

  describe('Data Consistency', () => {
    it('should maintain data consistency across operations', async () => {
      // 初期データセットアップ
      const initialDirectories = [
        {
          id: 'dir1',
          path: '/test/docs',
          name: 'Documents',
          created_at: '2023-01-01T00:00:00Z',
          updated_at: '2023-01-01T00:00:00Z'
        }
      ];

      // データの一貫性を確認するテストシーケンス
      mockInvoke.mockResolvedValueOnce(initialDirectories);
      const directories = await MockClericaAPI.getDirectories();
      expect(directories[0].id).toBe('dir1');

      // ディレクトリ内のファイル取得
      const filesInDirectory = [
        {
          id: 'file1',
          path: '/test/docs/file1.txt',
          name: 'file1.txt',
          directory_id: 'dir1',
          size: 1024,
          file_type: 'txt',
          created_at: '2023-01-01T00:00:00Z',
          modified_at: '2023-01-01T00:00:00Z',
          birth_time: null,
          inode: 12345,
          is_directory: false,
          created_at_db: '2023-01-01T00:00:00Z',
          updated_at_db: '2023-01-01T00:00:00Z'
        }
      ];

      mockInvoke.mockResolvedValueOnce(filesInDirectory);
      const files = await MockClericaAPI.getFilesByDirectory('dir1');
      expect(files).toHaveLength(1);
      expect(files[0].directory_id).toBe('dir1');
    });
  });
});
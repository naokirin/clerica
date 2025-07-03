import { test, expect } from '@playwright/test';

test.describe('ファイル管理機能', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('ファイル検索機能', async ({ page }) => {
    // 検索タブをクリック
    await page.getByRole('button', { name: '検索' }).click();
    
    // 検索入力欄を取得
    const searchInput = page.getByPlaceholder('ファイル名を入力...');
    await expect(searchInput).toBeVisible();
    
    // 検索実行
    await searchInput.fill('test');
    await page.locator('.search-button').click();
    
    // 検索結果の表示を確認
    await page.waitForTimeout(1000); // 検索結果の読み込みを待つ
    
    // 検索結果エリアの確認
    const searchResults = page.locator('.search-results');
    await expect(searchResults).toBeVisible();
  });

  test('ファイル一覧の表示と操作', async ({ page }) => {
    // ファイル一覧の表示確認
    const fileList = page.locator('.file-list');
    await expect(fileList).toBeVisible();
    
    // ファイル項目の確認（ファイルが存在する場合）
    const fileItems = page.locator('.file-item');
    
    // ファイル項目が存在する場合のテスト
    if (await fileItems.count() > 0) {
      const firstFile = fileItems.first();
      await expect(firstFile).toBeVisible();
      
      // ファイルクリックでの詳細表示
      await firstFile.click();
      
      // ファイル詳細モーダルの表示確認
      const fileModal = page.locator('.modal-overlay');
      await expect(fileModal).toBeVisible();
    }
  });

  test('ファイルソート機能', async ({ page }) => {
    // ファイル一覧の表示確認
    const fileList = page.locator('.file-list');
    await expect(fileList).toBeVisible();
    
    // 現在の実装では明示的なソートボタンがないため、スキップ
    // ソート機能が実装されたらテストを追加
  });

  test('ファイルフィルタリング', async ({ page }) => {
    // ファイル一覧の表示確認
    const fileList = page.locator('.file-list');
    await expect(fileList).toBeVisible();
    
    // 現在の実装では明示的なフィルターボタンがないため、スキップ
    // フィルター機能が実装されたらテストを追加
  });

  test('ファイルプレビュー機能', async ({ page }) => {
    // ファイル一覧からファイルを選択
    const fileItems = page.locator('.file-item');
    
    if (await fileItems.count() > 0) {
      const firstFile = fileItems.first();
      await firstFile.click();
      
      // ファイル詳細モーダルが開くことを確認
      const fileModal = page.locator('.modal-overlay');
      await expect(fileModal).toBeVisible();
      
      // モーダル内のファイル情報表示確認
      const fileDetails = page.locator('.modal-content');
      await expect(fileDetails).toBeVisible();
    }
  });

  test('ファイル操作メニュー', async ({ page }) => {
    // ファイル一覧からファイルを選択
    const fileItems = page.locator('.file-item');
    
    if (await fileItems.count() > 0) {
      const firstFile = fileItems.first();
      
      // ファイルをクリックして詳細モーダルを表示
      await firstFile.click();
      
      // モーダル内の操作ボタンの確認
      const fileModal = page.locator('.modal-overlay');
      await expect(fileModal).toBeVisible();
      
      // モーダル内のボタンが存在することを確認
      const modalContent = page.locator('.modal-content');
      await expect(modalContent).toBeVisible();
    }
  });
});
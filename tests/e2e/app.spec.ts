import { test, expect } from '@playwright/test';

test.describe('Clerica アプリケーション', () => {
  test('アプリケーションの起動と基本UI表示', async ({ page }) => {
    await page.goto('/');
    
    // タイトルの確認
    await expect(page).toHaveTitle('Clerica - File Manager');
    
    // 基本的なUIコンポーネントの存在確認
    await expect(page.getByRole('heading', { name: 'Clerica' })).toBeVisible();
  });

  test('ディレクトリ追加機能', async ({ page }) => {
    await page.goto('/');
    
    // ディレクトリ追加ボタンの存在確認（サイドバー内のボタン）
    const addButton = page.locator('.sidebar .add-button').first();
    await expect(addButton).toBeVisible();
    
    // ボタンをクリックしてダイアログが開くことを確認
    await addButton.click();
    // ダイアログの表示は実際のファイル選択に依存するため、ここではクリックできることのみ確認
  });

  test('検索機能の表示', async ({ page }) => {
    await page.goto('/');
    
    // 検索タブをクリック
    await page.getByRole('button', { name: '検索' }).click();
    
    // 検索入力欄の存在確認
    const searchInput = page.getByPlaceholder('ファイル名を入力...');
    await expect(searchInput).toBeVisible();
    
    // 検索入力欄にテキストを入力
    await searchInput.fill('test');
    await expect(searchInput).toHaveValue('test');
  });

  test('ファイル一覧の表示', async ({ page }) => {
    await page.goto('/');
    
    // ファイルタブの表示確認（デフォルト）
    const filesTab = page.getByRole('button', { name: 'ファイル' });
    await expect(filesTab).toBeVisible();
    
    // ファイル一覧エリアの存在確認
    const fileList = page.locator('.file-list');
    await expect(fileList).toBeVisible();
  });

  test('タグ管理機能の表示', async ({ page }) => {
    await page.goto('/');
    
    // タグ管理タブをクリック
    await page.getByRole('button', { name: 'タグ管理' }).click();
    
    // タグ管理エリアの存在確認
    const tagSection = page.locator('.tags-view');
    await expect(tagSection).toBeVisible();
    
    // タグ管理のヘッダー確認
    await expect(page.getByRole('heading', { name: 'タグ管理' })).toBeVisible();
  });

  test('レスポンシブデザイン', async ({ page }) => {
    await page.goto('/');
    
    // デスクトップサイズでの表示確認
    await page.setViewportSize({ width: 1200, height: 800 });
    await expect(page.getByRole('heading', { name: 'Clerica' })).toBeVisible();
    
    // モバイルサイズでの表示確認
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.getByRole('heading', { name: 'Clerica' })).toBeVisible();
  });

  test('アプリケーションの基本ナビゲーション', async ({ page }) => {
    await page.goto('/');
    
    // ページの読み込み完了を待つ
    await page.waitForLoadState('networkidle');
    
    // 基本的なナビゲーション要素の確認
    await expect(page.locator('body')).toBeVisible();
    await expect(page.locator('.app')).toBeVisible();
    
    // タブ切り替えテスト
    await page.getByRole('button', { name: '検索' }).click();
    await expect(page.locator('.search-view')).toBeVisible();
    
    await page.getByRole('button', { name: 'タグ管理' }).click();
    await expect(page.locator('.tags-view')).toBeVisible();
    
    await page.getByRole('button', { name: 'ファイル' }).click();
    await expect(page.locator('.file-list')).toBeVisible();
  });

  test('サイドバーの表示', async ({ page }) => {
    await page.goto('/');
    
    // サイドバーの表示確認
    const sidebar = page.locator('.sidebar');
    await expect(sidebar).toBeVisible();
    
    // ディレクトリセクションの確認
    await expect(page.locator('.sidebar h3').first()).toBeVisible();
    await expect(page.locator('.sidebar .add-button').first()).toBeVisible();
    
    // タグセクションの確認
    await expect(page.locator('.sidebar .add-button').last()).toBeVisible();
  });

});
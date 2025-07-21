import { test, expect } from '@playwright/test';

test.describe('タグ管理機能', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('タグセクションの表示', async ({ page }) => {
    // タグ管理タブをクリック
    await page.getByRole('button', { name: 'タグ管理' }).click();

    // タグセクションの表示確認
    const tagSection = page.locator('.tags-view');
    await expect(tagSection).toBeVisible();

    // タグ管理のヘッダー確認
    await expect(page.getByRole('heading', { name: 'タグ管理' })).toBeVisible();
  });
});
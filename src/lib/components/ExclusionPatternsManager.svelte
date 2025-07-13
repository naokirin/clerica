<script lang="ts">
  import { onMount } from 'svelte';
  import { exclusionPatternsApi, type ExclusionPattern } from '$lib/api';
  import { errorStore } from '$lib/stores/error';
  import LoadingScreen from './LoadingScreen.svelte';

  let patterns: ExclusionPattern[] = $state([]);
  let newPattern = $state('');
  let isLoading = $state(true);
  let isSaving = $state(false);
  let testPath = $state('');
  let testResult: boolean | null = $state(null);
  let validationError = $state('');

  onMount(async () => {
    await loadPatterns();
  });

  async function loadPatterns() {
    try {
      isLoading = true;
      patterns = await exclusionPatternsApi.getExclusionPatterns();
    } catch (error) {
      errorStore.setError('除外パターンの読み込みに失敗しました', error);
    } finally {
      isLoading = false;
    }
  }

  async function addPattern() {
    if (!newPattern.trim()) return;

    try {
      isSaving = true;
      validationError = '';
      
      // パターンの妥当性をチェック
      await exclusionPatternsApi.validateExclusionPattern(newPattern);
      
      // パターンを追加
      await exclusionPatternsApi.addExclusionPattern(newPattern);
      
      // リストを再読み込み
      await loadPatterns();
      
      // 入力フィールドをクリア
      newPattern = '';
      testResult = null;
    } catch (error) {
      validationError = error as string;
      errorStore.setError('除外パターンの追加に失敗しました', error);
    } finally {
      isSaving = false;
    }
  }

  async function deletePattern(id: number) {
    try {
      await exclusionPatternsApi.deleteExclusionPattern(id);
      await loadPatterns();
    } catch (error) {
      errorStore.setError('除外パターンの削除に失敗しました', error);
    }
  }

  async function testPattern() {
    if (!newPattern.trim() || !testPath.trim()) return;

    try {
      testResult = await exclusionPatternsApi.testExclusionPattern(newPattern, testPath);
    } catch (error) {
      errorStore.setError('パターンテストに失敗しました', error);
    }
  }

  function clearTest() {
    testPath = '';
    testResult = null;
  }

  const commonPatterns = [
    { name: 'ログファイル', pattern: '\\.log$' },
    { name: '一時ファイル', pattern: '\\.tmp$' },
    { name: 'DSファイル', pattern: '\\.DS_Store$' },
    { name: 'node_modules', pattern: '/node_modules/' },
    { name: 'Git ディレクトリ', pattern: '/\\.git/' },
    { name: 'ビルドディレクトリ', pattern: '/(build|dist|target)/' },
  ];
</script>

<div class="exclusion-patterns-manager">
  <h3>ファイル除外パターン</h3>
  <p class="description">
    正規表現でファイルやディレクトリを管理対象から除外できます。
  </p>

  {#if isLoading}
    <LoadingScreen />
  {:else}
    <!-- 新しいパターンの追加 -->
    <div class="add-pattern-section">
      <h4>新しいパターンを追加</h4>
      
      <div class="input-group">
        <input
          type="text"
          bind:value={newPattern}
          placeholder="正規表現パターンを入力 (例: \.log$)"
          class="pattern-input"
          disabled={isSaving}
        />
        <button onclick={addPattern} disabled={!newPattern.trim() || isSaving} class="add-button">
          {isSaving ? '追加中...' : '追加'}
        </button>
      </div>

      {#if validationError}
        <div class="error-message">{validationError}</div>
      {/if}

      <!-- よく使われるパターンの例 -->
      <div class="common-patterns">
        <h5>よく使われるパターン</h5>
        <div class="pattern-examples">
          {#each commonPatterns as example}
            <button 
              class="pattern-example"
              onclick={() => newPattern = example.pattern}
              disabled={isSaving}
            >
              <span class="pattern-name">{example.name}</span>
              <code class="pattern-code">{example.pattern}</code>
            </button>
          {/each}
        </div>
      </div>

      <!-- パターンテスト機能 -->
      <div class="pattern-test">
        <h5>パターンテスト</h5>
        <div class="test-inputs">
          <input
            type="text"
            bind:value={testPath}
            placeholder="テストするファイルパスを入力"
            class="test-path-input"
          />
          <button onclick={testPattern} disabled={!newPattern.trim() || !testPath.trim()}>
            テスト
          </button>
          <button onclick={clearTest}>クリア</button>
        </div>
        
        {#if testResult !== null}
          <div class="test-result {testResult ? 'match' : 'no-match'}">
            {testResult ? '✓ マッチしました（除外されます）' : '✗ マッチしません（除外されません）'}
          </div>
        {/if}
      </div>
    </div>

    <!-- 現在のパターン一覧 -->
    <div class="patterns-list">
      <h4>現在の除外パターン ({patterns.length}個)</h4>
      
      {#if patterns.length === 0}
        <div class="empty-state">
          除外パターンが設定されていません
        </div>
      {:else}
        <div class="patterns-grid">
          {#each patterns as pattern (pattern.id)}
            <div class="pattern-item">
              <code class="pattern-code">{pattern.pattern}</code>
              <div class="pattern-actions">
                <small class="pattern-date">
                  {new Date(pattern.created_at).toLocaleDateString('ja-JP')}
                </small>
                <button
                  onclick={() => deletePattern(pattern.id)}
                  class="delete-button"
                  title="削除"
                >
                  ×
                </button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .exclusion-patterns-manager {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
  }

  .description {
    color: #666;
    margin-bottom: 24px;
    line-height: 1.5;
  }

  .add-pattern-section {
    background: #f8f9fa;
    padding: 20px;
    border-radius: 8px;
    margin-bottom: 24px;
  }

  .input-group {
    display: flex;
    gap: 10px;
    margin-bottom: 16px;
  }

  .pattern-input {
    flex: 1;
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-family: 'Courier New', monospace;
  }

  .add-button {
    padding: 8px 16px;
    background: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .add-button:disabled {
    background: #ccc;
    cursor: not-allowed;
  }

  .error-message {
    color: #d32f2f;
    background: #ffebee;
    padding: 8px 12px;
    border-radius: 4px;
    margin-bottom: 16px;
    font-size: 14px;
  }

  .common-patterns {
    margin-bottom: 20px;
  }

  .pattern-examples {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 8px;
    margin-top: 8px;
  }

  .pattern-example {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: 8px 12px;
    background: white;
    border: 1px solid #ddd;
    border-radius: 4px;
    cursor: pointer;
    text-align: left;
  }

  .pattern-example:hover {
    background: #f0f0f0;
  }

  .pattern-name {
    font-size: 12px;
    color: #666;
  }

  .pattern-code {
    font-family: 'Courier New', monospace;
    background: #f5f5f5;
    padding: 2px 4px;
    border-radius: 2px;
    font-size: 12px;
  }

  .pattern-test {
    border-top: 1px solid #eee;
    padding-top: 16px;
  }

  .test-inputs {
    display: flex;
    gap: 8px;
    margin-bottom: 8px;
  }

  .test-path-input {
    flex: 1;
    padding: 6px 8px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-family: 'Courier New', monospace;
    font-size: 12px;
  }

  .test-inputs button {
    padding: 6px 12px;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: white;
    cursor: pointer;
  }

  .test-result {
    padding: 8px 12px;
    border-radius: 4px;
    font-size: 14px;
    margin-top: 8px;
  }

  .test-result.match {
    background: #e8f5e8;
    color: #2e7d32;
  }

  .test-result.no-match {
    background: #fff3e0;
    color: #f57c00;
  }

  .patterns-list {
    background: white;
    border: 1px solid #eee;
    border-radius: 8px;
    padding: 20px;
  }

  .empty-state {
    text-align: center;
    color: #999;
    padding: 40px;
    font-style: italic;
  }

  .patterns-grid {
    display: grid;
    gap: 8px;
  }

  .pattern-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px;
    background: #f8f9fa;
    border-radius: 4px;
    border: 1px solid #eee;
  }

  .pattern-item .pattern-code {
    flex: 1;
    margin-right: 12px;
  }

  .pattern-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .pattern-date {
    color: #999;
    font-size: 11px;
  }

  .delete-button {
    background: #dc3545;
    color: white;
    border: none;
    border-radius: 50%;
    width: 24px;
    height: 24px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 16px;
    line-height: 1;
  }

  .delete-button:hover {
    background: #c82333;
  }

  h3 {
    margin-bottom: 8px;
    color: #333;
  }

  h4 {
    margin: 0 0 16px 0;
    color: #333;
    font-size: 16px;
  }

  h5 {
    margin: 0 0 8px 0;
    color: #333;
    font-size: 14px;
  }
</style>
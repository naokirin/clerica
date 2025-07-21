<script lang="ts">
  import Snackbar from '../lib/components/parts/Snackbar.svelte';
  import Button from '../lib/components/parts/Button.svelte';
  import { errorStore } from '../lib/stores/error';

  // アイコン定義
  const successIcon = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20,6 9,17 4,12"/></svg>';
  const errorIcon = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>';
  const warningIcon = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>';
  const infoIcon = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>';

  import { derived } from 'svelte/store';
  
  // ストアの状態を監視するためのリアクティブな変数
  const errors = derived(errorStore, ($errorStore) => $errorStore.errors);

  function showSuccess() {
    console.log("showSuccess called");
    errorStore.showSuccess("操作が正常に完了しました！");
    console.log("Current errors:", $errors);
  }

  function showInfo() {
    console.log("showInfo called");
    errorStore.showInfo("新しい機能が利用可能です。");
    console.log("Current errors:", $errors);
  }

  function showWarning() {
    console.log("showWarning called");
    errorStore.showWarning("ディスク容量が不足しています。");
    console.log("Current errors:", $errors);
  }

  function showError() {
    console.log("showError called");
    errorStore.showError("ファイルの読み込みに失敗しました。");
    console.log("Current errors:", $errors);
  }

  function showLongMessage() {
    errorStore.showInfo("これは非常に長いメッセージの例です。通常のSnackbarよりも多くのテキストが含まれており、改行やワードラップがどのように動作するかをテストするためのものです。");
  }

  function showMultipleMessages() {
    errorStore.showSuccess("最初のメッセージ");
    setTimeout(() => errorStore.showInfo("2番目のメッセージ"), 500);
    setTimeout(() => errorStore.showWarning("3番目のメッセージ"), 1000);
  }

  function clearAll() {
    errorStore.clearAll();
  }
</script>

<div style="padding: 20px; max-width: 600px; position: relative;">
  <h3>Snackbar Examples</h3>
  <p>下のボタンをクリックして、異なるタイプのSnackbarを表示できます。</p>
  
  <!-- デバッグ情報 -->
  <div style="margin-bottom: 20px; padding: 10px; background: #f0f0f0; border-radius: 4px;">
    <strong>Debug Info:</strong><br>
    Current errors count: {$errors.length}<br>
    {#if $errors.length > 0}
      Latest error: {$errors[$errors.length - 1]?.message} (type: {$errors[$errors.length - 1]?.type})
    {/if}
  </div>
  
  <!-- Snackbarテスト用の固定表示 -->
  {#if $errors.length > 0}
    <div style="margin-bottom: 20px; padding: 10px; background: #dcfce7; color: #166534; border: 1px solid #bbf7d0; border-radius: 4px;">
      <strong>Inline Snackbar Test:</strong><br>
      {#each $errors as error}
        <div>{error.message} ({error.type})</div>
      {/each}
    </div>
  {/if}
  
  <div style="display: flex; gap: 10px; flex-wrap: wrap; margin-bottom: 20px;">
    <Button 
      variant="primary" 
      text="Success Message" 
      {...({ icon: successIcon } as any)}
      onclick={showSuccess}
    />
    <Button 
      variant="secondary" 
      text="Info Message" 
      {...({ icon: infoIcon } as any)}
      onclick={showInfo}
    />
    <Button 
      variant="danger" 
      text="Warning Message" 
      {...({ icon: warningIcon } as any)}
      onclick={showWarning}
    />
    <Button 
      variant="danger" 
      text="Error Message" 
      {...({ icon: errorIcon } as any)}
      onclick={showError}
    />
  </div>

  <div style="display: flex; gap: 10px; flex-wrap: wrap; margin-bottom: 20px;">
    <Button 
      variant="secondary" 
      text="Long Message" 
      onclick={showLongMessage}
    />
    <Button 
      variant="secondary" 
      text="Multiple Messages" 
      onclick={showMultipleMessages}
    />
    <Button 
      variant="secondary" 
      text="Clear All" 
      onclick={clearAll}
    />
  </div>
</div>

<Snackbar />
<script lang="ts">
  import Button from '../lib/components/parts/Button.svelte';
  import { writable } from 'svelte/store';

  // アイコン定義
  const successIcon = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20,6 9,17 4,12"/></svg>';
  const errorIcon = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>';
  const warningIcon = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>';
  const infoIcon = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>';

  // Storybook専用のローカルエラーストア
  interface ErrorInfo {
    id: string;
    message: string;
    type: 'error' | 'warning' | 'info' | 'success';
    duration?: number;
    timestamp: number;
  }

  const storybookErrors = writable<ErrorInfo[]>([]);

  // errorsがundefinedの場合のフォールバック
  $: safeErrors = $storybookErrors;

  function addError(message: string, type: ErrorInfo['type'], duration = 3000) {
    const error: ErrorInfo = {
      id: crypto.randomUUID(),
      message,
      type,
      duration,
      timestamp: Date.now()
    };
    
    storybookErrors.update(errors => [...errors, error]);
    
    // 自動削除
    if (duration > 0) {
      setTimeout(() => {
        storybookErrors.update(errors => errors.filter(e => e.id !== error.id));
      }, duration);
    }
  }

  function showSuccess() {
    console.log("showSuccess called");
    addError("操作が正常に完了しました！", "success");
    console.log("Current errors:", $storybookErrors);
  }

  function showInfo() {
    console.log("showInfo called");
    addError("新しい機能が利用可能です。", "info");
    console.log("Current errors:", $storybookErrors);
  }

  function showWarning() {
    console.log("showWarning called");
    addError("ディスク容量が不足しています。", "warning");
    console.log("Current errors:", $storybookErrors);
  }

  function showError() {
    console.log("showError called");
    addError("ファイルの読み込みに失敗しました。", "error");
    console.log("Current errors:", $storybookErrors);
  }

  function clearAll() {
    console.log("clearAll called");
    storybookErrors.set([]);
  }

  function dismissError(id: string) {
    storybookErrors.update(errors => errors.filter(e => e.id !== id));
  }

  // Snackbar用のカスタムコンポーネント（Storybook用）
  import { fly } from 'svelte/transition';
  import { X, AlertCircle, CheckCircle, Info, AlertTriangle } from 'lucide-svelte';

  const getIcon = (type: any) => {
    switch (type) {
      case 'error':
        return AlertCircle;
      case 'warning':
        return AlertTriangle;
      case 'info':
        return Info;
      case 'success':
        return CheckCircle;
      default:
        return Info;
    }
  };

  const getColorClass = (type: any) => {
    switch (type) {
      case 'error':
        return 'snackbar-error';
      case 'warning':
        return 'snackbar-warning';
      case 'info':
        return 'snackbar-info';
      case 'success':
        return 'snackbar-success';
      default:
        return 'snackbar-info';
    }
  };

  // dismissError関数は上で定義済み
</script>

<div style="padding: 20px; max-width: 600px; position: relative;">
  <h3>Snackbar Examples</h3>
  <p>下のボタンをクリックして、異なるタイプのSnackbarを表示できます。</p>
  
  <!-- デバッグ情報 -->
  <div style="margin-bottom: 20px; padding: 10px; background: #f0f0f0; border-radius: 4px;">
    <strong>Debug Info:</strong><br>
    Current errors count: {safeErrors.length}<br>
    {#if safeErrors.length > 0}
      Latest error: {safeErrors[safeErrors.length - 1]?.message} (type: {safeErrors[safeErrors.length - 1]?.type})
    {/if}
  </div>
  
  <div style="display: flex; gap: 10px; flex-wrap: wrap; margin-bottom: 20px;">
    <Button 
      variant="primary" 
      text="Success Message" 
      icon={successIcon}
      onclick={showSuccess}
    />
    <Button 
      variant="secondary" 
      text="Info Message" 
      icon={infoIcon}
      onclick={showInfo}
    />
    <Button 
      variant="danger" 
      text="Warning Message" 
      icon={warningIcon}
      onclick={showWarning}
    />
    <Button 
      variant="danger" 
      text="Error Message" 
      icon={errorIcon}
      onclick={showError}
    />
  </div>

  <div style="display: flex; gap: 10px; flex-wrap: wrap; margin-bottom: 20px;">
    <Button 
      variant="secondary" 
      text="Clear All" 
      onclick={clearAll}
    />
  </div>

  <!-- Storybook用のSnackbar表示 (position: fixed を使わない) -->
  <div style="position: relative; min-height: 200px; border: 1px dashed #ccc; padding: 20px; margin-top: 20px;">
    <p><strong>Snackbar Display Area:</strong></p>
    <div style="position: absolute; top: 20px; right: 20px; display: flex; flex-direction: column; gap: 8px; z-index: 1000;">
      {#each safeErrors as error (error.id)}
        <div
          class="snackbar {getColorClass(error.type)}"
          transition:fly="{{ y: -50, duration: 300 }}"
        >
          <div class="snackbar-content">
            <div class="snackbar-icon">
              <svelte:component this={getIcon(error.type)} size={20} />
            </div>
            <div class="snackbar-message">
              {error.message}
            </div>
            <button
              class="snackbar-close"
              onclick={() => dismissError(error.id)}
            >
              <X size={16} />
            </button>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .snackbar {
    min-width: 320px;
    max-width: 400px;
    padding: 12px 16px;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    font-size: 14px;
    line-height: 1.4;
  }

  .snackbar-content {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .snackbar-icon {
    flex-shrink: 0;
    display: flex;
    align-items: center;
  }

  .snackbar-message {
    flex: 1;
    word-wrap: break-word;
  }

  .snackbar-close {
    flex-shrink: 0;
    background: none;
    border: none;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.2s ease;
  }

  .snackbar-close:hover {
    background-color: rgba(255, 255, 255, 0.1);
  }

  .snackbar-error {
    background-color: #fee2e2;
    color: #991b1b;
    border: 1px solid #fecaca;
  }

  .snackbar-warning {
    background-color: #fef3c7;
    color: #92400e;
    border: 1px solid #fed7aa;
  }

  .snackbar-info {
    background-color: #dbeafe;
    color: #1e40af;
    border: 1px solid #bfdbfe;
  }

  .snackbar-success {
    background-color: #dcfce7;
    color: #166534;
    border: 1px solid #bbf7d0;
  }

  .snackbar-error .snackbar-close:hover {
    background-color: rgba(153, 27, 27, 0.1);
  }

  .snackbar-warning .snackbar-close:hover {
    background-color: rgba(146, 64, 14, 0.1);
  }

  .snackbar-info .snackbar-close:hover {
    background-color: rgba(30, 64, 175, 0.1);
  }

  .snackbar-success .snackbar-close:hover {
    background-color: rgba(22, 101, 52, 0.1);
  }

  /* ダークモード対応 */
  @media (prefers-color-scheme: dark) {
    .snackbar-error {
      background-color: #7f1d1d;
      color: #fef2f2;
      border: 1px solid #991b1b;
    }

    .snackbar-warning {
      background-color: #78350f;
      color: #fef3c7;
      border: 1px solid #92400e;
    }

    .snackbar-info {
      background-color: #1e3a8a;
      color: #dbeafe;
      border: 1px solid #3b82f6;
    }

    .snackbar-success {
      background-color: #14532d;
      color: #dcfce7;
      border: 1px solid #166534;
    }
  }
</style>
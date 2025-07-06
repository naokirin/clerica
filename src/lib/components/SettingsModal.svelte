<script lang="ts">
  import { X } from "lucide-svelte";
  import { onMount } from "svelte";
  import { getSettings, updateSettingBool, updateSettingInt } from "../api/settings";
  import { errorStore } from "../stores/error.js";

  export let isOpen = false;
  export let onClose: () => void;
  export let onSettingsChanged: (() => void) | undefined = undefined;

  // 設定の状態管理
  let darkMode = false;
  let detailViewDefault = false;
  let filesPerPage = 20;
  let showHiddenFiles = false;
  let useFuzzySearch = true;
  let highlightSearchResults = true;
  let isLoading = false;

  // 設定を読み込み
  onMount(async () => {
    try {
      const settings = await getSettings();
      showHiddenFiles = settings.show_hidden_files;
      filesPerPage = settings.files_per_page;
    } catch (error) {
      console.error('設定の読み込みに失敗しました:', error);
      errorStore.showError("設定の読み込みに失敗しました。アプリケーションを再起動してください。");
    }
  });

  // モーダルが開かれるたびに設定を再読み込み
  $: if (isOpen) {
    loadSettings();
  }

  const loadSettings = async () => {
    try {
      const settings = await getSettings();
      showHiddenFiles = settings.show_hidden_files;
      filesPerPage = settings.files_per_page;
    } catch (error) {
      console.error('設定の読み込みに失敗しました:', error);
      errorStore.showError("設定の読み込みに失敗しました。アプリケーションを再起動してください。");
    }
  };

  const handleClose = () => {
    onClose();
  };

  const handleOverlayClick = (event: MouseEvent) => {
    if (event.target === event.currentTarget) {
      handleClose();
    }
  };

  const handleSave = async () => {
    isLoading = true;
    try {
      await updateSettingBool('show_hidden_files', showHiddenFiles);
      await updateSettingInt('files_per_page', filesPerPage);
      console.log("Settings saved:", {
        darkMode,
        detailViewDefault,
        filesPerPage,
        showHiddenFiles,
        useFuzzySearch,
        highlightSearchResults,
      });
      
      errorStore.showSuccess("設定を保存しました。");
      
      // 設定が変更されたことを通知
      if (onSettingsChanged) {
        onSettingsChanged();
      }
      
      handleClose();
    } catch (error) {
      console.error('設定の保存に失敗しました:', error);
      errorStore.showError("設定の保存に失敗しました。もう一度お試しください。");
    } finally {
      isLoading = false;
    }
  };
</script>

{#if isOpen}
  <div
    class="modal-overlay"
    on:click={handleOverlayClick}
    role="dialog"
    aria-modal="true"
    aria-labelledby="settings-title"
  >
    <div class="modal-content" on:click={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 id="settings-title">設定</h3>
        <button
          class="close-button"
          on:click={handleClose}
          aria-label="設定を閉じる"
        >
          <X size={20} />
        </button>
      </div>
      <div class="modal-body">
        <div class="settings-section">
          <h4>ファイル管理</h4>
          <div class="setting-item">
            <label class="setting-label">
              1ページあたりのファイル数:
              <select class="setting-select" bind:value={filesPerPage}>
                <option value={10}>10</option>
                <option value={20}>20</option>
                <option value={50}>50</option>
                <option value={100}>100</option>
                <option value={200}>200</option>
              </select>
            </label>
          </div>
          <div class="setting-item">
            <label class="setting-label">
              <input
                type="checkbox"
                class="setting-checkbox"
                bind:checked={showHiddenFiles}
              />
              隠しファイルを表示
            </label>
          </div>
        </div>

        <div class="settings-actions">
          <button class="save-button" on:click={handleSave} disabled={isLoading}>
            {isLoading ? '保存中...' : '設定を保存'}
          </button>
          <button class="cancel-button" on:click={handleClose} disabled={isLoading}>
            キャンセル
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  /* 設定モーダル固有のスタイル */
  .settings-section {
    margin-bottom: 2rem;
  }

  .settings-section:last-child {
    margin-bottom: 0;
  }

  .settings-section h4 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: #374151;
    border-bottom: 1px solid #e5e7eb;
    padding-bottom: 0.5rem;
  }

  .setting-item {
    margin-bottom: 0.75rem;
  }

  .setting-item:last-child {
    margin-bottom: 0;
  }

  .setting-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    color: #374151;
    cursor: pointer;
  }

  .setting-checkbox {
    width: 16px;
    height: 16px;
    cursor: pointer;
  }

  .setting-select {
    padding: 0.375rem 0.5rem;
    border: 1px solid #d1d5db;
    border-radius: 0.25rem;
    font-size: 0.875rem;
    background-color: white;
    cursor: pointer;
  }

  .setting-select:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
  }

  .settings-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 2rem;
    padding-top: 1.5rem;
    border-top: 1px solid #e5e7eb;
  }

  .save-button,
  .cancel-button {
    padding: 0.75rem 1.5rem;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    border: 1px solid;
  }

  .save-button {
    background-color: #3b82f6;
    color: white;
    border-color: #3b82f6;
  }

  .save-button:hover:not(:disabled) {
    background-color: #2563eb;
    border-color: #2563eb;
  }

  .save-button:disabled {
    background-color: #9ca3af;
    border-color: #9ca3af;
    cursor: not-allowed;
    opacity: 0.6;
  }

  .cancel-button {
    background-color: white;
    color: #374151;
    border-color: #d1d5db;
  }

  .cancel-button:hover:not(:disabled) {
    background-color: #f9fafb;
    border-color: #9ca3af;
  }

  .cancel-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>

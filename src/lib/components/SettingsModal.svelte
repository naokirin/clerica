<script lang="ts">
  import { X } from "lucide-svelte";

  export let isOpen = false;
  export let onClose: () => void;

  // 設定の状態管理
  let darkMode = false;
  let detailViewDefault = false;
  let filesPerPage = "20";
  let showHiddenFiles = false;
  let useFuzzySearch = true;
  let highlightSearchResults = true;

  const handleClose = () => {
    onClose();
  };

  const handleOverlayClick = (event: MouseEvent) => {
    if (event.target === event.currentTarget) {
      handleClose();
    }
  };

  const handleSave = () => {
    // TODO: 設定の保存処理を実装
    console.log("Settings saved:", {
      darkMode,
      detailViewDefault,
      filesPerPage,
      showHiddenFiles,
      useFuzzySearch,
      highlightSearchResults,
    });
    handleClose();
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
                <option value="20">20</option>
                <option value="50">50</option>
                <option value="100">100</option>
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
          <button class="save-button" on:click={handleSave}>
            設定を保存
          </button>
          <button class="cancel-button" on:click={handleClose}>
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

  .save-button:hover {
    background-color: #2563eb;
    border-color: #2563eb;
  }

  .cancel-button {
    background-color: white;
    color: #374151;
    border-color: #d1d5db;
  }

  .cancel-button:hover {
    background-color: #f9fafb;
    border-color: #9ca3af;
  }
</style>

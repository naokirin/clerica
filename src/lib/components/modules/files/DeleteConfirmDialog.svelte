<script lang="ts">
  interface Props {
    isOpen: boolean;
    fileCount: number;
    onConfirm: () => void;
    onCancel: () => void;
  }

  let { isOpen, fileCount, onConfirm, onCancel }: Props = $props();

  const handleKeydown = (event: KeyboardEvent) => {
    if (event.key === "Escape") {
      onCancel();
    } else if (event.key === "Enter") {
      onConfirm();
    }
  };
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-overlay" onclick={onCancel} onkeydown={handleKeydown} role="dialog" aria-modal="true" tabindex="-1">
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3>ファイル削除の確認</h3>
      </div>
      
      <div class="modal-body">
        <div class="warning-icon">⚠️</div>
        <p>
          {fileCount === 1 
            ? "選択したファイルをゴミ箱に移動しますか？" 
            : `選択した${fileCount}件のファイルをゴミ箱に移動しますか？`}
        </p>
        <p class="warning-text">
          この操作は元に戻すことができません。
        </p>
      </div>
      
      <div class="modal-actions">
        <button class="cancel-btn" onclick={onCancel}>
          キャンセル
        </button>
        <button class="delete-btn" onclick={onConfirm}>
          削除
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    background: white;
    border-radius: 8px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    max-width: 400px;
    width: 90%;
    max-height: 90vh;
    overflow-y: auto;
  }

  .modal-header {
    padding: 20px 20px 0 20px;
  }

  .modal-header h3 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #333;
  }

  .modal-body {
    padding: 20px;
    text-align: center;
  }

  .warning-icon {
    font-size: 3rem;
    margin-bottom: 16px;
  }

  .modal-body p {
    margin: 8px 0;
    color: #333;
  }

  .warning-text {
    font-size: 0.9rem;
    color: #f44336;
    font-weight: 500;
  }

  .modal-actions {
    display: flex;
    gap: 12px;
    padding: 0 20px 20px 20px;
    justify-content: flex-end;
  }

  .cancel-btn, .delete-btn {
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .cancel-btn {
    background: #f5f5f5;
    color: #333;
    border: 1px solid #ddd;
  }

  .cancel-btn:hover {
    background: #e0e0e0;
    border-color: #bbb;
  }

  .delete-btn {
    background: #f44336;
    color: white;
  }

  .delete-btn:hover {
    background: #da190b;
  }

  .delete-btn:focus, .cancel-btn:focus {
    outline: 2px solid #2196f3;
    outline-offset: 2px;
  }
</style>
<script lang="ts">
  import { X, Trash2, Loader2 } from "lucide-svelte";
  import type { File } from "../types.js";
  import { formatFileSize, formatDate } from "../utils.js";

  interface Props {
    file: File | null;
    isDeleting: boolean;
    onOpenFile: (filePath: string) => void;
    onRevealInFinder: (filePath: string) => void;
    onDeleteFile: (filePath: string, fileName: string) => void;
    onClose: () => void;
  }

  let {
    file,
    isDeleting,
    onOpenFile,
    onRevealInFinder,
    onDeleteFile,
    onClose
  }: Props = $props();
</script>

{#if file}
  <div class="modal-overlay" onclick={isDeleting ? undefined : onClose}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3>ファイル詳細</h3>
        <div class="modal-actions">
          <button 
            class="action-button open-button" 
            onclick={() => onOpenFile(file.path)}
            title="ファイルを開く"
            disabled={isDeleting}
          >
            📂 開く
          </button>
          <button 
            class="action-button finder-button" 
            onclick={() => onRevealInFinder(file.path)}
            title="Finderで表示"
            disabled={isDeleting}
          >
            🔍 Finder
          </button>
          <button 
            class="action-button delete-button" 
            onclick={() => onDeleteFile(file.path, file.name)}
            title={isDeleting ? "削除中..." : "ゴミ箱に移動"}
            disabled={isDeleting}
          >
            {#if isDeleting}
              <Loader2 size={16} class="animate-spin" />
              削除中...
            {:else}
              <Trash2 size={16} />
              削除
            {/if}
          </button>
          <button 
            class="close-button" 
            onclick={onClose}
            disabled={isDeleting}
          >
            <X size={20} />
          </button>
        </div>
      </div>
      <div class="modal-body">
        <div class="file-detail-section">
          <h4>基本情報</h4>
          <div class="detail-grid">
            <div class="detail-item">
              <span class="detail-label">ファイル名:</span>
              <span class="detail-value">{file.name}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">パス:</span>
              <span class="detail-value">{file.path}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">サイズ:</span>
              <span class="detail-value">{formatFileSize(file.file_size || file.size)}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">種類:</span>
              <span class="detail-value">{file.is_directory ? 'ディレクトリ' : (file.mime_type || file.file_type || 'Unknown')}</span>
            </div>
          </div>
        </div>

        <div class="file-detail-section">
          <h4>日時情報</h4>
          <div class="detail-grid">
            <div class="detail-item">
              <span class="detail-label">作成日時:</span>
              <span class="detail-value">{formatDate(file.created_at)}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">更新日時:</span>
              <span class="detail-value">{formatDate(file.modified_at)}</span>
            </div>
            {#if file.birth_time}
              <div class="detail-item">
                <span class="detail-label">作成日時 (birth_time):</span>
                <span class="detail-value">{formatDate(file.birth_time)}</span>
              </div>
            {/if}
            {#if file.last_accessed}
              <div class="detail-item">
                <span class="detail-label">最終アクセス日時:</span>
                <span class="detail-value">{formatDate(file.last_accessed)}</span>
              </div>
            {/if}
          </div>
        </div>

        <div class="file-detail-section">
          <h4>システム情報</h4>
          <div class="detail-grid">
            {#if file.permissions}
              <div class="detail-item">
                <span class="detail-label">権限:</span>
                <span class="detail-value">{file.permissions}</span>
              </div>
            {/if}
            {#if file.owner_uid !== null}
              <div class="detail-item">
                <span class="detail-label">オーナー UID:</span>
                <span class="detail-value">{file.owner_uid}</span>
              </div>
            {/if}
            {#if file.group_gid !== null}
              <div class="detail-item">
                <span class="detail-label">グループ GID:</span>
                <span class="detail-value">{file.group_gid}</span>
              </div>
            {/if}
            {#if file.inode !== null}
              <div class="detail-item">
                <span class="detail-label">inode:</span>
                <span class="detail-value">{file.inode}</span>
              </div>
            {/if}
            {#if file.hard_links !== null}
              <div class="detail-item">
                <span class="detail-label">ハードリンク数:</span>
                <span class="detail-value">{file.hard_links}</span>
              </div>
            {/if}
            {#if file.device_id !== null}
              <div class="detail-item">
                <span class="detail-label">デバイス ID:</span>
                <span class="detail-value">{file.device_id}</span>
              </div>
            {/if}
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}
<script lang="ts">
  import { X, Loader2, Edit3 } from "@lucide/svelte";
  import type { File } from "../../../types";
  import * as filesApi from "../../../api/files";
  import { errorStore } from "../../../stores/error";
  import { t } from "$lib/i18n";
  import RenameHelp from "./RenameHelp.svelte";

  interface Props {
    file: File | null;
    onClose: () => void;
    onFileRenamed?: () => void;
  }

  let { file, onClose, onFileRenamed }: Props = $props();

  // リネーム機能の状態管理
  let renameMode = $state<"simple" | "regex">("simple");
  let newFileName = $state(""); // シンプルリネーム用
  let regexPattern = $state("");
  let formatTemplate = $state("");
  let previewResult = $state("");
  let isPreviewLoading = $state(false);
  let isRenaming = $state(false);
  let renameError = $state("");

  // プレビュー更新関数
  async function updatePreview() {
    if (!file) {
      previewResult = "";
      return;
    }

    // シンプルモードの場合
    if (renameMode === "simple") {
      if (!newFileName.trim()) {
        previewResult = "";
        return;
      }

      isPreviewLoading = true;
      renameError = "";

      try {
        const result = await filesApi.previewSimpleRename(
          file.id,
          newFileName.trim(),
        );
        previewResult = result;
      } catch (error: any) {
        console.error("プレビューエラー:", error);
        renameError = error.message || "プレビューの生成に失敗しました";
        previewResult = "";
      } finally {
        isPreviewLoading = false;
      }
      return;
    }

    // 正規表現モードの場合
    if (!regexPattern || !formatTemplate) {
      previewResult = "";
      return;
    }

    isPreviewLoading = true;
    renameError = "";

    try {
      const result = await filesApi.previewRename(
        file.id,
        regexPattern,
        formatTemplate,
      );
      previewResult = result;
    } catch (error: any) {
      console.error("プレビューエラー:", error);
      renameError = error.message || "プレビューの生成に失敗しました";
      previewResult = "";
    } finally {
      isPreviewLoading = false;
    }
  }

  // リネーム実行関数
  async function executeRename() {
    if (!file) return;

    // シンプルモードの場合
    if (renameMode === "simple" && !newFileName.trim()) return;

    // 正規表現モードの場合
    if (renameMode === "regex" && (!regexPattern || !formatTemplate)) return;

    isRenaming = true;
    renameError = "";

    try {
      let newName: string;

      if (renameMode === "simple") {
        newName = await filesApi.executeSimpleRename(
          file.id,
          newFileName.trim(),
        );
      } else {
        newName = await filesApi.executeRename(
          file.id,
          regexPattern,
          formatTemplate,
        );
      }

      console.log("リネーム成功:", newName);

      // 親コンポーネントに通知
      if (onFileRenamed) {
        onFileRenamed();
      }

      errorStore.showError(
        $t("common.fileDetail.renameSuccess", { name: newName }),
      );

      // モーダルを閉じる
      onClose();
    } catch (error: any) {
      console.error("リネームエラー:", error);
      renameError = error.message || $t("common.fileDetail.renameError");
    } finally {
      isRenaming = false;
    }
  }

  // 入力変更時にプレビューを更新
  $effect(() => {
    if (renameMode === "simple" && newFileName.trim()) {
      const timer = setTimeout(updatePreview, 300); // デバウンス
      return () => clearTimeout(timer);
    } else if (renameMode === "regex" && regexPattern && formatTemplate) {
      const timer = setTimeout(updatePreview, 300); // デバウンス
      return () => clearTimeout(timer);
    }
  });

  // ファイルが変更されたときに初期値を設定
  $effect(() => {
    if (file && renameMode === "simple") {
      // 拡張子を除いたファイル名を初期値として設定
      const fileName = file.name;
      const lastDotIndex = fileName.lastIndexOf(".");
      if (lastDotIndex > 0) {
        newFileName = fileName.substring(0, lastDotIndex);
      } else {
        newFileName = fileName;
      }
    }
  });
</script>

{#if file}
  <div class="modal-overlay" onclick={isRenaming ? undefined : onClose}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3>{$t("common.fileDetail.rename")}</h3>
        <button class="close-button" onclick={onClose} disabled={isRenaming}>
          <X size={20} />
        </button>
      </div>
      <div class="modal-body">
        <div class="file-info">
          <div class="file-name">
            <Edit3 size={16} class="icon" />
            <span>{file.name}</span>
          </div>
        </div>

        <div class="rename-form">
          <!-- リネームモード選択 -->
          <div class="rename-mode-selection">
            <h4>リネーム方法</h4>
            <div class="radio-group">
              <label class="radio-label">
                <input
                  type="radio"
                  bind:group={renameMode}
                  value="simple"
                  disabled={isRenaming}
                />
                <span class="radio-text"> シンプルリネーム </span>
              </label>
              <label class="radio-label">
                <input
                  type="radio"
                  bind:group={renameMode}
                  value="regex"
                  disabled={isRenaming}
                />
                <span class="radio-text"> 正規表現リネーム </span>
              </label>
            </div>
          </div>

          <!-- シンプルリネームモード -->
          {#if renameMode === "simple"}
            <div class="rename-input-group">
              <label for="new-file-name">新しいファイル名:</label>
              <input
                id="new-file-name"
                type="text"
                bind:value={newFileName}
                placeholder="新しいファイル名を入力"
                disabled={isRenaming}
              />
              <small class="input-hint"> 拡張子は自動的に保持されます </small>
            </div>
          {:else}
            <!-- 正規表現リネームモード -->
            <div class="rename-input-group">
              <label for="regex-pattern"
                >{$t("common.fileDetail.regexPattern")}:</label
              >
              <input
                id="regex-pattern"
                type="text"
                bind:value={regexPattern}
                placeholder="例: ^(.+)\.(.+)$"
                disabled={isRenaming}
              />
            </div>

            <div class="rename-input-group">
              <label for="format-template"
                >{$t("common.fileDetail.formatTemplate")}:</label
              >
              <input
                id="format-template"
                type="text"
                bind:value={formatTemplate}
                placeholder="例: {`{{ file.name_stem }}_{{ tags | join(sep=\"_\") }}.$2`}"
                disabled={isRenaming}
              />
            </div>
          {/if}

          <div class="rename-preview">
            <div class="preview-header">
              <span>{$t("common.fileDetail.preview")}:</span>
              {#if isPreviewLoading}
                <Loader2 size={14} class="animate-spin" />
              {/if}
            </div>
            <div class="preview-result">
              {#if renameError}
                <span class="error-text">{renameError}</span>
              {:else if previewResult}
                <span class="preview-text">{file.name} → {previewResult}</span>
              {:else}
                <span class="placeholder-text"
                  >{$t("common.fileDetail.previewPlaceholder")}</span
                >
              {/if}
            </div>
          </div>

          <!-- ヘルプセクション -->
          {#if renameMode === "regex"}
            <RenameHelp showRegexHelp={true} />
          {/if}

          <div class="rename-actions">
            <button
              class="cancel-button"
              onclick={onClose}
              disabled={isRenaming}
            >
              {$t("common.buttons.cancel")}
            </button>
            <button
              class="rename-execute-button"
              onclick={executeRename}
              disabled={!previewResult || isRenaming || !!renameError}
            >
              {#if isRenaming}
                <Loader2 size={16} class="animate-spin" />
                {$t("common.buttons.renaming")}
              {:else}
                {$t("common.buttons.executeRename")}
              {/if}
            </button>
          </div>
        </div>
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
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }

  .modal-content {
    background: white;
    border-radius: 8px;
    padding: 0;
    max-width: 600px;
    width: 90%;
    max-height: 90vh;
    overflow-y: auto;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid #e9ecef;
    background-color: #f8f9fa;
    border-radius: 8px 8px 0 0;
  }

  .modal-header h3 {
    margin: 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: #212529;
  }

  .close-button {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.25rem;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #6c757d;
    transition:
      background-color 0.2s,
      color 0.2s;
  }

  .close-button:hover:not(:disabled) {
    background-color: #e9ecef;
    color: #495057;
  }

  .close-button:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  .modal-body {
    padding: 1.5rem;
  }

  .file-info {
    margin-bottom: 1.5rem;
    padding: 1rem;
    background-color: #f8f9fa;
    border-radius: 6px;
    border: 1px solid #e9ecef;
  }

  .file-name {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 500;
    color: #495057;
  }

  .file-name .icon {
    color: #007acc;
  }

  .rename-mode-selection {
    margin-bottom: 1.5rem;
  }

  .rename-mode-selection h4 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: #495057;
  }

  .radio-group {
    display: flex;
    gap: 1.5rem;
  }

  .radio-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 0.875rem;
    color: #495057;
  }

  .radio-label input[type="radio"] {
    margin: 0;
  }

  .radio-text {
    font-weight: 500;
  }

  .input-hint {
    color: #6c757d;
    font-size: 0.75rem;
    margin-top: 0.25rem;
    font-style: italic;
  }

  .rename-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .rename-input-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .rename-input-group label {
    font-weight: 500;
    color: #495057;
    font-size: 0.875rem;
  }

  .rename-input-group input {
    padding: 0.75rem;
    border: 1px solid #ced4da;
    border-radius: 4px;
    font-size: 0.875rem;
    font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
  }

  .rename-input-group input:focus {
    outline: none;
    border-color: #007acc;
    box-shadow: 0 0 0 2px rgba(0, 122, 204, 0.2);
  }

  .rename-input-group input:disabled {
    background-color: #e9ecef;
    cursor: not-allowed;
  }

  .rename-preview {
    padding: 1rem;
    background-color: #f8f9fa;
    border: 1px solid #dee2e6;
    border-radius: 4px;
  }

  .preview-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #495057;
    font-size: 0.875rem;
  }

  .preview-result {
    font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
    font-size: 0.875rem;
    min-height: 1.5rem;
  }

  .preview-text {
    color: #28a745;
  }

  .error-text {
    color: #dc3545;
  }

  .placeholder-text {
    color: #6c757d;
    font-style: italic;
  }

  .rename-actions {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    padding-top: 1rem;
    border-top: 1px solid #dee2e6;
  }

  .cancel-button {
    padding: 0.75rem 1.5rem;
    background-color: #6c757d;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    transition: background-color 0.2s;
  }

  .cancel-button:hover:not(:disabled) {
    background-color: #545b62;
  }

  .cancel-button:disabled {
    background-color: #adb5bd;
    cursor: not-allowed;
  }

  .rename-execute-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.5rem;
    background-color: #28a745;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    transition: background-color 0.2s;
  }

  .rename-execute-button:hover:not(:disabled) {
    background-color: #218838;
  }

  .rename-execute-button:disabled {
    background-color: #6c757d;
    cursor: not-allowed;
  }
</style>

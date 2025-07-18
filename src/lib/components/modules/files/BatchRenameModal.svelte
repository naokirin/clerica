<script lang="ts">
  import type { File } from "../../../types";
  import {
    batchRenameFiles,
    type BatchRenameOperation,
    type BatchRenameResult,
    previewAdvancedBatchRename,
    executeAdvancedBatchRename,
    type AdvancedBatchRenameOperation,
    type AdvancedBatchRenamePreview,
  } from "../../../api/files";
  import RenameHelp from "./RenameHelp.svelte";

  interface Props {
    isOpen: boolean;
    files: File[];
    onClose: () => void;
    onFilesRenamed: () => void;
  }

  let { isOpen, files, onClose, onFilesRenamed }: Props = $props();

  // リネーム操作の種類
  type RenameOperation =
    | "advanced"
    | "replace"
    | "prefix"
    | "suffix"
    | "sequence"
    | "case";

  // 状態管理
  let operation: RenameOperation = $state("advanced");
  let isProcessing = $state(false);

  // 高度なリネーム用（正規表現 + テンプレート）
  let findPattern = $state("");
  let replacePattern = $state("");
  let useRegex = $state(false);
  let useTemplate = $state(false);

  // 置換操作用（従来の単純置換）
  let findText = $state("");
  let replaceText = $state("");

  // プレフィックス・サフィックス用
  let addText = $state("");

  // 連番用
  let sequenceStart = $state(1);
  let sequenceStep = $state(1);
  let sequencePadding = $state(3);
  let sequencePosition: "prefix" | "suffix" = $state("suffix");

  // ケース変換用
  let caseType: "upper" | "lower" | "title" = $state("lower");

  // プレビューリスト
  interface PreviewItem {
    file: File;
    oldName: string;
    newName: string;
    error: string | null;
  }

  // 高度なリネームのプレビュー状態
  let advancedPreviewList = $state<AdvancedBatchRenamePreview[]>([]);
  let isPreviewLoading = $state(false);
  let previewError = $state<string | null>(null);

  // 高度なリネームのプレビューを更新
  async function updateAdvancedPreview() {
    if (operation !== "advanced" || !findPattern || !replacePattern) {
      advancedPreviewList = [];
      return;
    }

    isPreviewLoading = true;
    previewError = null;

    try {
      const operations: AdvancedBatchRenameOperation[] = files.map((file) => ({
        file_id: file.id.toString(),
        find_pattern: findPattern,
        replace_pattern: replacePattern,
        use_regex: useRegex,
        use_template: useTemplate,
      }));

      advancedPreviewList = await previewAdvancedBatchRename(operations);
    } catch (error: any) {
      console.error("プレビューエラー:", error);
      previewError = error.message || "プレビューの生成に失敗しました";
      advancedPreviewList = [];
    } finally {
      isPreviewLoading = false;
    }
  }

  // パターンの変更を監視してプレビューを更新
  $effect(() => {
    if (operation === "advanced") {
      updateAdvancedPreview();
    }
  });

  // プレビュー計算（リアクティブ）
  let previewList = $derived.by(() => {
    // 高度なリネームの場合は専用のプレビューを使用
    if (operation === "advanced") {
      return advancedPreviewList.map((preview) => ({
        file: files.find((f) => f.id.toString() === preview.file_id)!,
        oldName: preview.old_name,
        newName: preview.new_name,
        error: preview.error || null,
      }));
    }

    const results: PreviewItem[] = [];
    const nameCount = new Map<string, number>();

    // 各ファイルのリネーム結果を計算（従来のモード）
    files.forEach((file, index) => {
      const oldName = file.name;
      let newName = oldName;
      let error: string | null = null;

      try {
        switch (operation) {
          case "replace":
            if (findText) {
              newName = oldName.replaceAll(findText, replaceText);
            }
            break;

          case "prefix":
            newName = addText + oldName;
            break;

          case "suffix":
            const lastDotIndex = oldName.lastIndexOf(".");
            if (lastDotIndex > 0) {
              newName =
                oldName.slice(0, lastDotIndex) +
                addText +
                oldName.slice(lastDotIndex);
            } else {
              newName = oldName + addText;
            }
            break;

          case "sequence":
            const sequenceNumber = (sequenceStart + index * sequenceStep)
              .toString()
              .padStart(sequencePadding, "0");
            const lastDot = oldName.lastIndexOf(".");
            const baseName = lastDot > 0 ? oldName.slice(0, lastDot) : oldName;
            const extension = lastDot > 0 ? oldName.slice(lastDot) : "";

            if (sequencePosition === "prefix") {
              newName = sequenceNumber + "_" + baseName + extension;
            } else {
              newName = baseName + "_" + sequenceNumber + extension;
            }
            break;

          case "case":
            const lastDotCase = oldName.lastIndexOf(".");
            const baseNameCase =
              lastDotCase > 0 ? oldName.slice(0, lastDotCase) : oldName;
            const extensionCase =
              lastDotCase > 0 ? oldName.slice(lastDotCase) : "";

            let transformedBaseName = baseNameCase;
            switch (caseType) {
              case "upper":
                transformedBaseName = baseNameCase.toUpperCase();
                break;
              case "lower":
                transformedBaseName = baseNameCase.toLowerCase();
                break;
              case "title":
                transformedBaseName = baseNameCase
                  .toLowerCase()
                  .replace(/\b\w/g, (l) => l.toUpperCase());
                break;
            }
            newName = transformedBaseName + extensionCase;
            break;
        }

        // ファイル名の妥当性チェック
        if (newName.length === 0) {
          error = "ファイル名が空になります";
        } else if (newName.length > 255) {
          error = "ファイル名が長すぎます";
        } else if (/[<>:"/\\|?*]/.test(newName)) {
          error = "無効な文字が含まれています";
        } else if (newName === oldName) {
          // 変更なしは問題ないが、視覚的に示す
        }
      } catch (e) {
        error = "リネーム処理でエラーが発生しました";
      }

      // 重複チェック用にカウント
      const currentCount = nameCount.get(newName) || 0;
      nameCount.set(newName, currentCount + 1);

      results.push({
        file,
        oldName,
        newName,
        error,
      });
    });

    // 重複チェック
    results.forEach((item) => {
      if (!item.error && nameCount.get(item.newName)! > 1) {
        item.error = "ファイル名が他のファイルと重複します";
      }
    });

    return results;
  });

  // エラーがあるかどうか
  let hasErrors = $derived(previewList.some((item) => item.error !== null));
  let changedCount = $derived(
    previewList.filter((item) => item.oldName !== item.newName && !item.error)
      .length,
  );

  // 実行処理
  const handleRename = async () => {
    if (hasErrors || changedCount === 0) return;

    isProcessing = true;
    try {
      let result: BatchRenameResult;

      if (operation === "advanced") {
        // 高度なリネーム
        const operations: AdvancedBatchRenameOperation[] = previewList
          .filter((item) => item.oldName !== item.newName && !item.error)
          .map((item) => ({
            file_id: item.file.id.toString(),
            find_pattern: findPattern,
            replace_pattern: replacePattern,
            use_regex: useRegex,
            use_template: useTemplate,
          }));

        result = await executeAdvancedBatchRename(operations);
      } else {
        // 従来のバッチリネーム
        const operations: BatchRenameOperation[] = previewList
          .filter((item) => item.oldName !== item.newName && !item.error)
          .map((item) => ({
            old_path: item.file.path,
            new_name: item.newName,
          }));

        result = await batchRenameFiles(operations);
      }

      // 結果の通知
      if (result.successful_files.length > 0) {
        console.log(
          `${result.successful_files.length}件のファイルをリネームしました`,
        );
      }

      if (result.failed_files.length > 0) {
        console.error(
          `${result.failed_files.length}件のファイルのリネームに失敗しました:`,
        );
        result.failed_files.forEach(([path, error]) => {
          console.error(`  ${path}: ${error}`);
        });
        // TODO: エラー通知UI
      }

      onFilesRenamed();
      onClose();
    } catch (error) {
      console.error("バッチリネーム処理でエラーが発生しました:", error);
      // TODO: エラー通知UI
    } finally {
      isProcessing = false;
    }
  };

  // リセット処理
  const resetSettings = () => {
    findPattern = "";
    replacePattern = "";
    useRegex = false;
    useTemplate = false;
    findText = "";
    replaceText = "";
    addText = "";
    sequenceStart = 1;
    sequenceStep = 1;
    sequencePadding = 3;
    sequencePosition = "suffix";
    caseType = "lower";
    advancedPreviewList = [];
    previewError = null;
  };

  // モーダルが開かれたときにリセット
  $effect(() => {
    if (isOpen) {
      resetSettings();
    }
  });

  // キーボードショートカット
  const handleKeydown = (event: KeyboardEvent) => {
    if (event.key === "Escape") {
      onClose();
    } else if (
      event.key === "Enter" &&
      event.ctrlKey &&
      !hasErrors &&
      changedCount > 0 &&
      !isProcessing
    ) {
      handleRename();
    }
  };
</script>

{#if isOpen}
  <div
    class="modal-overlay"
    onclick={onClose}
    onkeydown={handleKeydown}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3>バッチリネーム - {files.length}件のファイル</h3>
        <button class="close-btn" onclick={onClose}>×</button>
      </div>

      <div class="modal-body">
        <!-- 操作選択 -->
        <div class="operation-section">
          <label class="operation-label">リネーム方法:</label>
          <select bind:value={operation} class="operation-select">
            <option value="advanced"
              >高度なリネーム（正規表現・テンプレート）</option
            >
            <option value="replace">文字列の置換</option>
            <option value="prefix">プレフィックスの追加</option>
            <option value="suffix">サフィックスの追加</option>
            <option value="sequence">連番の追加</option>
            <option value="case">大文字・小文字の変換</option>
          </select>
        </div>

        <!-- 操作オプション -->
        <div class="options-section">
          {#if operation === "advanced"}
            <div class="advanced-options">
              <div class="option-group">
                <label>
                  <input type="checkbox" bind:checked={useRegex} />
                  正規表現を使用
                </label>
              </div>
              <div class="option-group">
                <label>
                  <input type="checkbox" bind:checked={useTemplate} />
                  テンプレート機能を使用（{`{{ file.name }}, {{ file.ext }}, {{ n }}`}
                  など）
                </label>
              </div>
              <div class="option-group">
                <label>検索パターン:</label>
                <input
                  type="text"
                  bind:value={findPattern}
                  placeholder={useRegex
                    ? "正規表現パターン (例: ^(.+)\\.(\\w+)$)"
                    : "検索文字列"}
                />
              </div>
              <div class="option-group">
                <label>置換パターン:</label>
                <input
                  type="text"
                  bind:value={replacePattern}
                  placeholder={useTemplate
                    ? `テンプレート (例: {{ file.name }}_{{ n:padding=3 }}.{{ file.ext }})`
                    : useRegex
                      ? "置換文字列 (後方参照: $1, $2)"
                      : "置換文字列"}
                />
              </div>
              <!-- ヘルプセクション -->
              <RenameHelp showRegexHelp={true} />
            </div>
          {:else if operation === "replace"}
            <div class="option-group">
              <label>
                <input type="checkbox" bind:checked={useRegex} />
                正規表現を使用
              </label>
            </div>
            <div class="option-group">
              <label>検索する文字列:</label>
              <input
                type="text"
                bind:value={findText}
                placeholder="検索文字列"
              />
            </div>
            <div class="option-group">
              <label>置換後の文字列:</label>
              <input
                type="text"
                bind:value={replaceText}
                placeholder="置換文字列"
              />
            </div>
          {:else if operation === "prefix"}
            <div class="option-group">
              <label>先頭に追加する文字列:</label>
              <input
                type="text"
                bind:value={addText}
                placeholder="プレフィックス"
              />
            </div>
          {:else if operation === "suffix"}
            <div class="option-group">
              <label>末尾に追加する文字列:</label>
              <input
                type="text"
                bind:value={addText}
                placeholder="サフィックス"
              />
            </div>
          {:else if operation === "sequence"}
            <div class="sequence-options">
              <div class="option-group">
                <label>開始番号:</label>
                <input type="number" bind:value={sequenceStart} min="0" />
              </div>
              <div class="option-group">
                <label>増分:</label>
                <input type="number" bind:value={sequenceStep} min="1" />
              </div>
              <div class="option-group">
                <label>桁数:</label>
                <input
                  type="number"
                  bind:value={sequencePadding}
                  min="1"
                  max="10"
                />
              </div>
              <div class="option-group">
                <label>位置:</label>
                <select bind:value={sequencePosition}>
                  <option value="prefix">先頭</option>
                  <option value="suffix">末尾</option>
                </select>
              </div>
            </div>
          {:else if operation === "case"}
            <div class="option-group">
              <label>変換方法:</label>
              <select bind:value={caseType}>
                <option value="upper">すべて大文字</option>
                <option value="lower">すべて小文字</option>
                <option value="title">単語の先頭を大文字</option>
              </select>
            </div>
          {/if}
        </div>

        <!-- プレビューリスト -->
        <div class="preview-section">
          <div class="preview-header">
            <h4>プレビュー</h4>
            <div class="preview-summary">
              {#if isPreviewLoading}
                <span class="loading-text">プレビューを生成中...</span>
              {:else if previewError}
                <span class="error-text">エラー: {previewError}</span>
              {:else}
                {changedCount}件が変更されます
                {#if hasErrors}
                  <span class="error-count"
                    >（{previewList.filter((item) => item.error)
                      .length}件のエラー）</span
                  >
                {/if}
              {/if}
            </div>
          </div>

          <div class="preview-list">
            <div class="preview-list-header">
              <div class="col-old">変更前</div>
              <div class="col-new">変更後</div>
            </div>
            <div class="preview-items">
              {#each previewList as item (item.file.id)}
                <div
                  class="preview-item"
                  class:error={item.error}
                  class:changed={item.oldName !== item.newName}
                >
                  <div class="col-old" title={item.oldName}>
                    {item.oldName}
                  </div>
                  <div class="col-new" title={item.newName}>
                    {item.newName}
                    {#if item.error}
                      <span class="error-icon" title={item.error}>⚠️</span>
                    {:else if item.oldName !== item.newName}
                      <span class="changed-icon">✓</span>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          </div>
        </div>
      </div>

      <div class="modal-actions">
        <button class="cancel-btn" onclick={onClose} disabled={isProcessing}>
          キャンセル
        </button>
        <button
          class="rename-btn"
          onclick={handleRename}
          disabled={hasErrors || changedCount === 0 || isProcessing}
        >
          {#if isProcessing}
            処理中...
          {:else}
            リネーム実行 ({changedCount}件)
          {/if}
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
    max-width: 800px;
    width: 90%;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px 20px 0 20px;
  }

  .modal-header h3 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: #333;
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    color: #666;
    padding: 0;
    width: 30px;
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-btn:hover {
    color: #333;
  }

  .modal-body {
    padding: 20px;
    overflow-y: auto;
    flex: 1;
  }

  .operation-section {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 20px;
  }

  .operation-label {
    font-weight: 500;
    color: #333;
  }

  .operation-select {
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.9rem;
  }

  .options-section {
    margin-bottom: 20px;
    padding: 16px;
    background: #f8f9fa;
    border-radius: 6px;
  }

  .option-group {
    margin-bottom: 12px;
  }

  .option-group:last-child {
    margin-bottom: 0;
  }

  .option-group label {
    display: block;
    margin-bottom: 4px;
    font-weight: 500;
    color: #555;
  }

  .option-group input,
  .option-group select {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.9rem;
  }

  .sequence-options {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
  }

  .preview-section {
    margin-bottom: 20px;
  }

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .preview-header h4 {
    margin: 0;
    color: #333;
  }

  .preview-summary {
    font-size: 0.9rem;
    color: #666;
  }

  .error-count {
    color: #f44336;
    font-weight: 500;
  }

  .preview-list {
    border: 1px solid #ddd;
    border-radius: 6px;
    max-height: 300px;
    overflow: hidden;
  }

  .preview-list-header {
    display: grid;
    grid-template-columns: 1fr 1fr;
    background: #f5f5f5;
    padding: 12px;
    font-weight: 500;
    color: #333;
    border-bottom: 1px solid #ddd;
  }

  .preview-items {
    max-height: 250px;
    overflow-y: auto;
  }

  .preview-item {
    display: grid;
    grid-template-columns: 1fr 1fr;
    padding: 8px 12px;
    border-bottom: 1px solid #eee;
    font-size: 0.9rem;
  }

  .preview-item:last-child {
    border-bottom: none;
  }

  .preview-item.error {
    background: #fff5f5;
    color: #d32f2f;
  }

  .preview-item.changed {
    background: #f0f8ff;
  }

  .col-old,
  .col-new {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .error-icon {
    margin-left: 8px;
  }

  .changed-icon {
    margin-left: 8px;
    color: #4caf50;
  }

  .modal-actions {
    display: flex;
    gap: 12px;
    padding: 0 20px 20px 20px;
    justify-content: flex-end;
  }

  .cancel-btn,
  .rename-btn {
    padding: 10px 20px;
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

  .cancel-btn:hover:not(:disabled) {
    background: #e0e0e0;
    border-color: #bbb;
  }

  .rename-btn {
    background: #2196f3;
    color: white;
  }

  .rename-btn:hover:not(:disabled) {
    background: #1976d2;
  }

  .rename-btn:disabled,
  .cancel-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* 高度なリネーム用のスタイル */
  .advanced-options {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .loading-text {
    color: #666;
    font-style: italic;
  }

  .error-text {
    color: #f44336;
    font-weight: 500;
  }
</style>

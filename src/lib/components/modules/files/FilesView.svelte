<script lang="ts">
  import type {
    File,
    FileWithTags,
    FileCategory,
    SortOptions,
  } from "../../../types";
  import FileList from "./FileList.svelte";
  import FileSortControl from "../../modules/files/FileSortControl.svelte";
  import FileCategoryFilters from "./FileCategoryFilters.svelte";
  import Pagination from "../../parts/Pagination.svelte";
  import { t } from "$lib/i18n";
  import { selectedFileIds } from "../../../stores/files";
  import DeleteConfirmDialog from "./DeleteConfirmDialog.svelte";
  import BatchRenameModal from "./BatchRenameModal.svelte";
  import { deleteFiles, type DeleteResult } from "../../../api/files";
  import { viewMode, type ViewMode } from "../../../stores/common";
  import { List, Grid } from "@lucide/svelte";
  import Button from "../../parts/Button.svelte";

  interface Props {
    files: File[];
    filesWithTags: FileWithTags[];
    selectedCategory: FileCategory;
    categoryCounts: Record<FileCategory, number>;
    currentPage: number;
    totalFiles: number;
    totalPages: number;
    itemsPerPage: number;
    selectedDirectoryId: string | "all";
    sortOptions: SortOptions;
    onSelectFile: (file: File) => void;
    onSelectCategory: (category: FileCategory) => Promise<void>;
    onGoToPage: (page: number) => Promise<void>;
    onGoToPreviousPage: () => Promise<void>;
    onGoToNextPage: () => Promise<void>;
    onGoToFirstPage: () => Promise<void>;
    onGoToLastPage: () => Promise<void>;
    onSortChange: (options: SortOptions) => Promise<void>;
    onOpenRenameModal?: (file: File) => void;
  }

  let {
    files,
    filesWithTags,
    selectedCategory,
    categoryCounts,
    currentPage,
    totalFiles,
    totalPages,
    itemsPerPage,
    selectedDirectoryId,
    sortOptions,
    onSelectFile,
    onSelectCategory,
    onGoToPage,
    onGoToPreviousPage,
    onGoToNextPage,
    onGoToFirstPage,
    onGoToLastPage,
    onSortChange,
    onOpenRenameModal,
  }: Props = $props();


  // 選択中のファイル数
  let selectedCount = $derived($selectedFileIds.size);
  let hasSelection = $derived(selectedCount > 0);
  let isMultipleSelection = $derived(selectedCount > 1);

  // 全選択・全解除
  const handleSelectAll = () => {
    selectedFileIds.update(() => {
      const newSelected = new Set<string>();
      filesWithTags.forEach((fileWithTags) => {
        newSelected.add(fileWithTags.file.id);
      });
      return newSelected;
    });
  };

  const handleClearSelection = () => {
    selectedFileIds.update(() => new Set());
  };

  // モーダルの状態
  let isDeleteDialogOpen = $state(false);
  let isBatchRenameModalOpen = $state(false);

  // 削除処理
  const handleDeleteSelected = async () => {
    if (!hasSelection) return;
    isDeleteDialogOpen = true;
  };

  // 削除確認
  const confirmDelete = async () => {
    if (!hasSelection) return;

    const selectedIds = Array.from($selectedFileIds)
      .filter(id => id && id.trim().length > 0);
    
    if (selectedIds.length === 0) {
      console.error("有効なファイルIDが選択されていません");
      return;
    }

    try {
      const result: DeleteResult = await deleteFiles(selectedIds);

      // 結果をユーザーに通知
      if (result.successful_files.length > 0) {
        console.log(
          `${result.successful_files.length}件のファイルが削除されました`,
        );
      }

      if (result.failed_files.length > 0) {
        console.error(
          `${result.failed_files.length}件のファイルの削除に失敗しました:`,
        );
        result.failed_files.forEach(([path, error]) => {
          console.error(`  ${path}: ${error}`);
        });
        // TODO: エラー通知UI
      }

      // 選択をクリア
      selectedFileIds.update(() => new Set());

      // ファイル一覧を再読み込み
      // TODO: 親コンポーネントから再読み込み関数を受け取る
    } catch (error) {
      console.error("削除処理でエラーが発生しました:", error);
      // TODO: エラー通知UI
    } finally {
      isDeleteDialogOpen = false;
    }
  };

  // 削除キャンセル
  const cancelDelete = () => {
    isDeleteDialogOpen = false;
  };

  // リネーム処理
  const handleRenameSelected = async () => {
    if (!hasSelection) return;

    if (selectedCount === 1) {
      // 単一ファイルの場合は直接リネームモーダルを開く
      const selectedId = Array.from($selectedFileIds)[0];
      const selectedFile = filesWithTags.find(
        (f) => f.file.id === selectedId,
      )?.file;
      if (selectedFile && onOpenRenameModal) {
        selectedFileIds.update(() => new Set());
        onOpenRenameModal(selectedFile);
      }
    } else {
      // 複数ファイルの場合はバッチリネームモーダルを使用
      isBatchRenameModalOpen = true;
    }
  };

  // バッチリネームモーダルのクローズ処理
  const closeBatchRenameModal = () => {
    isBatchRenameModalOpen = false;
  };

  // バッチリネーム完了後の処理
  const handleBatchRenameCompleted = () => {
    selectedFileIds.update(() => new Set());
    // TODO: ファイル一覧を再読み込み
  };

  // 選択中のファイル情報を取得
  let selectedFiles = $derived.by(() => {
    const selectedIds = Array.from($selectedFileIds);
    return filesWithTags
      .filter((fileWithTags) => selectedIds.includes(fileWithTags.file.id))
      .map((fileWithTags) => fileWithTags.file);
  });
</script>

<div class="files-view">
  <div class="files-header">
    <div class="header-top">
      <div class="header-left">
        <h2>{$t("common.files.title")}</h2>
      </div>

      <!-- 表示モード切り替えボタン -->
      <div class="view-mode-switcher">
        <button
          class="view-mode-btn {$viewMode === 'list' ? 'active' : ''}"
          onclick={() => viewMode.set("list")}
          title="リスト表示"
          aria-label="リスト表示"
          aria-pressed={$viewMode === "list"}
        >
          <List size={18} />
        </button>
        <button
          class="view-mode-btn {$viewMode === 'grid' ? 'active' : ''}"
          onclick={() => viewMode.set("grid")}
          title="グリッド表示"
          aria-label="グリッド表示"
          aria-pressed={$viewMode === "grid"}
        >
          <Grid size={18} />
        </button>
      </div>
    </div>

    <div class="header-bottom">
      <div class="files-stats">
        <span class="total-files">
          {selectedCategory === "all"
            ? $t("common.files.total")
            : $t(`common.files.category.${selectedCategory}`)}:
          {$t("common.files.totalFiles", {
            count: totalFiles.toLocaleString(),
          } as any)}
        </span>
        {#if totalPages > 1}
          <span class="page-info">
            {$t("common.pagination.page")}
            {currentPage}
            {$t("common.pagination.of", { total: totalPages } as any)}
            ({$t("common.pagination.showing", {
              start: ((currentPage - 1) * itemsPerPage + 1).toLocaleString(),
              end: Math.min(
                currentPage * itemsPerPage,
                totalFiles,
              ).toLocaleString(),
              total: totalFiles.toLocaleString(),
            } as any)})
          </span>
        {/if}
      </div>
    </div>
  </div>

  <!-- 削除確認ダイアログ -->
  <DeleteConfirmDialog
    isOpen={isDeleteDialogOpen}
    fileCount={selectedCount}
    onConfirm={confirmDelete}
    onCancel={cancelDelete}
  />

  <!-- バッチリネームモーダル -->
  <BatchRenameModal
    isOpen={isBatchRenameModalOpen}
    files={selectedFiles}
    onClose={closeBatchRenameModal}
    onFilesRenamed={handleBatchRenameCompleted}
  />

  <!-- ファイル種別フィルター -->
  <FileCategoryFilters {selectedCategory} {categoryCounts} {onSelectCategory} />

  <!-- コンテキストアクションバー -->
  {#if hasSelection}
    <div class="selection-action-bar">
      <div class="selection-info">
        <span class="selection-count">{selectedCount}件のファイルを選択中</span>
        <Button
          id="clear-selection-files-btn"
          variant="neutral"
          size="small"
          text="選択を解除"
          onclick={handleClearSelection}
        />
      </div>
      <div class="selection-actions">
        <Button
          id="rename-selected-files-btn"
          variant="success"
          size="small"
          text="リネーム"
          onclick={handleRenameSelected}
          {...({ title: selectedCount === 1
            ? "選択したファイルをリネーム"
            : `${selectedCount}件のファイルをバッチリネーム` } as any)}
        />
        <Button
          id="delete-selected-files-btn"
          variant="danger"
          size="small"
          text="削除"
          onclick={handleDeleteSelected}
        />
      </div>
    </div>
  {:else}
    <div class="bulk-actions">
      <Button
        id="select-all-files-btn"
        variant="neutral"
        size="small"
        text="すべて選択"
        onclick={handleSelectAll}
      />
    </div>
  {/if}

  <div class="pagination-controls">
    <Pagination
      {currentPage}
      {totalPages}
      onGoToPage={(page) => onGoToPage(page)}
      onGoToPreviousPage={() => onGoToPreviousPage()}
      onGoToNextPage={() => onGoToNextPage()}
      onGoToFirstPage={() => onGoToFirstPage()}
      onGoToLastPage={() => onGoToLastPage()}
    />
    <div class="sort-section">
      <FileSortControl
        sortField={sortOptions.field}
        sortOrder={sortOptions.order}
        {onSortChange}
      />
    </div>
  </div>

  <FileList
    {filesWithTags}
    {currentPage}
    {totalPages}
    viewMode={$viewMode}
    emptyMessage={totalFiles === 0 && selectedDirectoryId === "all"
      ? $t("common.files.emptyDirectories")
      : $t("common.files.noFiles")}
    showEmptyState={files.length === 0}
    {onSelectFile}
    {onGoToPage}
    {onGoToPreviousPage}
    {onGoToNextPage}
    {onGoToFirstPage}
    {onGoToLastPage}
  />
</div>

<style>
  .files-view {
    max-width: 800px;
    margin: 0 auto;
  }

  .files-view h2 {
    margin: 0 0 1.5rem 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: #111827;
  }

  .files-header .header-top,
  .files-header .header-bottom {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1rem;
  }

  .header-left h2 {
    margin: 0 0 0.5rem 0;
  }

  .files-stats {
    text-align: right;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 0.25rem;
  }

  .total-files {
    font-size: 1rem;
    font-weight: 600;
    color: #374151;
  }

  .page-info {
    font-size: 0.875rem;
    color: #6b7280;
  }

  .pagination-controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin: 1rem 0;
  }


  .selection-action-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    margin: 1rem 0;
    background: #e3f2fd;
    border: 1px solid #2196f3;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .selection-info {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .selection-count {
    font-weight: 500;
    color: #1976d2;
  }


  .selection-actions {
    display: flex;
    gap: 8px;
  }


  .bulk-actions {
    display: flex;
    justify-content: flex-end;
    margin: 1rem 0;
  }

  /* 表示モード切り替えボタン */
  .view-mode-switcher {
    display: flex;
    gap: 0;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    overflow: hidden;
    background: #ffffff;
  }

  .view-mode-btn {
    padding: 8px 12px;
    background: #ffffff;
    border: none;
    cursor: pointer;
    color: #6b7280;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
  }

  .view-mode-btn:not(:last-child)::after {
    content: "";
    position: absolute;
    right: 0;
    top: 20%;
    bottom: 20%;
    width: 1px;
    background: #d1d5db;
  }

  .view-mode-btn:hover {
    background: #f3f4f6;
    color: #374151;
  }

  .view-mode-btn.active {
    background: #3b82f6;
    color: #ffffff;
  }

  .view-mode-btn.active:hover {
    background: #2563eb;
  }

  .view-mode-btn.active:not(:last-child)::after {
    background: transparent;
  }

  /* レスポンシブ対応 */
  @media (max-width: 768px) {
    .files-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 1rem;
    }

    .files-stats {
      align-items: flex-start;
      width: 100%;
    }
  }
</style>

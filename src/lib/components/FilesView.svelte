<script lang="ts">
  import type {
    File,
    FileWithTags,
    FileCategory,
    FileCategoryInfo,
    SortOptions,
  } from "../types";
  import FileList from "./FileList.svelte";
  import SortControl from "./SortControl.svelte";
  import FileCategoryFilters from "./FileCategoryFilters.svelte";
  import Pagination from "./Pagination.svelte";
  import { t } from "$lib/i18n";
  import { selectedFileIds } from "../stores/files";
  import DeleteConfirmDialog from "./DeleteConfirmDialog.svelte";
  import BatchRenameModal from "./BatchRenameModal.svelte";
  import { deleteFiles, type DeleteResult } from "../api/files";
  import { createEventDispatcher } from "svelte";

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
  }: Props = $props();
  
  // イベントディスパッチャー
  const dispatch = createEventDispatcher();
  
  // 選択中のファイル数
  let selectedCount = $derived($selectedFileIds.size);
  let hasSelection = $derived(selectedCount > 0);
  let isMultipleSelection = $derived(selectedCount > 1);
  
  // 全選択・全解除
  const handleSelectAll = () => {
    selectedFileIds.update(() => {
      const newSelected = new Set<number>();
      filesWithTags.forEach(fileWithTags => {
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
    
    const selectedIds = Array.from($selectedFileIds);
    
    try {
      const result: DeleteResult = await deleteFiles(selectedIds);
      
      // 結果をユーザーに通知
      if (result.successful_files.length > 0) {
        console.log(`${result.successful_files.length}件のファイルが削除されました`);
      }
      
      if (result.failed_files.length > 0) {
        console.error(`${result.failed_files.length}件のファイルの削除に失敗しました:`);
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
      console.error('削除処理でエラーが発生しました:', error);
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
      const selectedFile = filesWithTags.find(f => f.file.id === selectedId)?.file;
      if (selectedFile) {
        selectedFileIds.update(() => new Set());
        dispatch('open-rename-modal', selectedFile);
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
      .filter(fileWithTags => selectedIds.includes(fileWithTags.file.id))
      .map(fileWithTags => fileWithTags.file);
  });

</script>

<div class="files-view">
  <div class="files-header">
    <h2>{$t("common.files.title")}</h2>
    <div class="files-stats">
      <span class="total-files">
        {selectedCategory === "all"
          ? $t("common.files.total")
          : $t(`common.files.category.${selectedCategory}`)}:
        {$t("common.files.totalFiles", { count: totalFiles.toLocaleString() })}
      </span>
      {#if totalPages > 1}
        <span class="page-info">
          {$t("common.pagination.page")} {currentPage} {$t("common.pagination.of", { total: totalPages })}
          ({$t("common.pagination.showing", { 
            start: ((currentPage - 1) * itemsPerPage + 1).toLocaleString(),
            end: Math.min(currentPage * itemsPerPage, totalFiles).toLocaleString(),
            total: totalFiles.toLocaleString()
          })})
        </span>
      {/if}
    </div>
  </div>
  
  <!-- コンテキストアクションバー -->
  {#if hasSelection}
    <div class="selection-action-bar">
      <div class="selection-info">
        <span class="selection-count">{selectedCount}件のファイルを選択中</span>
        <button class="clear-selection-btn" onclick={handleClearSelection}>
          選択を解除
        </button>
      </div>
      <div class="selection-actions">
        <button 
          class="action-btn rename-btn" 
          onclick={handleRenameSelected}
          title={selectedCount === 1 ? "選択したファイルをリネーム" : `${selectedCount}件のファイルをバッチリネーム`}
        >
          リネーム
        </button>
        <button class="action-btn delete-btn" onclick={handleDeleteSelected}>
          削除
        </button>
      </div>
    </div>
  {:else}
    <div class="bulk-actions">
      <button class="select-all-btn" onclick={handleSelectAll}>
        すべて選択
      </button>
    </div>
  {/if}
  
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
  <FileCategoryFilters
    {selectedCategory}
    {categoryCounts}
    onSelectCategory={onSelectCategory}
  />

  <div class="pagination-controls">
    <Pagination
      {currentPage}
      {totalPages}
      on:goToPage={async (e) => await onGoToPage(e.detail.page)}
      on:goToPreviousPage={async () => await onGoToPreviousPage()}
      on:goToNextPage={async () => await onGoToNextPage()}
      on:goToFirstPage={async () => await onGoToFirstPage()}
      on:goToLastPage={async () => await onGoToLastPage()}
    />
    <div class="sort-section">
      <SortControl
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
  .files-header h2 {
    margin: 0 0 1rem 0;
  }

  .pagination-controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin: 1rem 0;
  }

  .sort-section {
    /* Sort control positioned on the right of pagination */
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
  
  .clear-selection-btn {
    padding: 4px 8px;
    background: transparent;
    border: 1px solid #2196f3;
    border-radius: 4px;
    color: #2196f3;
    cursor: pointer;
    font-size: 0.875rem;
    transition: all 0.2s ease;
  }
  
  .clear-selection-btn:hover {
    background: #2196f3;
    color: white;
  }
  
  .selection-actions {
    display: flex;
    gap: 8px;
  }
  
  .action-btn {
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s ease;
  }
  
  .rename-btn {
    background: #4caf50;
    color: white;
  }
  
  .rename-btn:hover:not(:disabled) {
    background: #45a049;
  }
  
  .rename-btn:disabled {
    background: #cccccc;
    color: #666;
    cursor: not-allowed;
  }
  
  /* 複数選択時のリネームボタンのスタイル調整は不要（無効化しないため） */
  
  .delete-btn {
    background: #f44336;
    color: white;
  }
  
  .delete-btn:hover {
    background: #da190b;
  }
  
  .bulk-actions {
    display: flex;
    justify-content: flex-end;
    margin: 1rem 0;
  }
  
  .select-all-btn {
    padding: 8px 16px;
    background: #f5f5f5;
    border: 1px solid #ddd;
    border-radius: 4px;
    cursor: pointer;
    color: #333;
    font-weight: 500;
    transition: all 0.2s ease;
  }
  
  .select-all-btn:hover {
    background: #e0e0e0;
    border-color: #bbb;
  }
</style>

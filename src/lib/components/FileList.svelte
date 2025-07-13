<script lang="ts">
  import type { FileWithTags } from "../types";
  import type { ViewMode } from "../stores/common";
  import FileItemDisplay from "./FileItemDisplay.svelte";

  interface Props {
    filesWithTags: FileWithTags[];
    currentPage: number;
    totalPages: number;
    viewMode: ViewMode;
    emptyMessage?: string;
    showEmptyState?: boolean;
    onSelectFile: (file: any) => void;
    onGoToPage: (page: number) => Promise<void>;
    onGoToPreviousPage: () => Promise<void>;
    onGoToNextPage: () => Promise<void>;
    onGoToFirstPage: () => Promise<void>;
    onGoToLastPage: () => Promise<void>;
  }

  let {
    filesWithTags,
    currentPage,
    totalPages,
    viewMode,
    emptyMessage = "ファイルが見つかりませんでした", // デフォルト、親から渡されることを想定
    showEmptyState = true,
    onSelectFile,
    onGoToPage,
    onGoToPreviousPage,
    onGoToNextPage,
    onGoToFirstPage,
    onGoToLastPage,
  }: Props = $props();
</script>

<div class="file-list" class:grid-view={viewMode === 'grid'}>
  {#each filesWithTags as fileWithTags (fileWithTags.file.id)}
    <FileItemDisplay
      file={fileWithTags.file}
      tags={fileWithTags.tags}
      {viewMode}
      {onSelectFile}
    />
  {/each}
  {#if filesWithTags.length === 0 && showEmptyState}
    <div class="no-files">
      {emptyMessage}
    </div>
  {/if}
</div>

{#if totalPages > 1}
  <div class="pagination-controls pagination-bottom">
    <button
      class="pagination-btn"
      onclick={onGoToFirstPage}
      disabled={currentPage === 1}
    >
      ≪
    </button>
    <button
      class="pagination-btn"
      onclick={onGoToPreviousPage}
      disabled={currentPage === 1}
    >
      ‹
    </button>

    {#each Array.from({ length: Math.min(7, totalPages) }, (_, i) => {
      let start = Math.max(1, currentPage - 3);
      let end = Math.min(totalPages, start + 6);
      start = Math.max(1, end - 6);
      return start + i;
    }).filter((page) => page <= totalPages) as page}
      <button
        class="pagination-btn {currentPage === page ? 'active' : ''}"
        onclick={() => onGoToPage(page)}
      >
        {page}
      </button>
    {/each}

    <button
      class="pagination-btn"
      onclick={onGoToNextPage}
      disabled={currentPage === totalPages}
    >
      ›
    </button>
    <button
      class="pagination-btn"
      onclick={onGoToLastPage}
      disabled={currentPage === totalPages}
    >
      ≫
    </button>
  </div>
{/if}

<style>
  .file-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding: 1rem;
  }

  .file-list.grid-view {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1.5rem;
  }

  .no-files {
    grid-column: 1 / -1;
    text-align: center;
    color: #6b7280;
    font-size: 1rem;
    padding: 2rem;
    background: #f9fafb;
    border-radius: 8px;
    border: 2px dashed #d1d5db;
  }

  .pagination-controls {
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 1rem 0;
  }

  .pagination-bottom {
    justify-content: center;
    gap: 0.5rem;
  }

  .pagination-btn {
    padding: 0.5rem 0.75rem;
    border: 1px solid #d1d5db;
    background: #ffffff;
    color: #374151;
    border-radius: 0.375rem;
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 0.875rem;
    font-weight: 500;
    min-width: 2.5rem;
    height: 2.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .pagination-btn:hover:not(:disabled) {
    background: #f3f4f6;
    border-color: #9ca3af;
  }

  .pagination-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    background: #f9fafb;
  }

  .pagination-btn.active {
    background: #3b82f6;
    color: #ffffff;
    border-color: #3b82f6;
  }

  .pagination-btn.active:hover {
    background: #2563eb;
    border-color: #2563eb;
  }
</style>

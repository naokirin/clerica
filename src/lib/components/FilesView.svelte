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
</style>

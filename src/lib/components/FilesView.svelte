<script lang="ts">
  import type { File, FileCategory, FileCategoryInfo, SortOptions } from "../types.js";
  import FileItemDisplay from "./FileItemDisplay.svelte";
  import SortControl from "./SortControl.svelte";

  interface Props {
    files: File[];
    selectedCategory: FileCategory;
    categoryCounts: Record<FileCategory, number>;
    currentPage: number;
    totalFiles: number;
    totalPages: number;
    selectedDirectoryId: string | "all";
    sortOptions: SortOptions;
    onSelectFile: (file: File) => void;
    onSelectCategory: (category: FileCategory) => void;
    onGoToPage: (page: number) => void;
    onGoToPreviousPage: () => void;
    onGoToNextPage: () => void;
    onGoToFirstPage: () => void;
    onGoToLastPage: () => void;
    onSortChange: (options: SortOptions) => void;
  }

  let {
    files,
    selectedCategory,
    categoryCounts,
    currentPage,
    totalFiles,
    totalPages,
    selectedDirectoryId,
    sortOptions,
    onSelectFile,
    onSelectCategory,
    onGoToPage,
    onGoToPreviousPage,
    onGoToNextPage,
    onGoToFirstPage,
    onGoToLastPage,
    onSortChange
  }: Props = $props();

  const fileCategories: FileCategoryInfo[] = [
    {
      key: "all",
      label: "すべて",
      icon: "📁",
      mimeTypes: [],
      extensions: []
    },
    {
      key: "image",
      label: "画像",
      icon: "🖼️",
      mimeTypes: ["image/"],
      extensions: ["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg", "ico", "tiff", "raw"]
    },
    {
      key: "audio",
      label: "音声",
      icon: "🎵",
      mimeTypes: ["audio/"],
      extensions: ["mp3", "wav", "ogg", "flac", "aac", "m4a", "wma", "opus"]
    },
    {
      key: "video",
      label: "動画",
      icon: "🎬",
      mimeTypes: ["video/"],
      extensions: ["mp4", "avi", "mov", "wmv", "flv", "webm", "mkv", "m4v", "3gp"]
    },
    {
      key: "document",
      label: "ドキュメント",
      icon: "📄",
      mimeTypes: ["application/pdf", "application/msword", "application/vnd.", "text/"],
      extensions: ["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "md", "html", "htm", "css", "js", "json", "xml", "csv", "rtf"]
    },
    {
      key: "archive",
      label: "アーカイブ",
      icon: "📦",
      mimeTypes: ["application/zip", "application/x-rar", "application/x-7z", "application/x-tar", "application/gzip"],
      extensions: ["zip", "rar", "7z", "tar", "gz", "bz2", "xz", "lzma"]
    },
    {
      key: "other",
      label: "その他",
      icon: "📄",
      mimeTypes: [],
      extensions: []
    }
  ];

  const itemsPerPage = 25;
</script>

<div class="files-view">
  <div class="files-header">
    <h2>ファイル一覧</h2>
    <div class="files-stats">
      <span class="total-files">
        {selectedCategory === "all" ? "合計" : fileCategories.find(c => c.key === selectedCategory)?.label}: 
        {totalFiles.toLocaleString()} ファイル
      </span>
      {#if totalPages > 1}
        <span class="page-info">
          ページ {currentPage} / {totalPages} 
          ({((currentPage - 1) * itemsPerPage + 1).toLocaleString()} - {Math.min(currentPage * itemsPerPage, totalFiles).toLocaleString()})
        </span>
      {/if}
    </div>
  </div>

  <!-- ファイル種別フィルター -->
  <div class="file-category-filters">
    {#each fileCategories as category (category.key)}
      <button
        class="category-filter-btn {selectedCategory === category.key ? 'active' : ''}"
        onclick={() => onSelectCategory(category.key)}
        disabled={category.key !== "all" && categoryCounts[category.key] === 0}
      >
        <span class="category-icon">{category.icon}</span>
        <span class="category-label">{category.label}</span>
        <span class="category-count">({categoryCounts[category.key].toLocaleString()})</span>
      </button>
    {/each}
  </div>

  {#if totalPages > 1}
    <div class="pagination-controls">
      <div class="pagination-buttons">
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
        
        {#each Array.from({length: Math.min(7, totalPages)}, (_, i) => {
          let start = Math.max(1, currentPage - 3);
          let end = Math.min(totalPages, start + 6);
          start = Math.max(1, end - 6);
          return start + i;
        }).filter(page => page <= totalPages) as page}
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
      <div class="sort-section">
        <SortControl 
          sortField={sortOptions.field}
          sortOrder={sortOptions.order}
          onSortChange={onSortChange}
        />
      </div>
    </div>
  {/if}

  <div class="file-list">
    {#each files as file (file.id)}
      <FileItemDisplay 
        file={file} 
        onSelectFile={onSelectFile}
      />
    {/each}
    {#if files.length === 0}
      <div class="no-files">
        {#if totalFiles === 0 && selectedDirectoryId === "all"}
          ディレクトリを追加してファイルをスキャンしてください
        {:else}
          対象のファイルが存在しません
        {/if}
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
      
      {#each Array.from({length: Math.min(7, totalPages)}, (_, i) => {
        let start = Math.max(1, currentPage - 3);
        let end = Math.min(totalPages, start + 6);
        start = Math.max(1, end - 6);
        return start + i;
      }).filter(page => page <= totalPages) as page}
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

  .pagination-buttons {
    display: flex;
    gap: 0.5rem;
  }

  .sort-section {
    /* Sort control positioned on the right of pagination */
  }
</style>
<script lang="ts">
  import { Search } from "lucide-svelte";
  import type { SearchResult, FileCategory, FileCategoryInfo } from "../types.js";
  import { formatFileSize } from "../utils.js";

  interface Props {
    searchQuery: string;
    searchResults: SearchResult[];
    filteredResults: SearchResult[];
    selectedCategory: FileCategory;
    categoryCounts: Record<FileCategory, number>;
    currentPage: number;
    totalPages: number;
    onSearchQueryChange: (query: string) => void;
    onSearch: () => void;
    onSelectFile: (file: any) => void;
    onSelectCategory: (category: FileCategory) => void;
    onGoToPage: (page: number) => void;
    onGoToPreviousPage: () => void;
    onGoToNextPage: () => void;
    onGoToFirstPage: () => void;
    onGoToLastPage: () => void;
  }

  let {
    searchQuery = $bindable(),
    searchResults,
    filteredResults,
    selectedCategory,
    categoryCounts,
    currentPage,
    totalPages,
    onSearchQueryChange,
    onSearch,
    onSelectFile,
    onSelectCategory,
    onGoToPage,
    onGoToPreviousPage,
    onGoToNextPage,
    onGoToFirstPage,
    onGoToLastPage
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
      label: "圧縮ファイル",
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

<div class="search-view">
  <div class="search-header">
    <h2>ファイル検索</h2>
    {#if searchResults.length > 0}
      <div class="search-stats">
        <span class="total-results">
          {selectedCategory === "all" ? "検索結果" : fileCategories.find(c => c.key === selectedCategory)?.label}: 
          {filteredResults.length.toLocaleString()} 件
        </span>
        {#if totalPages > 1}
          <span class="page-info">
            ページ {currentPage} / {totalPages} 
            ({((currentPage - 1) * itemsPerPage + 1).toLocaleString()} - {Math.min(currentPage * itemsPerPage, filteredResults.length).toLocaleString()})
          </span>
        {/if}
      </div>
    {/if}
  </div>

  <div class="search-controls">
    <input
      type="text"
      placeholder="ファイル名を入力..."
      bind:value={searchQuery}
      class="search-input"
    />
    <button onclick={onSearch} class="search-button">
      <Search size={16} />
      検索
    </button>
  </div>

  <!-- 検索結果のファイル種別フィルター -->
  {#if searchResults.length > 0}
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
  {/if}

  {#if totalPages > 1}
    <div class="pagination-controls">
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

  <div class="search-results">
    {#each filteredResults as result (result.file.id)}
      <div class="search-result-item" onclick={() => onSelectFile(result.file)}>
        <div class="file-icon">
          {#if result.file.is_directory}
            📁
          {:else if result.file.mime_type?.startsWith('image/')}
            🖼️
          {:else if result.file.mime_type?.startsWith('video/')}
            🎬
          {:else if result.file.mime_type?.startsWith('audio/')}
            🎵
          {:else if result.file.mime_type?.includes('pdf')}
            📄
          {:else if result.file.mime_type?.includes('text')}
            📝
          {:else}
            📄
          {/if}
        </div>
        <div class="search-result-details">
          <div class="result-file-name">{result.file.name}</div>
          <div class="file-info">
            {#if !result.file.is_directory}
              {formatFileSize(result.file.file_size || result.file.size)} 
              {#if result.file.mime_type}
                • {result.file.mime_type}
              {:else if result.file.file_type}
                • {result.file.file_type}
              {/if}
            {:else}
              ディレクトリ
            {/if}
          </div>
          <div class="file-path">{result.file.path}</div>
          <div class="result-tags">
            {#each result.tags as tag (tag.id)}
              <span
                class="result-tag"
                style="background-color: {tag.color}"
              >
                {tag.name}
              </span>
            {/each}
          </div>
        </div>
      </div>
    {/each}
    {#if searchResults.length === 0 && searchQuery}
      <div class="no-results">
        検索結果が見つかりませんでした
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
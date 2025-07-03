<script lang="ts">
  import type { File, FileCategory, FileCategoryInfo } from "../types.js";
  import { formatFileSize, formatDate, isImageFile, getImageUrl } from "../utils.js";
  import { onMount } from 'svelte';

  interface Props {
    files: File[];
    selectedCategory: FileCategory;
    categoryCounts: Record<FileCategory, number>;
    currentPage: number;
    totalFiles: number;
    totalPages: number;
    selectedDirectoryId: string | "all";
    onSelectFile: (file: File) => void;
    onSelectCategory: (category: FileCategory) => void;
    onGoToPage: (page: number) => void;
    onGoToPreviousPage: () => void;
    onGoToNextPage: () => void;
    onGoToFirstPage: () => void;
    onGoToLastPage: () => void;
  }

  let {
    files,
    selectedCategory,
    categoryCounts,
    currentPage,
    totalFiles,
    totalPages,
    selectedDirectoryId,
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

  // 画像URLキャッシュ
  let imageUrlCache = new Map<string, string>();

  // 画像URLを取得する関数
  async function loadImageUrl(filePath: string): Promise<string> {
    if (imageUrlCache.has(filePath)) {
      return imageUrlCache.get(filePath)!;
    }
    
    const url = await getImageUrl(filePath);
    imageUrlCache.set(filePath, url);
    return url;
  }
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

  <div class="file-list">
    {#each files as file (file.id)}
      <div class="file-item" onclick={() => onSelectFile(file)}>
        <div class="file-icon">
          {#if file.is_directory}
            <span class="icon-emoji">📁</span>
          {:else if isImageFile(file)}
            <div class="image-preview">
              {#await loadImageUrl(file.path)}
                <span class="icon-emoji loading">🖼️</span>
              {:then imageUrl}
                <img 
                  src={imageUrl} 
                  alt={file.name}
                  class="thumbnail"
                  onerror={(e) => {
                    // 画像読み込みエラー時はアイコンに戻す
                    console.error('Failed to load image:', imageUrl);
                    e.currentTarget.style.display = 'none';
                    e.currentTarget.nextElementSibling.style.display = 'block';
                  }}
                />
                <span class="icon-emoji fallback" style="display: none;">🖼️</span>
              {:catch}
                <span class="icon-emoji">🖼️</span>
              {/await}
            </div>
          {:else if file.mime_type?.startsWith('video/')}
            <span class="icon-emoji">🎬</span>
          {:else if file.mime_type?.startsWith('audio/')}
            <span class="icon-emoji">🎵</span>
          {:else if file.mime_type?.includes('pdf')}
            <span class="icon-emoji">📄</span>
          {:else if file.mime_type?.includes('text')}
            <span class="icon-emoji">📝</span>
          {:else}
            <span class="icon-emoji">📄</span>
          {/if}
        </div>
        <div class="file-details">
          <div class="file-name">{file.name}</div>
          <div class="file-info">
            {#if !file.is_directory}
              {formatFileSize(file.file_size || file.size)} 
              {#if file.mime_type}
                • {file.mime_type}
              {:else if file.file_type}
                • {file.file_type}
              {/if}
            {:else}
              ディレクトリ
            {/if}
          </div>
          <div class="file-path">{file.path}</div>
          <div class="file-meta">
            {#if file.modified_at}
              更新: {formatDate(file.modified_at)}
            {/if}
            {#if file.permissions}
              • 権限: {file.permissions}
            {/if}
          </div>
        </div>
      </div>
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
  .file-icon {
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .icon-emoji {
    font-size: 32px;
    line-height: 1;
  }

  .image-preview {
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    overflow: hidden;
    background-color: #f5f5f5;
  }

  .thumbnail {
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: 4px;
    transition: transform 0.2s ease;
  }

  .thumbnail:hover {
    transform: scale(1.1);
  }

  .loading {
    opacity: 0.6;
    animation: pulse 1.5s ease-in-out infinite alternate;
  }

  @keyframes pulse {
    from {
      opacity: 0.6;
    }
    to {
      opacity: 1;
    }
  }

  .fallback {
    color: #999;
  }

  /* ファイルアイテムのレイアウト調整 */
  .file-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px;
    border-radius: 6px;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }

  .file-item:hover {
    background-color: #f8f9fa;
  }

  .file-details {
    flex: 1;
    min-width: 0;
  }

  .file-name {
    font-weight: 500;
    color: #333;
    word-break: break-word;
  }

  .file-info {
    font-size: 0.875rem;
    color: #666;
    margin-top: 2px;
  }

  .file-path {
    font-size: 0.75rem;
    color: #999;
    margin-top: 2px;
    word-break: break-all;
  }

  .file-meta {
    font-size: 0.75rem;
    color: #999;
    margin-top: 2px;
  }
</style>
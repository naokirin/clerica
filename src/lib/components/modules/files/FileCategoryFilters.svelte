<script lang="ts">
  import type { FileCategory, FileCategoryInfo } from "../../../types";
  import { t } from "$lib/i18n";

  interface Props {
    selectedCategory: FileCategory;
    categoryCounts: Record<FileCategory, number>;
    fileCategories?: FileCategoryInfo[];
    onSelectCategory: (category: FileCategory) => Promise<void>;
  }

  let {
    selectedCategory,
    categoryCounts,
    fileCategories,
    onSelectCategory,
  }: Props = $props();

  const defaultFileCategories = $derived([
    {
      key: "all",
      label: $t("common.files.category.all"),
      icon: "📁",
      mimeTypes: [],
      extensions: [],
    },
    {
      key: "image",
      label: $t("common.files.category.image"),
      icon: "🖼️",
      mimeTypes: ["image/"],
      extensions: [
        "jpg",
        "jpeg",
        "png",
        "gif",
        "bmp",
        "webp",
        "svg",
        "ico",
        "tiff",
        "raw",
      ],
    },
    {
      key: "audio",
      label: $t("common.files.category.audio"),
      icon: "🎵",
      mimeTypes: ["audio/"],
      extensions: ["mp3", "wav", "ogg", "flac", "aac", "m4a", "wma", "opus"],
    },
    {
      key: "video",
      label: $t("common.files.category.video"),
      icon: "🎬",
      mimeTypes: ["video/"],
      extensions: [
        "mp4",
        "avi",
        "mov",
        "wmv",
        "flv",
        "webm",
        "mkv",
        "m4v",
        "3gp",
      ],
    },
    {
      key: "document",
      label: $t("common.files.category.document"),
      icon: "📄",
      mimeTypes: [
        "application/pdf",
        "application/msword",
        "application/vnd.",
        "text/",
      ],
      extensions: [
        "pdf",
        "doc",
        "docx",
        "xls",
        "xlsx",
        "ppt",
        "pptx",
        "txt",
        "md",
        "html",
        "htm",
        "css",
        "js",
        "json",
        "xml",
        "csv",
        "rtf",
      ],
    },
    {
      key: "archive",
      label: $t("common.files.category.archive"),
      icon: "📦",
      mimeTypes: [
        "application/zip",
        "application/x-rar",
        "application/x-7z",
        "application/x-tar",
        "application/gzip",
      ],
      extensions: ["zip", "rar", "7z", "tar", "gz", "bz2", "xz", "lzma"],
    },
    {
      key: "other",
      label: $t("common.files.category.other"),
      icon: "📄",
      mimeTypes: [],
      extensions: [],
    },
  ]);

  const categories = $derived(() => fileCategories ?? defaultFileCategories);

  async function handleCategorySelect(category: FileCategory) {
    await onSelectCategory(category);
  }
</script>

<div class="file-category-filters">
  {#each categories() as category (category.key)}
    <button
      class="category-filter-btn {selectedCategory === category.key
        ? 'active'
        : ''}"
      onclick={() => handleCategorySelect(category.key as FileCategory)}
      disabled={category.key !== "all" &&
        categoryCounts[category.key as FileCategory] === 0}
    >
      <span class="category-icon">{category.icon}</span>
      <span class="category-label">{category.label}</span>
      <span class="category-count"
        >({categoryCounts[category.key as FileCategory].toLocaleString()})</span
      >
    </button>
  {/each}
</div>

<style>
  .file-category-filters {
    display: flex;
    gap: 0.75rem;
    margin-bottom: 1.5rem;
    padding: 1rem;
    background-color: #f8fafc;
    border-radius: 0.5rem;
    border: 1px solid #e2e8f0;
    flex-wrap: wrap;
  }

  .category-filter-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    background-color: white;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.875rem;
    color: #374151;
    white-space: nowrap;
  }

  .category-filter-btn:hover:not(:disabled) {
    background-color: #f9fafb;
    border-color: #9ca3af;
    transform: translateY(-1px);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  }

  .category-filter-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    background-color: #f9fafb;
    color: #9ca3af;
  }

  .category-filter-btn.active {
    background-color: #3b82f6;
    border-color: #3b82f6;
    color: white;
    box-shadow: 0 2px 8px rgba(59, 130, 246, 0.3);
  }

  .category-filter-btn.active:hover {
    background-color: #2563eb;
    border-color: #2563eb;
  }

  .category-icon {
    font-size: 1rem;
  }

  .category-label {
    font-weight: 500;
  }

  .category-count {
    font-size: 0.75rem;
    opacity: 0.8;
    font-weight: 400;
  }

  .category-filter-btn.active .category-count {
    opacity: 0.9;
  }

  /* レスポンシブ対応 */
  @media (max-width: 768px) {
    .file-category-filters {
      gap: 0.5rem;
      padding: 0.75rem;
    }

    .category-filter-btn {
      padding: 0.5rem 0.75rem;
      font-size: 0.75rem;
    }

    .category-label {
      display: none;
    }

    .category-icon {
      font-size: 1.125rem;
    }
  }
</style>

<script lang="ts">
  import type { FileCategory, FileCategoryInfo } from "../types";
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
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .category-filter-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background-color: #f8f9fa;
    border: 1px solid #dee2e6;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 0.875rem;
    color: #495057;
  }

  .category-filter-btn:hover:not(:disabled) {
    background-color: #e9ecef;
    border-color: #adb5bd;
  }

  .category-filter-btn.active {
    background-color: #007bff;
    border-color: #007bff;
    color: white;
  }

  .category-filter-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
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
  }
</style>

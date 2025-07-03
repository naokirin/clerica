<script lang="ts">
  import { Search, Plus, X } from "lucide-svelte";
  import type {
    SearchResult,
    FileCategory,
    FileCategoryInfo,
    MetadataSearchFilter,
    CustomMetadataKey,
  } from "../types.js";
  import FileItemDisplay from "./FileItemDisplay.svelte";

  interface Props {
    searchQuery: string;
    searchResults: SearchResult[];
    filteredResults: SearchResult[];
    selectedCategory: FileCategory;
    categoryCounts: Record<FileCategory, number>;
    currentPage: number;
    totalPages: number;
    metadataSearchFilters: MetadataSearchFilter[];
    availableMetadataKeys: CustomMetadataKey[];
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
    metadataSearchFilters = $bindable(),
    availableMetadataKeys,
    onSearchQueryChange,
    onSearch,
    onSelectFile,
    onSelectCategory,
    onGoToPage,
    onGoToPreviousPage,
    onGoToNextPage,
    onGoToFirstPage,
    onGoToLastPage,
  }: Props = $props();

  const fileCategories: FileCategoryInfo[] = [
    {
      key: "all",
      label: "すべて",
      icon: "📁",
      mimeTypes: [],
      extensions: [],
    },
    {
      key: "image",
      label: "画像",
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
      label: "音声",
      icon: "🎵",
      mimeTypes: ["audio/"],
      extensions: ["mp3", "wav", "ogg", "flac", "aac", "m4a", "wma", "opus"],
    },
    {
      key: "video",
      label: "動画",
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
      label: "ドキュメント",
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
      label: "アーカイブ",
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
      label: "その他",
      icon: "📄",
      mimeTypes: [],
      extensions: [],
    },
  ];

  const itemsPerPage = 25;

  function getOperatorLabel(operator: string): string {
    switch (operator) {
      case "equals":
        return "等しい";
      case "contains":
        return "含む";
      case "greater_than":
        return "より大きい";
      case "less_than":
        return "より小さい";
      case "not_equals":
        return "等しくない";
      default:
        return operator;
    }
  }

  function getOperatorsForDataType(dataType: string): string[] {
    switch (dataType) {
      case "text":
        return ["equals", "contains", "not_equals"];
      case "number":
      case "date":
        return ["equals", "greater_than", "less_than", "not_equals"];
      case "boolean":
        return ["equals", "not_equals"];
      case "json":
        return ["contains", "equals"];
      default:
        return ["equals", "contains"];
    }
  }

  function addMetadataFilter() {
    metadataSearchFilters = [
      ...metadataSearchFilters,
      {
        keyId: "",
        keyName: "",
        displayName: "",
        dataType: "text",
        value: "",
        operator: "equals",
      },
    ];
  }

  function removeMetadataFilter(index: number) {
    metadataSearchFilters = metadataSearchFilters.filter((_, i) => i !== index);
  }

  function updateMetadataFilter(index: number, filter: MetadataSearchFilter) {
    metadataSearchFilters = metadataSearchFilters.map((f, i) =>
      i === index ? filter : f,
    );
  }
</script>

<div class="search-view">
  <div class="search-header">
    <h2>ファイル検索</h2>
    {#if searchResults.length > 0}
      <div class="search-stats">
        <span class="total-results">
          {selectedCategory === "all"
            ? "検索結果"
            : fileCategories.find((c) => c.key === selectedCategory)?.label}:
          {filteredResults.length.toLocaleString()} 件
        </span>
        {#if totalPages > 1}
          <span class="page-info">
            ページ {currentPage} / {totalPages}
            ({((currentPage - 1) * itemsPerPage + 1).toLocaleString()} - {Math.min(
              currentPage * itemsPerPage,
              filteredResults.length,
            ).toLocaleString()})
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

  <!-- カスタムメタデータ検索フィルタ -->
  <div class="metadata-search-section">
    <div class="metadata-search-header">
      <h3>カスタムメタデータ検索</h3>
      <button onclick={addMetadataFilter} class="add-filter-btn">
        <Plus size={16} />
        フィルタを追加
      </button>
    </div>

    {#if metadataSearchFilters.length > 0}
      <div class="metadata-filters">
        {#each metadataSearchFilters as filter, index (index)}
          <div class="metadata-filter">
            <select
              value={filter.keyId}
              onchange={(e) => {
                const keyId = e.currentTarget.value;
                const key = availableMetadataKeys.find((k) => k.id === keyId);
                if (key) {
                  const operators = getOperatorsForDataType(key.data_type);
                  updateMetadataFilter(index, {
                    ...filter,
                    keyId: key.id,
                    keyName: key.name,
                    displayName: key.display_name,
                    dataType: key.data_type,
                    operator: operators.includes(filter.operator)
                      ? filter.operator
                      : operators[0],
                  });
                }
              }}
              class="metadata-key-select"
            >
              <option value="">メタデータキーを選択...</option>
              {#each availableMetadataKeys as key}
                <option value={key.id}>{key.display_name}</option>
              {/each}
            </select>

            {#if filter.keyId}
              <select
                value={filter.operator}
                onchange={(e) => {
                  updateMetadataFilter(index, {
                    ...filter,
                    operator: e.currentTarget.value,
                  });
                }}
                class="metadata-operator-select"
              >
                {#each getOperatorsForDataType(filter.dataType) as operator}
                  <option value={operator}>{getOperatorLabel(operator)}</option>
                {/each}
              </select>

              {#if filter.dataType === "boolean"}
                <select
                  value={filter.value}
                  onchange={(e) => {
                    updateMetadataFilter(index, {
                      ...filter,
                      value: e.currentTarget.value,
                    });
                  }}
                  class="metadata-value-input"
                >
                  <option value="">選択...</option>
                  <option value="true">はい</option>
                  <option value="false">いいえ</option>
                </select>
              {:else if filter.dataType === "date"}
                <input
                  type="date"
                  value={filter.value}
                  oninput={(e) => {
                    updateMetadataFilter(index, {
                      ...filter,
                      value: e.currentTarget.value,
                    });
                  }}
                  class="metadata-value-input"
                />
              {:else if filter.dataType === "number"}
                <input
                  type="number"
                  value={filter.value}
                  placeholder="数値を入力..."
                  oninput={(e) => {
                    updateMetadataFilter(index, {
                      ...filter,
                      value: e.currentTarget.value,
                    });
                  }}
                  class="metadata-value-input"
                />
              {:else}
                <input
                  type="text"
                  value={filter.value}
                  placeholder="値を入力..."
                  oninput={(e) => {
                    updateMetadataFilter(index, {
                      ...filter,
                      value: e.currentTarget.value,
                    });
                  }}
                  class="metadata-value-input"
                />
              {/if}
            {/if}

            <button
              onclick={() => removeMetadataFilter(index)}
              class="remove-filter-btn"
              title="フィルタを削除"
            >
              <X size={16} />
            </button>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <!-- 検索結果のファイル種別フィルター -->
  {#if searchResults.length > 0}
    <div class="file-category-filters">
      {#each fileCategories as category (category.key)}
        <button
          class="category-filter-btn {selectedCategory === category.key
            ? 'active'
            : ''}"
          onclick={() => onSelectCategory(category.key)}
          disabled={category.key !== "all" &&
            categoryCounts[category.key] === 0}
        >
          <span class="category-icon">{category.icon}</span>
          <span class="category-label">{category.label}</span>
          <span class="category-count"
            >({categoryCounts[category.key].toLocaleString()})</span
          >
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

  <div class="search-results">
    {#each filteredResults as result (result.file.id)}
      <FileItemDisplay 
        file={result.file} 
        tags={result.tags}
        onSelectFile={onSelectFile}
      />
    {/each}
    {#if searchResults.length === 0 && (searchQuery || metadataSearchFilters.some(f => f.keyId && f.value))}
      <div class="no-results">検索結果が見つかりませんでした</div>
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
</div>

<style>
  .metadata-search-section {
    margin: 1rem 0;
    padding: 1rem;
    border: 1px solid #e5e5e5;
    border-radius: 8px;
    background-color: #fafafa;
  }

  .metadata-search-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .metadata-search-header h3 {
    margin: 0;
    font-size: 1rem;
    color: #333;
  }

  .add-filter-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
    transition: background-color 0.2s;
  }

  .add-filter-btn:hover {
    background-color: #0056b3;
  }

  .metadata-filters {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .metadata-filter {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    background-color: white;
    border: 1px solid #ddd;
    border-radius: 6px;
  }

  .metadata-key-select,
  .metadata-operator-select {
    min-width: 150px;
    padding: 0.5rem;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 0.875rem;
  }

  .metadata-value-input {
    flex: 1;
    min-width: 200px;
    padding: 0.5rem;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 0.875rem;
  }

  .remove-filter-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    background-color: #dc3545;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .remove-filter-btn:hover {
    background-color: #c82333;
  }
</style>

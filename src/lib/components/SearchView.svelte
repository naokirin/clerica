<script lang="ts">
  import { Search, Plus, X, Tag } from "lucide-svelte";
  import type {
    SearchResult,
    FileCategory,
    FileCategoryInfo,
    MetadataSearchFilter,
    CustomMetadataKey,
    SortOptions,
    Tag as TagType,
  } from "../types.js";
  import FileList from "./FileList.svelte";
  import SortControl from "./SortControl.svelte";

  interface Props {
    searchQuery: string;
    searchResults: SearchResult[];
    filteredResults: SearchResult[];
    selectedCategory: FileCategory;
    categoryCounts: Record<FileCategory, number>;
    currentPage: number;
    totalPages: number;
    itemsPerPage: number;
    selectedTags: string[];
    allTags: TagType[];
    topTags: TagType[];
    tagSearchResults: TagType[];
    metadataSearchFilters: MetadataSearchFilter[];
    metadataLogic: 'AND' | 'OR';
    availableMetadataKeys: CustomMetadataKey[];
    sortOptions: SortOptions;
    onSearchQueryChange: (query: string) => void;
    onSearch: () => void;
    onSelectFile: (file: any) => void;
    onSelectCategory: (category: FileCategory) => void;
    onGoToPage: (page: number) => void;
    onGoToPreviousPage: () => void;
    onGoToNextPage: () => void;
    onGoToFirstPage: () => void;
    onGoToLastPage: () => void;
    onTagAdd: (tagId: string) => void;
    onTagRemove: (tagId: string) => void;
    onTagSearch: (query: string) => void;
    onMetadataLogicChange: (logic: 'AND' | 'OR') => void;
    onSortChange: (options: SortOptions) => void;
  }

  let {
    searchQuery = $bindable(),
    searchResults,
    filteredResults,
    selectedCategory,
    categoryCounts,
    currentPage,
    totalPages,
    itemsPerPage,
    selectedTags,
    allTags,
    topTags,
    tagSearchResults,
    metadataSearchFilters = $bindable(),
    metadataLogic,
    availableMetadataKeys,
    sortOptions,
    onSearchQueryChange,
    onSearch,
    onSelectFile,
    onSelectCategory,
    onGoToPage,
    onGoToPreviousPage,
    onGoToNextPage,
    onGoToFirstPage,
    onGoToLastPage,
    onTagAdd,
    onTagRemove,
    onTagSearch,
    onMetadataLogicChange,
    onSortChange,
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

  <!-- タグフィルタ -->
  <div class="tag-filter-section">
    <div class="tag-filter-header">
      <Tag size={16} />
      <h3>タグで絞り込み</h3>
    </div>

    <!-- 選択されたタグ -->
    {#if selectedTags.length > 0}
      <div class="selected-tags">
        <h4>選択中のタグ:</h4>
        <div class="tag-chips">
          {#each selectedTags as tagId}
            {@const selectedTag = allTags.find(tag => tag.id === tagId)}
            {#if selectedTag}
              <div class="tag-chip selected">
                <span class="tag-color" style="background-color: {selectedTag.color}"></span>
                <span class="tag-name">{selectedTag.name}</span>
                <button onclick={() => onTagRemove(tagId)} class="tag-remove-btn">
                  <X size={12} />
                </button>
              </div>
            {/if}
          {/each}
        </div>
      </div>
    {/if}

    <!-- 上位タグ -->
    <div class="top-tags">
      <h4>よく使われるタグ (上位10件):</h4>
      <div class="tag-chips">
        {#each topTags as tag}
          {#if !selectedTags.includes(tag.id)}
            <button onclick={() => onTagAdd(tag.id)} class="tag-chip">
              <span class="tag-color" style="background-color: {tag.color}"></span>
              <span class="tag-name">{tag.name}</span>
            </button>
          {/if}
        {/each}
      </div>
    </div>

    <!-- タグ検索 -->
    <div class="tag-search">
      <h4>タグを検索:</h4>
      <input
        type="text"
        placeholder="タグ名を入力..."
        oninput={(e) => onTagSearch(e.target.value)}
        class="tag-search-input"
      />
      {#if tagSearchResults.length > 0}
        <div class="tag-search-results">
          {#each tagSearchResults as tag}
            {#if !selectedTags.includes(tag.id)}
              <button onclick={() => onTagAdd(tag.id)} class="tag-search-result">
                <span class="tag-color" style="background-color: {tag.color}"></span>
                <span class="tag-name">{tag.name}</span>
              </button>
            {/if}
          {/each}
        </div>
      {/if}
    </div>
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

    {#if metadataSearchFilters.length > 1}
      <div class="metadata-logic-section">
        <label class="metadata-logic-label">検索条件の結合方法:</label>
        <div class="metadata-logic-options">
          <label class="metadata-logic-option">
            <input
              type="radio"
              value="AND"
              checked={metadataLogic === 'AND'}
              onchange={() => onMetadataLogicChange('AND')}
              class="metadata-logic-radio"
            />
            すべての条件に一致 (AND)
          </label>
          <label class="metadata-logic-option">
            <input
              type="radio"
              value="OR"
              checked={metadataLogic === 'OR'}
              onchange={() => onMetadataLogicChange('OR')}
              class="metadata-logic-radio"
            />
            いずれかの条件に一致 (OR)
          </label>
        </div>
      </div>
    {/if}

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
      <div class="sort-section">
        <SortControl 
          sortField={sortOptions.field}
          sortOrder={sortOptions.order}
          onSortChange={onSortChange}
        />
      </div>
    </div>
  {/if}

  <FileList
    filesWithTags={filteredResults.map(result => ({ file: result.file, tags: result.tags }))}
    {currentPage}
    {totalPages}
    emptyMessage={searchResults.length === 0 && (searchQuery || metadataSearchFilters.some(f => f.keyId && f.value)) 
      ? "検索結果が見つかりませんでした" 
      : "検索条件を入力してください"}
    showEmptyState={filteredResults.length === 0}
    {onSelectFile}
    {onGoToPage}
    {onGoToPreviousPage}
    {onGoToNextPage}
    {onGoToFirstPage}
    {onGoToLastPage}
  />
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

  .search-header h2 {
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

  /* タグフィルタスタイル */
  .tag-filter-section {
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 1rem;
  }

  .tag-filter-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .tag-filter-header h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: #374151;
  }

  .selected-tags, .top-tags, .tag-search {
    margin-bottom: 1rem;
  }

  .selected-tags h4, .top-tags h4, .tag-search h4 {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    font-weight: 500;
    color: #4b5563;
  }

  .tag-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .tag-chip {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
    background: #ffffff;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-size: 0.75rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .tag-chip:hover {
    background: #f3f4f6;
    border-color: #9ca3af;
  }

  .tag-chip.selected {
    background: #dbeafe;
    border-color: #3b82f6;
  }

  .tag-color {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .tag-name {
    color: #374151;
    font-weight: 500;
  }

  .tag-remove-btn {
    background: none;
    border: none;
    color: #6b7280;
    cursor: pointer;
    padding: 2px;
    border-radius: 2px;
    transition: all 0.2s ease;
  }

  .tag-remove-btn:hover {
    background: #f3f4f6;
    color: #ef4444;
  }

  .tag-search-input {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-size: 0.875rem;
    margin-bottom: 0.5rem;
  }

  .tag-search-input:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .tag-search-results {
    background: #ffffff;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    max-height: 200px;
    overflow-y: auto;
  }

  .tag-search-result {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    background: none;
    border: none;
    border-bottom: 1px solid #e5e7eb;
    cursor: pointer;
    transition: background-color 0.2s ease;
    text-align: left;
  }

  .tag-search-result:last-child {
    border-bottom: none;
  }

  .tag-search-result:hover {
    background: #f3f4f6;
  }
</style>

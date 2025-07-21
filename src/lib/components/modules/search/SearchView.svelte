<script lang="ts">
  import { Search, Plus, X, Tag, List, Grid } from "@lucide/svelte";
  import Button from "../../parts/Button.svelte";
  import TextInput from "../../parts/TextInput.svelte";
  import NumberInput from "../../parts/NumberInput.svelte";
  import DateInput from "../../parts/DateInput.svelte";
  import RadioButtonGroup from "../../parts/RadioButtonGroup.svelte";
  import Select from "../../parts/Select.svelte";
  import type {
    SearchResult,
    FileCategory,
    MetadataSearchFilter,
    CustomMetadataKey,
    SortOptions,
    Tag as TagType,
  } from "../../../types";
  import FileList from "../files/FileList.svelte";
  import FileSortControl from "../../modules/files/FileSortControl.svelte";
  import FileCategoryFilters from "../files/FileCategoryFilters.svelte";
  import Pagination from "../../parts/Pagination.svelte";
  import { t } from "$lib/i18n";
  import { selectedFileIds } from "../../../stores/files";
  import DeleteConfirmDialog from "../files/DeleteConfirmDialog.svelte";
  import BatchRenameModal from "../files/BatchRenameModal.svelte";
  import { deleteFiles, type DeleteResult } from "../../../api/files";
  import { viewMode } from "../../../stores/common";

  interface Props {
    searchQuery: string;
    searchResults: SearchResult[];
    filteredResults: SearchResult[];
    allFilteredResults: SearchResult[];
    totalSearchResults: number;
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
    metadataLogic: "AND" | "OR";
    availableMetadataKeys: CustomMetadataKey[];
    sortOptions: SortOptions;
    selectedDirectoryId: string | "all";
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
    onMetadataLogicChange: (logic: "AND" | "OR") => void;
    onSortChange: (options: SortOptions) => void;
    onTagsUpdated?: () => void; // タグ更新時のコールバックを追加
  }

  let {
    searchQuery = $bindable(),
    searchResults,
    filteredResults,
    allFilteredResults,
    totalSearchResults,
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
    selectedDirectoryId,
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
    onTagsUpdated,
  }: Props = $props();

  // 選択中のファイル数
  let selectedCount = $derived($selectedFileIds.size);
  let hasSelection = $derived(selectedCount > 0);
  let isMultipleSelection = $derived(selectedCount > 1);

  // モーダルの状態
  let isDeleteDialogOpen = $state(false);
  let isBatchRenameModalOpen = $state(false);

  // メタデータロジック選択のオプション
  const metadataLogicOptions = [
    { value: "AND", label: $t("common.search.metadataLogicAnd") },
    { value: "OR", label: $t("common.search.metadataLogicOr") }
  ];

  // 全選択・全解除
  const handleSelectAll = () => {
    selectedFileIds.update(() => {
      const newSelected = new Set<string>();
      filteredResults.forEach((result) => {
        newSelected.add(result.file.id);
      });
      return newSelected;
    });
  };

  const handleClearSelection = () => {
    selectedFileIds.update(() => new Set());
  };

  // 削除処理
  const handleDeleteSelected = async () => {
    if (!hasSelection) return;
    isDeleteDialogOpen = true;
  };

  // 削除確認
  const confirmDelete = async () => {
    if (!hasSelection) return;

    const selectedIds = Array.from($selectedFileIds).map(id => Number(id));

    try {
      const result: DeleteResult = await deleteFiles(selectedIds);

      // 結果をユーザーに通知
      if (result.successful_files.length > 0) {
        console.log(
          `${result.successful_files.length}件のファイルが削除されました`,
        );
      }

      if (result.failed_files.length > 0) {
        console.error(
          `${result.failed_files.length}件のファイルの削除に失敗しました:`,
        );
        result.failed_files.forEach(([path, error]) => {
          console.error(`  ${path}: ${error}`);
        });
        // TODO: エラー通知UI
      }

      // 選択をクリア
      selectedFileIds.update(() => new Set());

      // 検索結果を再読み込み
      onSearch();
    } catch (error) {
      console.error("削除処理でエラーが発生しました:", error);
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
      // 単一ファイルの場合は既存のファイル詳細モーダルを使用
      const selectedId = Array.from($selectedFileIds)[0];
      const selectedResult = filteredResults.find(
        (r) => r.file.id === selectedId,
      );
      if (selectedResult) {
        selectedFileIds.update(() => new Set());
        onSelectFile(selectedResult.file);
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
    onSearch(); // 検索結果を再読み込み
  };

  // 選択中のファイル情報を取得
  let selectedFiles = $derived.by(() => {
    const selectedIds = Array.from($selectedFileIds);
    return filteredResults
      .filter((result) => selectedIds.includes(result.file.id))
      .map((result) => result.file);
  });

  // タグが更新された時の処理
  function handleTagsUpdated() {
    if (onTagsUpdated) {
      onTagsUpdated();
    }
  }

  // allTagsが更新された時にタグ関連の情報も更新
  $effect(() => {
    // allTagsが変更された時に、選択されたタグの存在確認を行う
    if (selectedTags.length > 0 && allTags.length > 0) {
      const validSelectedTags = selectedTags.filter((tagId) =>
        allTags.some((tag) => tag.id === tagId),
      );

      console.log(allTags);

      // 無効なタグが選択されている場合は削除
      if (validSelectedTags.length !== selectedTags.length) {
        console.log("無効なタグが選択されているため、選択から削除します");
        // 親コンポーネントに通知して選択タグを更新
        validSelectedTags.forEach((tagId) => {
          if (!selectedTags.includes(tagId)) {
            onTagAdd(tagId);
          }
        });
        selectedTags.forEach((tagId) => {
          if (!validSelectedTags.includes(tagId)) {
            onTagRemove(tagId);
          }
        });
      }
    }
  });

  function getOperatorLabel(operator: string): string {
    switch (operator) {
      case "equals":
        return $t("common.search.operators.equals");
      case "contains":
        return $t("common.search.operators.contains");
      case "greater_than":
        return $t("common.search.operators.greaterThan");
      case "less_than":
        return $t("common.search.operators.lessThan");
      case "not_equals":
        return $t("common.search.operators.notEquals");
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
    <div class="header-top">
      <div class="header-left">
        <h2>{$t("common.search.title")}</h2>
      </div>

      <!-- 表示モード切り替えボタン -->
      <div class="view-mode-switcher">
        <button
          class="view-mode-btn {$viewMode === 'list' ? 'active' : ''}"
          onclick={() => viewMode.set("list")}
          title="リスト表示"
          aria-label="リスト表示"
          aria-pressed={$viewMode === "list"}
        >
          <List size={18} />
        </button>
        <button
          class="view-mode-btn {$viewMode === 'grid' ? 'active' : ''}"
          onclick={() => viewMode.set("grid")}
          title="グリッド表示"
          aria-label="グリッド表示"
          aria-pressed={$viewMode === "grid"}
        >
          <Grid size={18} />
        </button>
      </div>
    </div>

    <div class="header-bottom">
      {#if searchResults.length > 0}
        <div class="search-stats">
          <span class="total-results">
            {selectedCategory === "all"
              ? $t("common.search.results")
              : $t(`common.files.category.${selectedCategory}`)}:
            {totalSearchResults.toLocaleString()}
            {$t("common.search.items")}
          </span>
          <span class="page-info">
            {#if totalPages > 1}
              ページ {currentPage} / {totalPages} ({(
                (currentPage - 1) * itemsPerPage +
                1
              ).toLocaleString()} - {Math.min(
                currentPage * itemsPerPage,
                totalSearchResults,
              ).toLocaleString()} / {totalSearchResults.toLocaleString()} 件を表示)
            {:else}
              {totalSearchResults.toLocaleString()} 件を表示
            {/if}
          </span>
        </div>
      {/if}
    </div>
  </div>

  <div class="search-controls">
    <TextInput
      id="search-query-input"
      placeholder={$t("common.search.inputPlaceholder")}
      bind:value={searchQuery}
    />
    <Button
      id="search-button"
      onclick={onSearch}
      class="search-button"
      iconName="Search"
      text={$t("common.buttons.search")}
    />
  </div>

  <!-- タグフィルタ -->
  <div class="tag-filter-section">
    <div class="tag-filter-header">
      <Tag size={16} />
      <h3>{$t("common.search.tagFilter")}</h3>
    </div>

    <!-- 選択されたタグ -->
    {#if selectedTags.length > 0}
      <div class="selected-tags">
        <h4>{$t("common.search.selectedTags")}:</h4>
        <div class="tag-chips">
          {#each selectedTags as tagId}
            {@const selectedTag = allTags.find((tag) => tag.id === tagId)}
            <div class="tag-chip selected">
              <span
                class="tag-color"
                style="background-color: {selectedTag?.color || '#666'}"
              ></span>
              <span class="tag-name"
                >{selectedTag?.name ||
                  $t("common.search.tagId", { id: tagId } as any)}</span
              >
              <button onclick={() => onTagRemove(tagId)} class="tag-remove-btn">
                <X size={12} />
              </button>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    <!-- 上位タグ -->
    <div class="top-tags">
      <h4>{$t("common.search.topTags")}:</h4>
      <div class="tag-chips">
        {#each topTags as tag}
          {#if !selectedTags.includes(tag.id)}
            <button onclick={() => onTagAdd(tag.id)} class="tag-chip">
              <span class="tag-color" style="background-color: {tag.color}"
              ></span>
              <span class="tag-name">{tag.name}</span>
            </button>
          {/if}
        {/each}
      </div>
    </div>

    <!-- タグ検索 -->
    <div class="tag-search">
      <h4>{$t("common.search.searchTags")}:</h4>
      <TextInput
        id="tag-search-input"
        value=""
        placeholder={$t("common.search.tagSearchPlaceholder")}
        oninput={(e) => onTagSearch((e.target as HTMLInputElement)?.value || "")}
        class="tag-search-input"
      />
      {#if tagSearchResults.length > 0}
        <div class="tag-search-results">
          {#each tagSearchResults as tag}
            {#if !selectedTags.includes(tag.id)}
              <button
                onclick={() => onTagAdd(tag.id)}
                class="tag-search-result"
              >
                <span class="tag-color" style="background-color: {tag.color}"
                ></span>
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
      <h3>{$t("common.search.metadataSearch")}</h3>
      <Button
        id="add-metadata-filter-btn"
        onclick={addMetadataFilter}
        class="add-filter-btn"
        iconName="Plus"
        text={$t("common.search.addFilter")}
      />
    </div>

    {#if metadataSearchFilters.length > 1}
      <RadioButtonGroup
        title={$t("common.search.metadataLogicLabel")}
        options={metadataLogicOptions}
        value={metadataLogic}
        onValueChange={onMetadataLogicChange as any}
      />
    {/if}

    {#if metadataSearchFilters.length > 0}
      <div class="metadata-filters">
        {#each metadataSearchFilters as filter, index (index)}
          <div class="metadata-filter">
            <Select
              options={[
                { value: "", label: $t("common.search.selectMetadataKey") },
                ...availableMetadataKeys.map(key => ({ value: key.id, label: key.display_name }))
              ]}
              value={filter.keyId}
              on:change={(e) => {
                const target = e.target as HTMLSelectElement;
                const keyId = target.value || "";
                const key = availableMetadataKeys.find((k) => k.id === keyId);
                if (key) {
                  const operators = getOperatorsForDataType(key.data_type);
                  updateMetadataFilter(index, {
                    ...filter,
                    keyId: key.id,
                    keyName: key.name,
                    displayName: key.display_name,
                    dataType: key.data_type,
                    operator: operators[0] as "equals" | "contains" | "greater_than" | "less_than" | "not_equals",
                  });
                }
              }}
              className="metadata-key-select"
            />

            {#if filter.keyId}
              <Select
                options={getOperatorsForDataType(filter.dataType).map(op => ({
                  value: op,
                  label: getOperatorLabel(op)
                }))}
                value={filter.operator}
                on:change={(e) => {
                  const target = e.target as HTMLSelectElement;
                  updateMetadataFilter(index, {
                    ...filter,
                    operator: target.value as any,
                  });
                }}
                className="metadata-operator-select"
              />

              {#if filter.dataType === "boolean"}
                <Select
                  options={[
                    { value: "", label: $t("common.search.selectValue") },
                    { value: "true", label: $t("common.search.yes") },
                    { value: "false", label: $t("common.search.no") }
                  ]}
                  value={filter.value}
                  on:change={(e) => {
                    const target = e.target as HTMLSelectElement;
                    updateMetadataFilter(index, {
                      ...filter,
                      value: target.value || "",
                    });
                  }}
                  className="metadata-value-input"
                />
              {:else if filter.dataType === "date"}
                <DateInput
                  id="metadata-filter-date-{index}"
                  value={filter.value}
                  oninput={(e) => {
                    updateMetadataFilter(index, {
                      ...filter,
                      value: (e.currentTarget as any)?.value || "",
                    });
                  }}
                  class="metadata-value-input"
                />
              {:else if filter.dataType === "number"}
                <NumberInput
                  id="metadata-filter-number-{index}"
                  value={filter.value ? Number(filter.value) : undefined}
                  placeholder={$t("common.search.enterNumber")}
                  oninput={(e) => {
                    updateMetadataFilter(index, {
                      ...filter,
                      value: (e.currentTarget as any)?.value || "",
                    });
                  }}
                  class="metadata-value-input"
                />
              {:else}
                <TextInput
                  id="metadata-filter-{index}"
                  value={filter.value}
                  placeholder={$t("common.search.enterValue")}
                  oninput={(e) => {
                    updateMetadataFilter(index, {
                      ...filter,
                      value: (e.currentTarget as any)?.value || "",
                    });
                  }}
                  class="metadata-value-input"
                />
              {/if}
            {/if}

            <button
              onclick={() => removeMetadataFilter(index)}
              class="remove-filter-btn"
              title={$t("common.search.removeFilter")}
            >
              <X size={16} />
            </button>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <!-- 検索結果のファイル種別フィルター -->
  <FileCategoryFilters {selectedCategory} {categoryCounts} onSelectCategory={onSelectCategory as any} />

  <!-- コンテキストアクションバー -->
  {#if hasSelection}
    <div class="selection-action-bar">
      <div class="selection-info">
        <span class="selection-count">{selectedCount}件のファイルを選択中</span>
        <Button
          id="clear-selection-btn"
          variant="neutral"
          size="small"
          text="選択を解除"
          onclick={handleClearSelection}
        />
      </div>
      <div class="selection-actions">
        <Button
          id="rename-selected-btn"
          variant="success"
          size="small"
          text="リネーム"
          onclick={handleRenameSelected}
          {...({ title: selectedCount === 1
            ? "選択したファイルをリネーム"
            : `${selectedCount}件のファイルをバッチリネーム` } as any)}
        />
        <Button
          id="delete-selected-btn"
          variant="danger"
          size="small"
          text="削除"
          onclick={handleDeleteSelected}
        />
      </div>
    </div>
  {:else}
    <div class="bulk-actions">
      <Button
        id="select-all-btn"
        variant="neutral"
        size="small"
        text="すべて選択"
        onclick={handleSelectAll}
      />
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

  <div class="pagination-controls">
    <Pagination
      {currentPage}
      {totalPages}
      {onGoToPage}
      {onGoToPreviousPage}
      {onGoToNextPage}
      {onGoToFirstPage}
      {onGoToLastPage}
    />
    <div class="sort-section">
      <FileSortControl
        sortField={sortOptions.field}
        sortOrder={sortOptions.order}
        onSortChange={onSortChange as any}
      />
    </div>
  </div>

  <FileList
    filesWithTags={filteredResults.map((result) => ({
      file: result.file,
      tags: result.tags,
    }))}
    {currentPage}
    {totalPages}
    viewMode={$viewMode}
    emptyMessage={searchResults.length === 0 &&
    (searchQuery || metadataSearchFilters.some((f) => f.keyId && f.value))
      ? $t("common.search.noSearchResults")
      : ""}
    showEmptyState={false}
    {onSelectFile}
    onGoToPage={onGoToPage as any}
    onGoToPreviousPage={onGoToPreviousPage as any}
    onGoToNextPage={onGoToNextPage as any}
    onGoToFirstPage={onGoToFirstPage as any}
    onGoToLastPage={onGoToLastPage as any}
  />
</div>

<style>
  .search-view {
    max-width: 800px;
    margin: 0 auto;
  }

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

  .search-header .header-top,
  .search-header .header-bottom {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1rem;
  }

  .header-left h2 {
    margin: 0 0 0.5rem 0;
  }

  .search-stats {
    text-align: right;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 0.25rem;
  }

  /* 表示モード切り替えボタン */
  .view-mode-switcher {
    display: flex;
    gap: 0;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    overflow: hidden;
    background: #ffffff;
  }

  .view-mode-btn {
    padding: 8px 12px;
    background: #ffffff;
    border: none;
    cursor: pointer;
    color: #6b7280;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
  }

  .view-mode-btn:not(:last-child)::after {
    content: "";
    position: absolute;
    right: 0;
    top: 20%;
    bottom: 20%;
    width: 1px;
    background: #d1d5db;
  }

  .view-mode-btn:hover {
    background: #f3f4f6;
    color: #374151;
  }

  .view-mode-btn.active {
    background: #3b82f6;
    color: #ffffff;
  }

  .view-mode-btn.active:hover {
    background: #2563eb;
  }

  .view-mode-btn.active:not(:last-child)::after {
    background: transparent;
  }

  .search-controls {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .pagination-controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin: 1rem 0;
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

  .selected-tags,
  .top-tags,
  .tag-search {
    margin-bottom: 1rem;
  }

  .selected-tags h4,
  .top-tags h4,
  .tag-search h4 {
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
  /* コンテキストアクションバーのスタイル */
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


  .selection-actions {
    display: flex;
    gap: 8px;
  }


  .bulk-actions {
    display: flex;
    justify-content: flex-end;
    margin: 1rem 0;
  }
</style>

<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { open, confirm } from "@tauri-apps/plugin-dialog";
  import { FileText, Search, Tag, Settings } from "lucide-svelte";

  // コンポーネントのインポート
  import LoadingScreen from "../lib/components/LoadingScreen.svelte";
  import SimpleLoadingScreen from "../lib/components/SimpleLoadingScreen.svelte";
  import Sidebar from "../lib/components/Sidebar.svelte";
  import FilesView from "../lib/components/FilesView.svelte";
  import SearchView from "../lib/components/SearchView.svelte";
  import TagsView from "../lib/components/TagsView.svelte";
  import FileDetailModal from "../lib/components/FileDetailModal.svelte";
  import FileRenameModal from "../lib/components/FileRenameModal.svelte";
  import CustomMetadataKeyManager from "../lib/components/CustomMetadataKeyManager.svelte";
  import SettingsModal from "../lib/components/SettingsModal.svelte";

  // ViewModel のインポート
  import { AppViewModel, type ActiveTab } from "../lib/viewmodels/AppViewModel";
  import type { File } from "../lib/types";
  import "../lib/App.css";
  import { errorStore } from "../lib/stores/error";
  import { isLoading, loadingMessage } from "../lib/stores/common";
  import { t } from "$lib/i18n";

  // AppViewModel インスタンス
  const appViewModel = new AppViewModel();

  // ViewModelからのストア取得
  const {
    activeTab,
    isAppLoading,
    loadingSteps,
    loadingProgress,
    directoryViewModel,
    fileViewModel,
    searchViewModel,
    tagViewModel,
  } = appViewModel;

  const { directories, selectedDirectoryId } = directoryViewModel;
  const {
    files,
    filesWithTags,
    paginatedFilesWithTags,
    selectedFile,
    selectedCategory,
    currentPage,
    totalFiles,
    isDeleting,
    categoryCounts,
    filteredFiles,
    totalPages,
    paginatedFiles,
    itemsPerPage,
    sortOptions,
  } = fileViewModel;
  const {
    searchQuery,
    selectedTags,
    metadataSearchFilters,
    metadataLogic,
    searchResults,
    totalSearchResults,
    selectedCategory: searchSelectedCategory,
    currentPage: searchCurrentPage,
    searchCategoryCounts,
    totalSearchCategoryCounts,
    filteredSearchResults,
    searchTotalPages,
    paginatedSearchResults,
    itemsPerPage: searchItemsPerPage,
    sortOptions: searchSortOptions,
  } = searchViewModel;
  const { tags, topTags, tagSearchResults, customMetadataKeys } = tagViewModel;

  // 設定モーダルの状態管理
  let isSettingsModalOpen = false;
  
  // リネームモーダルの状態管理
  let isRenameModalOpen = false;
  let renameTargetFile: File | null = null;

  let fileSystemChangeListenerPromise: Promise<void> | null = null;
  let unlisten: (() => void) | null = null;

  const openSettingsModal = () => {
    isSettingsModalOpen = true;
  };

  const closeSettingsModal = () => {
    isSettingsModalOpen = false;
  };
  
  // リネームモーダルの関数
  const openRenameModal = (file: File) => {
    renameTargetFile = file;
    isRenameModalOpen = true;
  };

  const closeRenameModal = () => {
    isRenameModalOpen = false;
    renameTargetFile = null;
  };

  onMount(() => {
    // ViewModelが自動的に初期化するため、特別な処理は不要
    if (fileSystemChangeListenerPromise) {
      // 既にリスナーが登録されている場合は何もしない
      return;
    }
    fileSystemChangeListenerPromise = Promise.resolve().then(async () => {
      // ファイルシステム変更のリスナーを追加
      const { listen } = await import("@tauri-apps/api/event");
      unlisten = await listen("file_system_change", (event) => {
        // ファイル一覧を再読み込み
        fileViewModel.loadFiles();
      });
    });
  });

  onDestroy(() => {
    unlisten?.(); // コンポーネントが破棄される際にリスナーを解除
    appViewModel.dispose();
  });

  const reloadData = async () => {
    await appViewModel.reloadAllData();
  };

  const addDirectory = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: $t("common.dialog.selectDirectory"),
      });

      if (selected && typeof selected === "string") {
        const name = selected.split("/").pop() || selected;
        await directoryViewModel.addNewDirectoryWithLoading(selected, name, tagViewModel);
      }
    } catch (error) {
      console.error("Failed to add directory:", error);
      const fallbackPath = prompt($t("common.dialog.fallbackDirectoryInput"));
      if (fallbackPath && fallbackPath.trim()) {
        const name = fallbackPath.split("/").pop() || fallbackPath;
        await directoryViewModel.addNewDirectoryWithLoading(fallbackPath.trim(), name, tagViewModel);
      }
    }
  };

  const selectDirectory = (directoryId: string | "all") => {
    directoryViewModel.selectDirectory(directoryId);
    searchViewModel.setSelectedDirectoryId(directoryId);
  };

  const rescanDirectory = async (directoryId: string) => {
    const success = await appViewModel.rescanDirectory(directoryId);
    if (!success) {
      errorStore.showError($t("common.error.directoryRescanFailed"));
    }
  };

  const rescanAll = async () => {
    const success = await appViewModel.rescanAllDirectories();
    if (!success) {
      errorStore.showError($t("common.error.allDirectoriesRescanFailed"));
    }
  };

  const removeDirectory = async (
    directoryId: string,
    directoryName: string,
  ) => {
    const confirmed = await confirm(
      $t("common.dialog.confirmRemoveDirectory", { name: directoryName }),
      { title: $t("common.dialog.confirm"), kind: "warning" } as any,
    );
    if (confirmed) {
      const result = await directoryViewModel.removeExistingDirectory(
        directoryId,
        tagViewModel,
      );
      if (result) {
        await fileViewModel.loadFiles();

        // タグが削除された場合の通知
        if (result.deleted_tags_count > 0) {
          console.log(
            `ディレクトリ削除により${result.deleted_tags_count}個のタグが削除されました`,
          );
        }
      } else {
        errorStore.showError($t("common.error.directoryRemoveFailed"));
      }
    }
  };

  const createTag = async () => {
    const name = prompt($t("common.dialog.enterTagName"));
    if (name) {
      await tagViewModel.createNewTag(name, "#3B82F6");
    }
  };

  const searchFiles = async () => {
    await searchViewModel.performSearch();
  };

  const selectFile = (file: File) => {
    fileViewModel.selectFile(file);
  };

  const closeFileDetails = () => {
    fileViewModel.closeFileDetails();
  };

  const openFile = async (filePath: string) => {
    const success = await fileViewModel.openSelectedFile(filePath);
    if (!success) {
      errorStore.showError($t("common.error.fileOpenFailed"));
    }
  };

  const revealInFinder = async (filePath: string) => {
    const success = await fileViewModel.revealFileInFinder(filePath);
    if (!success) {
      errorStore.showError($t("common.error.fileRevealFailed"));
    }
  };

  const deleteFile = async (filePath: string, fileName: string) => {
    const confirmed = await confirm(
      $t("common.dialog.confirmDeleteFile", { name: fileName }),
      {
        title: $t("common.dialog.confirm"),
        kind: "warning",
      } as any,
    );
    if (confirmed) {
      const success = await fileViewModel.deleteSelectedFile(filePath);
      if (!success) {
        errorStore.showError($t("common.error.fileDeleteFailed"));
      }
    }
  };

  // カスタムメタデータキーが更新された時の処理
  const handleCustomMetadataKeysUpdated = async () => {
    await tagViewModel.loadCustomMetadataKeys();
  };

  // タグ追加ハンドラー
  const handleTagAdd = async (tagId: string) => {
    let currentTags: string[] = [];
    const unsubscribe = selectedTags.subscribe((tags) => (currentTags = tags));
    unsubscribe();

    if (!currentTags.includes(tagId)) {
      searchViewModel.setSelectedTags([...currentTags, tagId]);
      await searchViewModel.performSearch(); // 検索を再実行
    }
  };

  // タグ削除ハンドラー
  const handleTagRemove = async (tagId: string) => {
    let currentTags: string[] = [];
    const unsubscribe = selectedTags.subscribe((tags) => (currentTags = tags));
    unsubscribe();

    const newTags = currentTags.filter((id) => id !== tagId);
    searchViewModel.setSelectedTags(newTags);
    await searchViewModel.performSearch(); // 検索を再実行
  };

  // 設定が変更された時の処理
  const handleSettingsChanged = async () => {
    await fileViewModel.updateItemsPerPage();
    await searchViewModel.updateItemsPerPage();
    await fileViewModel.loadFiles();
    await searchViewModel.performSearch();
  };

  // タグが更新された時の処理
  const handleTagsUpdated = async () => {
    await fileViewModel.loadFiles(); // ファイル一覧を再読み込み
    await tagViewModel.loadTags(); // タグ一覧を再読み込み
    await tagViewModel.loadTopTags(); // トップタグを再読み込み
    await searchViewModel.refreshSearchResults(); // 検索結果も再読み込み
  };
</script>

<div class="app">
  <LoadingScreen
    isVisible={$isAppLoading}
    progress={$loadingProgress}
    steps={$loadingSteps}
  />

  {#if $isLoading}
    <SimpleLoadingScreen message={$loadingMessage} />
  {/if}

  <div class="app-content {$isAppLoading || $isLoading ? 'loading' : ''}">
    <Sidebar
      directories={$directories}
      tags={$tags}
      selectedDirectoryId={$selectedDirectoryId}
      onAddDirectory={addDirectory}
      onSelectDirectory={selectDirectory}
      onRescanDirectory={rescanDirectory}
      onRemoveDirectory={removeDirectory}
      onCreateTag={createTag}
      onRescanAll={rescanAll}
      disabled={$isLoading}
      {appViewModel}
    />

    <div class="main-content">
      <div class="tabs">
        <div class="tab-group">
          <button
            class="tab {$activeTab === 'files' ? 'active' : ''}"
            onclick={() => appViewModel.setActiveTab("files")}
          >
            <FileText size={16} />
            {$t("common.tabs.files")}
          </button>
          <button
            class="tab {$activeTab === 'search' ? 'active' : ''}"
            onclick={() => appViewModel.setActiveTab("search")}
          >
            <Search size={16} />
            {$t("common.tabs.search")}
          </button>
          <button
            class="tab {$activeTab === 'tags' ? 'active' : ''}"
            onclick={() => appViewModel.setActiveTab("tags")}
          >
            <Tag size={16} />
            {$t("common.tabs.tags")}
          </button>
          <button
            class="tab {$activeTab === 'metadata' ? 'active' : ''}"
            onclick={() => appViewModel.setActiveTab("metadata")}
          >
            🏷️ {$t("common.tabs.metadata")}
          </button>
        </div>
        <button
          class="settings-button"
          onclick={openSettingsModal}
          title={$t("common.buttons.settings")}
        >
          <Settings size={16} />
        </button>
      </div>

      <div class="content-area">
        {#if $activeTab === "files"}
          <FilesView
            files={$paginatedFiles}
            filesWithTags={$paginatedFilesWithTags}
            selectedCategory={$selectedCategory}
            categoryCounts={$categoryCounts}
            currentPage={$currentPage}
            totalFiles={$totalFiles}
            totalPages={$totalPages}
            itemsPerPage={$itemsPerPage}
            selectedDirectoryId={$selectedDirectoryId}
            sortOptions={$sortOptions}
            onSelectFile={selectFile}
            onSelectCategory={async (category) =>
              await fileViewModel.selectCategory(category)}
            onGoToPage={async (page) => await fileViewModel.goToPage(page)}
            onGoToPreviousPage={async () =>
              await fileViewModel.goToPreviousPage()}
            onGoToNextPage={async () => await fileViewModel.goToNextPage()}
            onGoToFirstPage={async () => await fileViewModel.goToFirstPage()}
            onGoToLastPage={async () => await fileViewModel.goToLastPage()}
            onSortChange={async (options) =>
              await fileViewModel.setSortOptions(options)}
            on:open-rename-modal={(event) => openRenameModal(event.detail)}
          />
        {/if}

        {#if $activeTab === "search"}
          <SearchView
            bind:searchQuery={$searchQuery}
            searchResults={$searchResults}
            filteredResults={$paginatedSearchResults}
            allFilteredResults={$filteredSearchResults}
            totalSearchResults={$totalSearchResults}
            selectedCategory={$searchSelectedCategory}
            categoryCounts={$totalSearchCategoryCounts}
            currentPage={$searchCurrentPage}
            totalPages={$searchTotalPages}
            itemsPerPage={$searchItemsPerPage}
            selectedTags={$selectedTags}
            allTags={$tags}
            topTags={$topTags}
            tagSearchResults={$tagSearchResults}
            bind:metadataSearchFilters={$metadataSearchFilters}
            availableMetadataKeys={$customMetadataKeys}
            metadataLogic={$metadataLogic}
            sortOptions={$searchSortOptions}
            selectedDirectoryId={$selectedDirectoryId}
            onSearchQueryChange={(query) =>
              searchViewModel.setSearchQuery(query)}
            onSearch={searchFiles}
            onSelectFile={selectFile}
            onSelectCategory={(category) =>
              searchViewModel.selectCategory(category)}
            onGoToPage={(page) => searchViewModel.goToPage(page)}
            onGoToPreviousPage={() => searchViewModel.goToPreviousPage()}
            onGoToNextPage={() => searchViewModel.goToNextPage()}
            onGoToFirstPage={() => searchViewModel.goToFirstPage()}
            onGoToLastPage={() => searchViewModel.goToLastPage()}
            onTagAdd={(tagId) => handleTagAdd(tagId)}
            onTagRemove={(tagId) => handleTagRemove(tagId)}
            onTagSearch={(query) => tagViewModel.searchTags(query)}
            onMetadataLogicChange={(logic) =>
              searchViewModel.setMetadataLogic(logic)}
            onSortChange={(options) => searchViewModel.setSortOptions(options)}
            onTagsUpdated={handleTagsUpdated}
          />
        {/if}

        {#if $activeTab === "tags"}
          <TagsView tags={$tags} />
        {/if}

        {#if $activeTab === "metadata"}
          <CustomMetadataKeyManager
            keys={$customMetadataKeys}
            onKeysUpdated={handleCustomMetadataKeysUpdated}
          />
        {/if}
      </div>
    </div>
  </div>

  <FileDetailModal
    file={$selectedFile}
    isDeleting={$isDeleting}
    customMetadataKeys={$customMetadataKeys}
    onOpenFile={openFile}
    onRevealInFinder={revealInFinder}
    onDeleteFile={deleteFile}
    onClose={closeFileDetails}
    onTagsUpdated={handleTagsUpdated}
    onFileRenamed={handleTagsUpdated}
    onOpenRenameModal={openRenameModal}
  />
  
  <FileRenameModal
    file={renameTargetFile}
    onClose={closeRenameModal}
    onFileRenamed={handleTagsUpdated}
  />

  <!-- 設定モーダル -->
  <SettingsModal
    isOpen={isSettingsModalOpen}
    onClose={closeSettingsModal}
    onSettingsChanged={handleSettingsChanged}
  />
</div>

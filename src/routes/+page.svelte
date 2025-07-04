<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { open, confirm } from "@tauri-apps/plugin-dialog";
  import { FileText, Search, Tag } from "lucide-svelte";

  // コンポーネントのインポート
  import LoadingScreen from "../lib/components/LoadingScreen.svelte";
  import Sidebar from "../lib/components/Sidebar.svelte";
  import FilesView from "../lib/components/FilesView.svelte";
  import SearchView from "../lib/components/SearchView.svelte";
  import TagsView from "../lib/components/TagsView.svelte";
  import FileDetailModal from "../lib/components/FileDetailModal.svelte";
  import CustomMetadataKeyManager from "../lib/components/CustomMetadataKeyManager.svelte";

  // ViewModel のインポート
  import { AppViewModel, type ActiveTab } from "../lib/viewmodels/AppViewModel.js";
  import type { File } from "../lib/types.js";
  import "../lib/App.css";

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
    tagViewModel
  } = appViewModel;

  const { directories, selectedDirectoryId } = directoryViewModel;
  const { 
    files, 
    selectedFile, 
    selectedCategory, 
    currentPage, 
    isDeleting,
    categoryCounts,
    filteredFiles,
    totalPages,
    paginatedFiles 
  } = fileViewModel;
  const { 
    searchQuery, 
    selectedTags, 
    metadataSearchFilters, 
    searchResults,
    selectedCategory: searchSelectedCategory,
    currentPage: searchCurrentPage,
    searchCategoryCounts,
    filteredSearchResults,
    searchTotalPages,
    paginatedSearchResults
  } = searchViewModel;
  const { tags, customMetadataKeys } = tagViewModel;

  onMount(async () => {
    // ViewModelが自動的に初期化するため、特別な処理は不要
    
    // ファイルシステム変更のリスナーを追加
    const { listen } = await import('@tauri-apps/api/event');
    const unlisten = await listen('file_system_change', (event) => {
      console.log('ファイルシステム変更イベント:', event.payload);
      // ファイル一覧を再読み込み
      fileViewModel.loadFiles();
    });
    
    return () => {
      unlisten();
    };
  });

  onDestroy(() => {
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
        title: "追加するディレクトリを選択",
      });

      if (selected && typeof selected === "string") {
        const name = selected.split("/").pop() || selected;
        await directoryViewModel.addNewDirectory(selected, name);
        await fileViewModel.loadFiles(); // ファイル一覧も更新
      }
    } catch (error) {
      console.error("Failed to add directory:", error);
      const fallbackPath = prompt(
        "ディレクトリ選択に失敗しました。パスを直接入力してください:",
      );
      if (fallbackPath && fallbackPath.trim()) {
        const name = fallbackPath.split("/").pop() || fallbackPath;
        await directoryViewModel.addNewDirectory(fallbackPath.trim(), name);
        await fileViewModel.loadFiles();
      }
    }
  };

  const selectDirectory = (directoryId: string | "all") => {
    directoryViewModel.selectDirectory(directoryId);
    searchViewModel.setSelectedDirectoryId(directoryId);
  };

  const rescanDirectory = async (directoryId: string) => {
    const success = await directoryViewModel.rescanExistingDirectory(directoryId);
    if (success) {
      await fileViewModel.loadFiles();
    } else {
      alert("ディレクトリの再スキャンに失敗しました。");
    }
  };

  const removeDirectory = async (
    directoryId: string,
    directoryName: string,
  ) => {
    const confirmed = await confirm(
      `「${directoryName}」を登録から削除しますか？\nファイルは削除されません。`,
      { title: "確認", kind: "warning" },
    );
    if (confirmed) {
      const success = await directoryViewModel.removeExistingDirectory(directoryId);
      if (success) {
        await fileViewModel.loadFiles();
      } else {
        alert("ディレクトリの削除に失敗しました。");
      }
    }
  };

  const createTag = async () => {
    const name = prompt("Enter tag name:");
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
      alert(`ファイルを開けませんでした`);
    }
  };

  const revealInFinder = async (filePath: string) => {
    const success = await fileViewModel.revealFileInFinder(filePath);
    if (!success) {
      alert(`Finderで表示できませんでした`);
    }
  };

  const deleteFile = async (filePath: string, fileName: string) => {
    const confirmed = await confirm(`「${fileName}」をゴミ箱に移動しますか？`, {
      title: "確認",
      kind: "warning",
    });
    if (confirmed) {
      const success = await fileViewModel.deleteSelectedFile(filePath);
      if (!success) {
        alert(`ファイルをゴミ箱に移動できませんでした`);
      }
    }
  };





  // カスタムメタデータキーが更新された時の処理
  const handleCustomMetadataKeysUpdated = async () => {
    await tagViewModel.loadCustomMetadataKeys();
  };
</script>

<div class="app">
  <LoadingScreen
    isVisible={$isAppLoading}
    progress={$loadingProgress}
    steps={$loadingSteps}
  />

  <header class="app-header">
    <h1>Clerica</h1>
    <p>Mac向けファイル整理・検索ツール</p>
  </header>

  <div class="app-content {$isAppLoading ? 'loading' : ''}">
    <Sidebar
      directories={$directories}
      tags={$tags}
      selectedDirectoryId={$selectedDirectoryId}
      onAddDirectory={addDirectory}
      onSelectDirectory={selectDirectory}
      onRescanDirectory={rescanDirectory}
      onRemoveDirectory={removeDirectory}
      onCreateTag={createTag}
    />

    <div class="main-content">
      <div class="tabs">
        <button
          class="tab {$activeTab === 'files' ? 'active' : ''}"
          onclick={() => appViewModel.setActiveTab('files')}
        >
          <FileText size={16} />
          ファイル
        </button>
        <button
          class="tab {$activeTab === 'search' ? 'active' : ''}"
          onclick={() => appViewModel.setActiveTab('search')}
        >
          <Search size={16} />
          検索
        </button>
        <button
          class="tab {$activeTab === 'tags' ? 'active' : ''}"
          onclick={() => appViewModel.setActiveTab('tags')}
        >
          <Tag size={16} />
          タグ管理
        </button>
        <button
          class="tab {$activeTab === 'metadata' ? 'active' : ''}"
          onclick={() => appViewModel.setActiveTab('metadata')}
        >
          🏷️ メタデータ
        </button>
      </div>

      <div class="content-area">
        {#if $activeTab === "files"}
          <FilesView
            files={$paginatedFiles}
            selectedCategory={$selectedCategory}
            categoryCounts={$categoryCounts}
            currentPage={$currentPage}
            totalFiles={$filteredFiles.length}
            totalPages={$totalPages}
            selectedDirectoryId={$selectedDirectoryId}
            onSelectFile={selectFile}
            onSelectCategory={(category) => fileViewModel.selectCategory(category)}
            onGoToPage={(page) => fileViewModel.goToPage(page)}
            onGoToPreviousPage={() => fileViewModel.goToPreviousPage()}
            onGoToNextPage={() => fileViewModel.goToNextPage()}
            onGoToFirstPage={() => fileViewModel.goToFirstPage()}
            onGoToLastPage={() => fileViewModel.goToLastPage($totalPages)}
          />
        {/if}

        {#if $activeTab === "search"}
          <SearchView
            bind:searchQuery={$searchQuery}
            searchResults={$searchResults}
            filteredResults={$paginatedSearchResults}
            selectedCategory={$searchSelectedCategory}
            categoryCounts={$searchCategoryCounts}
            currentPage={$searchCurrentPage}
            totalPages={$searchTotalPages}
            bind:metadataSearchFilters={$metadataSearchFilters}
            availableMetadataKeys={$customMetadataKeys}
            onSearchQueryChange={(query) => searchViewModel.setSearchQuery(query)}
            onSearch={searchFiles}
            onSelectFile={selectFile}
            onSelectCategory={(category) => searchViewModel.selectCategory(category)}
            onGoToPage={(page) => searchViewModel.goToPage(page)}
            onGoToPreviousPage={() => searchViewModel.goToPreviousPage()}
            onGoToNextPage={() => searchViewModel.goToNextPage()}
            onGoToFirstPage={() => searchViewModel.goToFirstPage()}
            onGoToLastPage={() => searchViewModel.goToLastPage($searchTotalPages)}
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
  />
</div>

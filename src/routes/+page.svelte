<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open, confirm } from "@tauri-apps/plugin-dialog";
  import { FileText, Search, Tag } from "lucide-svelte";
  
  // コンポーネントのインポート
  import LoadingScreen from "../lib/components/LoadingScreen.svelte";
  import Sidebar from "../lib/components/Sidebar.svelte";
  import FilesView from "../lib/components/FilesView.svelte";
  import SearchView from "../lib/components/SearchView.svelte";
  import TagsView from "../lib/components/TagsView.svelte";
  import FileDetailModal from "../lib/components/FileDetailModal.svelte";
  
  // 型とユーティリティのインポート
  import type { Directory, File, Tag as TagType, SearchResult, FileCategory, LoadingSteps } from "../lib/types.js";
  import { getFileCategory } from "../lib/utils.js";
  import "../lib/App.css";

  // 状態管理
  let directories: Directory[] = $state([]);
  let files: File[] = $state([]);
  let tags: TagType[] = $state([]);
  let searchQuery = $state("");
  let selectedTags: string[] = $state([]);
  let searchResults: SearchResult[] = $state([]);
  let activeTab: "files" | "search" | "tags" = $state("files");
  let selectedFile: File | null = $state(null);
  
  // ディレクトリフィルタリング状態
  let selectedDirectoryId: string | "all" = $state("all");
  
  // 削除処理中の状態管理
  let isDeleting = $state(false);
  
  // 読み込み状態管理
  let isLoading = $state(true);
  let loadingSteps: LoadingSteps = $state({
    directories: false,
    tags: false,
    files: false
  });
  let loadingProgress = $state(0);
  
  // ページネーション状態
  let currentPage = $state(1);
  let itemsPerPage = 25;
  let totalFiles = $state(0);
  let totalPages = $state(0);
  let paginatedFiles: File[] = $state([]);
  
  // 検索結果のページネーション状態
  let searchCurrentPage = $state(1);
  let searchTotalPages = $state(0);
  let paginatedSearchResults: SearchResult[] = $state([]);
  
  // 検索結果のフィルタリング状態
  let searchSelectedCategory: FileCategory = $state("all");
  let filteredSearchResults: SearchResult[] = $state([]);
  let searchCategoryCounts: Record<FileCategory, number> = $state({
    all: 0,
    image: 0,
    audio: 0,
    video: 0,
    document: 0,
    archive: 0,
    other: 0
  });
  
  // ファイル種別フィルタリング状態
  let selectedCategory: FileCategory = $state("all");
  let filteredFiles: File[] = $state([]);
  let categoryCounts: Record<FileCategory, number> = $state({
    all: 0,
    image: 0,
    audio: 0,
    video: 0,
    document: 0,
    archive: 0,
    other: 0
  });

  onMount(() => {
    loadData();
  });

  const loadData = async () => {
    try {
      isLoading = true;
      loadingProgress = 0;
      loadingSteps = { directories: false, tags: false, files: false };
      
      // ディレクトリの読み込み
      const directoriesData = await invoke("get_directories");
      directories = directoriesData as Directory[];
      loadingSteps.directories = true;
      loadingProgress = 33;
      
      // タグの読み込み
      const tagsData = await invoke("get_tags");
      tags = tagsData as TagType[];
      loadingSteps.tags = true;
      loadingProgress = 66;
      
      // ファイルの読み込み
      const filesData = await invoke("get_files");
      files = filesData as File[];
      loadingSteps.files = true;
      loadingProgress = 100;
      
      // カテゴリ別ファイル数を計算
      updateCategoryCounts();
      
      // フィルタリングを適用
      filterFilesByCategory();
      
      // 読み込み完了
      setTimeout(() => {
        isLoading = false;
      }, 500);
    } catch (error) {
      console.error("Failed to load data:", error);
      isLoading = false;
    }
  };

  const addDirectory = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "追加するディレクトリを選択"
      });
      
      if (selected && typeof selected === 'string') {
        const name = selected.split('/').pop() || selected;
        await invoke('add_directory', { path: selected, name });
        await loadData();
      }
    } catch (error) {
      console.error("Failed to add directory:", error);
      const fallbackPath = prompt("ディレクトリ選択に失敗しました。パスを直接入力してください:");
      if (fallbackPath && fallbackPath.trim()) {
        const name = fallbackPath.split('/').pop() || fallbackPath;
        await invoke('add_directory', { path: fallbackPath.trim(), name });
        await loadData();
      }
    }
  };

  const selectDirectory = (directoryId: string | "all") => {
    selectedDirectoryId = directoryId;
    updateCategoryCounts();
    filterFilesByCategory();
  };

  const rescanDirectory = async (directoryId: string) => {
    try {
      await invoke("rescan_directory", { directoryId });
      await loadData();
    } catch (error) {
      console.error("Failed to rescan directory:", error);
      alert("ディレクトリの再スキャンに失敗しました。");
    }
  };

  const removeDirectory = async (directoryId: string, directoryName: string) => {
    const confirmed = await confirm(`「${directoryName}」を登録から削除しますか？\nファイルは削除されません。`, { title: '確認', kind: 'warning' });
    if (confirmed) {
      try {
        await invoke('remove_directory', { id: directoryId });
        await loadData();
      } catch (error) {
        console.error("Failed to remove directory:", error);
        alert("ディレクトリの削除に失敗しました。");
      }
    }
  };

  const createTag = async () => {
    try {
      const name = prompt("Enter tag name:");
      if (name) {
        await invoke('create_tag', { name, color: '#3B82F6' });
        await loadData();
      }
    } catch (error) {
      console.error("Failed to create tag:", error);
    }
  };

  const searchFiles = async () => {
    try {
      const results = await invoke('search_files', { 
        query: searchQuery, 
        tag_ids: selectedTags 
      });
      searchResults = results as SearchResult[];
      updateSearchCategoryCounts();
      filterSearchResultsByCategory();
    } catch (error) {
      console.error("Failed to search files:", error);
    }
  };

  const selectFile = (file: File) => {
    selectedFile = file;
  };

  const closeFileDetails = () => {
    selectedFile = null;
  };

  const openFile = async (filePath: string) => {
    try {
      await invoke("open_file", { filePath });
      await loadData();
    } catch (error) {
      console.error("Failed to open file:", error);
      alert(`ファイルを開けませんでした: ${error}`);
    }
  };

  const revealInFinder = async (filePath: string) => {
    try {
      await invoke("reveal_in_finder", { filePath });
    } catch (error) {
      console.error("Failed to reveal in Finder:", error);
      alert(`Finderで表示できませんでした: ${error}`);
    }
  };

  const deleteFile = async (filePath: string, fileName: string) => {
    const confirmed = await confirm(`「${fileName}」をゴミ箱に移動しますか？`, { title: '確認', kind: 'warning' });
    if (confirmed) {
      isDeleting = true;
      try {
        await invoke("delete_file", { filePath });
        await loadData();
        closeFileDetails();
      } catch (error) {
        console.error("Failed to delete file:", error);
        alert(`ファイルをゴミ箱に移動できませんでした: ${error}`);
      } finally {
        isDeleting = false;
      }
    }
  };

  // ページネーション関数
  const updatePagination = () => {
    totalFiles = filteredFiles.length;
    totalPages = Math.ceil(totalFiles / itemsPerPage);
    
    if (currentPage > totalPages && totalPages > 0) {
      currentPage = 1;
    }
    
    const startIndex = (currentPage - 1) * itemsPerPage;
    const endIndex = startIndex + itemsPerPage;
    paginatedFiles = filteredFiles.slice(startIndex, endIndex);
  };

  const goToPage = (page: number) => {
    if (page >= 1 && page <= totalPages) {
      currentPage = page;
      updatePagination();
    }
  };

  const goToPreviousPage = () => {
    if (currentPage > 1) {
      currentPage--;
      updatePagination();
    }
  };

  const goToNextPage = () => {
    if (currentPage < totalPages) {
      currentPage++;
      updatePagination();
    }
  };

  const goToFirstPage = () => {
    currentPage = 1;
    updatePagination();
  };

  const goToLastPage = () => {
    currentPage = totalPages;
    updatePagination();
  };

  // 検索結果のページネーション関数
  const updateSearchPagination = () => {
    searchTotalPages = Math.ceil(filteredSearchResults.length / itemsPerPage);
    
    if (searchCurrentPage > searchTotalPages && searchTotalPages > 0) {
      searchCurrentPage = 1;
    }
    
    const startIndex = (searchCurrentPage - 1) * itemsPerPage;
    const endIndex = startIndex + itemsPerPage;
    paginatedSearchResults = filteredSearchResults.slice(startIndex, endIndex);
  };

  const goToSearchPage = (page: number) => {
    if (page >= 1 && page <= searchTotalPages) {
      searchCurrentPage = page;
      updateSearchPagination();
    }
  };

  const goToSearchPreviousPage = () => {
    if (searchCurrentPage > 1) {
      searchCurrentPage--;
      updateSearchPagination();
    }
  };

  const goToSearchNextPage = () => {
    if (searchCurrentPage < searchTotalPages) {
      searchCurrentPage++;
      updateSearchPagination();
    }
  };

  const goToSearchFirstPage = () => {
    searchCurrentPage = 1;
    updateSearchPagination();
  };

  const goToSearchLastPage = () => {
    searchCurrentPage = searchTotalPages;
    updateSearchPagination();
  };

  // カテゴリ別ファイル数を計算
  const updateCategoryCounts = () => {
    const directoryFilteredFiles = filterFilesByDirectory();
    const counts: Record<FileCategory, number> = {
      all: directoryFilteredFiles.length,
      image: 0,
      audio: 0,
      video: 0,
      document: 0,
      archive: 0,
      other: 0
    };

    directoryFilteredFiles.forEach(file => {
      const category = getFileCategory(file);
      counts[category]++;
    });

    categoryCounts = counts;
  };

  // ディレクトリでファイルをフィルタリング
  const filterFilesByDirectory = () => {
    let directoryFilteredFiles: File[];
    if (selectedDirectoryId === "all") {
      directoryFilteredFiles = [...files];
    } else {
      directoryFilteredFiles = files.filter(file => file.directory_id === selectedDirectoryId);
    }
    return directoryFilteredFiles;
  };

  // ファイルをフィルタリング
  const filterFilesByCategory = () => {
    const directoryFilteredFiles = filterFilesByDirectory();
    
    if (selectedCategory === "all") {
      filteredFiles = directoryFilteredFiles;
    } else {
      filteredFiles = directoryFilteredFiles.filter(file => getFileCategory(file) === selectedCategory);
    }
    
    currentPage = 1;
    updatePagination();
  };

  // カテゴリ選択
  const selectCategory = (category: FileCategory) => {
    selectedCategory = category;
    filterFilesByCategory();
  };

  // 検索結果のカテゴリ別ファイル数を計算
  const updateSearchCategoryCounts = () => {
    const counts: Record<FileCategory, number> = {
      all: searchResults.length,
      image: 0,
      audio: 0,
      video: 0,
      document: 0,
      archive: 0,
      other: 0
    };

    searchResults.forEach(result => {
      const category = getFileCategory(result.file);
      counts[category]++;
    });

    searchCategoryCounts = counts;
  };

  // 検索結果をフィルタリング
  const filterSearchResultsByCategory = () => {
    if (searchSelectedCategory === "all") {
      filteredSearchResults = [...searchResults];
    } else {
      filteredSearchResults = searchResults.filter(result => getFileCategory(result.file) === searchSelectedCategory);
    }
    
    searchCurrentPage = 1;
    updateSearchPagination();
  };

  // 検索結果のカテゴリ選択
  const selectSearchCategory = (category: FileCategory) => {
    searchSelectedCategory = category;
    filterSearchResultsByCategory();
  };
</script>

<div class="app">
  <LoadingScreen 
    isVisible={isLoading}
    progress={loadingProgress}
    steps={loadingSteps}
  />

  <header class="app-header">
    <h1>Clerica</h1>
    <p>Mac向けファイル整理・検索ツール</p>
  </header>

  <div class="app-content {isLoading ? 'loading' : ''}">
    <Sidebar
      {directories}
      {tags}
      {selectedDirectoryId}
      onAddDirectory={addDirectory}
      onSelectDirectory={selectDirectory}
      onRescanDirectory={rescanDirectory}
      onRemoveDirectory={removeDirectory}
      onCreateTag={createTag}
    />

    <div class="main-content">
      <div class="tabs">
        <button
          class="tab {activeTab === 'files' ? 'active' : ''}"
          onclick={() => (activeTab = "files")}
        >
          <FileText size={16} />
          ファイル
        </button>
        <button
          class="tab {activeTab === 'search' ? 'active' : ''}"
          onclick={() => (activeTab = "search")}
        >
          <Search size={16} />
          検索
        </button>
        <button
          class="tab {activeTab === 'tags' ? 'active' : ''}"
          onclick={() => (activeTab = "tags")}
        >
          <Tag size={16} />
          タグ管理
        </button>
      </div>

      <div class="content-area">
        {#if activeTab === "files"}
          <FilesView
            files={paginatedFiles}
            {selectedCategory}
            {categoryCounts}
            {currentPage}
            {totalFiles}
            {totalPages}
            onSelectFile={selectFile}
            onSelectCategory={selectCategory}
            onGoToPage={goToPage}
            onGoToPreviousPage={goToPreviousPage}
            onGoToNextPage={goToNextPage}
            onGoToFirstPage={goToFirstPage}
            onGoToLastPage={goToLastPage}
          />
        {/if}

        {#if activeTab === "search"}
          <SearchView
            bind:searchQuery={searchQuery}
            {searchResults}
            filteredResults={paginatedSearchResults}
            selectedCategory={searchSelectedCategory}
            categoryCounts={searchCategoryCounts}
            currentPage={searchCurrentPage}
            totalPages={searchTotalPages}
            onSearchQueryChange={(query) => searchQuery = query}
            onSearch={searchFiles}
            onSelectFile={selectFile}
            onSelectCategory={selectSearchCategory}
            onGoToPage={goToSearchPage}
            onGoToPreviousPage={goToSearchPreviousPage}
            onGoToNextPage={goToSearchNextPage}
            onGoToFirstPage={goToSearchFirstPage}
            onGoToLastPage={goToSearchLastPage}
          />
        {/if}

        {#if activeTab === "tags"}
          <TagsView {tags} />
        {/if}
      </div>
    </div>
  </div>

  <FileDetailModal
    file={selectedFile}
    {isDeleting}
    onOpenFile={openFile}
    onRevealInFinder={revealInFinder}
    onDeleteFile={deleteFile}
    onClose={closeFileDetails}
  />
</div>
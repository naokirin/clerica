<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open, confirm } from "@tauri-apps/plugin-dialog";
  import { FolderPlus, Search, Tag, FileText, X, RefreshCw } from "lucide-svelte";
  import "../lib/App.css";

  interface Directory {
    id: string;
    path: string;
    name: string;
    created_at: string;
    updated_at: string;
  }

  interface File {
    id: string;
    path: string;
    name: string;
    directory_id: string;
    size: number;
    file_type: string | null;
    created_at: string | null;
    modified_at: string | null;
    birth_time: string | null;
    inode: number | null;
    is_directory: boolean;
    created_at_db: string;
    updated_at_db: string;
    file_size: number | null;
    mime_type: string | null;
    permissions: string | null;
    owner_uid: number | null;
    group_gid: number | null;
    hard_links: number | null;
    device_id: number | null;
  }

  interface Tag {
    id: string;
    name: string;
    color: string;
    created_at: string;
  }

  interface SearchResult {
    file: File;
    tags: Tag[];
    score: number;
  }

  let directories: Directory[] = $state([]);
  let files: File[] = $state([]);
  let tags: Tag[] = $state([]);
  let searchQuery = $state("");
  let selectedTags: string[] = $state([]);
  let searchResults: SearchResult[] = $state([]);
  let activeTab: "files" | "search" | "tags" = $state("files");
  let selectedFile: File | null = $state(null);
  
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

  // ファイル種別の定義
  type FileCategory = "all" | "image" | "audio" | "video" | "document" | "archive" | "other";
  
  interface FileCategoryInfo {
    key: FileCategory;
    label: string;
    icon: string;
    mimeTypes: string[];
    extensions: string[];
  }

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

  onMount(() => {
    loadData();
  });

  const loadData = async () => {
    try {
      // ディレクトリの読み込み
      const directoriesData = await invoke("get_directories");
      directories = directoriesData as Directory[];
      
      // タグの読み込み
      const tagsData = await invoke("get_tags");
      tags = tagsData as Tag[];
      
      // ファイルの読み込み
      const filesData = await invoke("get_files");
      files = filesData as File[];
      
      // カテゴリ別ファイル数を計算
      updateCategoryCounts();
      
      // フィルタリングを適用
      filterFilesByCategory();
    } catch (error) {
      console.error("Failed to load data:", error);
    }
  };

  const addDirectory = async () => {
    try {
      // ディレクトリ選択ダイアログを開く
      const selected = await open({
        directory: true,
        multiple: false,
        title: "追加するディレクトリを選択"
      });
      
      if (selected && typeof selected === 'string') {
        // パスから名前を抽出
        const name = selected.split('/').pop() || selected;
        
        // ディレクトリをバックエンドに追加
        await invoke('add_directory', { path: selected, name });
        
        // データを再読み込み
        await loadData();
      }
    } catch (error) {
      console.error("Failed to add directory:", error);
      // エラー時はフォールバックとしてプロンプトを使用
      const fallbackPath = prompt("ディレクトリ選択に失敗しました。パスを直接入力してください:");
      if (fallbackPath && fallbackPath.trim()) {
        const name = fallbackPath.split('/').pop() || fallbackPath;
        await invoke('add_directory', { path: fallbackPath.trim(), name });
        await loadData();
      }
    }
  };

  const searchFiles = async () => {
    try {
      // ファイル検索の実装
      const results = await invoke('search_files', { 
        query: searchQuery, 
        tag_ids: selectedTags 
      });
      searchResults = results as SearchResult[];
      
      // 検索結果のカテゴリ別ファイル数を計算
      updateSearchCategoryCounts();
      
      // 検索結果のフィルタリングを適用
      filterSearchResultsByCategory();
    } catch (error) {
      console.error("Failed to search files:", error);
    }
  };

  const createTag = async () => {
    try {
      const name = prompt("Enter tag name:");
      if (name) {
        // タグを作成
        await invoke('create_tag', { name, color: '#3B82F6' });
        
        // データを再読み込み
        await loadData();
      }
    } catch (error) {
      console.error("Failed to create tag:", error);
    }
  };

  const rescanDirectory = async (directoryId: string) => {
    try {
      // ディレクトリを再スキャン
      await invoke("rescan_directory", { directoryId });
      // ファイル一覧を再読み込み
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
        // データを再読み込み
        await loadData();
      } catch (error) {
        console.error("Failed to remove directory:", error);
        alert("ディレクトリの削除に失敗しました。");
      }
    }
  };

  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const formatDate = (dateString: string | null): string => {
    if (!dateString) return 'N/A';
    const date = new Date(dateString);
    return date.toLocaleString('ja-JP');
  };

  const selectFile = (file: File) => {
    selectedFile = file;
  };

  const closeFileDetails = () => {
    selectedFile = null;
  };

  // ページネーション関数
  const updatePagination = () => {
    totalFiles = filteredFiles.length;
    totalPages = Math.ceil(totalFiles / itemsPerPage);
    
    // 現在のページが無効な場合は最初のページに戻る
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

  // ファイル種別判定関数
  const getFileCategory = (file: File): FileCategory => {
    if (file.is_directory) return "other";
    
    const mimeType = file.mime_type?.toLowerCase() || "";
    const extension = file.file_type?.toLowerCase() || "";
    
    for (const category of fileCategories) {
      if (category.key === "all" || category.key === "other") continue;
      
      // MIMEタイプでチェック
      for (const mime of category.mimeTypes) {
        if (mimeType.startsWith(mime.toLowerCase())) {
          return category.key;
        }
      }
      
      // 拡張子でチェック
      if (category.extensions.includes(extension)) {
        return category.key;
      }
    }
    
    return "other";
  };

  // カテゴリ別ファイル数を計算
  const updateCategoryCounts = () => {
    const counts: Record<FileCategory, number> = {
      all: files.length,
      image: 0,
      audio: 0,
      video: 0,
      document: 0,
      archive: 0,
      other: 0
    };

    files.forEach(file => {
      const category = getFileCategory(file);
      counts[category]++;
    });

    categoryCounts = counts;
  };

  // ファイルをフィルタリング
  const filterFilesByCategory = () => {
    if (selectedCategory === "all") {
      filteredFiles = [...files];
    } else {
      filteredFiles = files.filter(file => getFileCategory(file) === selectedCategory);
    }
    
    // フィルタリング後にページネーションを更新
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
    
    // フィルタリング後にページネーションを更新
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
  <header class="app-header">
    <h1>Clerica</h1>
    <p>Mac向けファイル整理・検索ツール</p>
  </header>

  <div class="app-content">
    <div class="sidebar">
      <div class="sidebar-section">
        <h3>ディレクトリ</h3>
        <button onclick={addDirectory} class="add-button">
          <FolderPlus size={16} />
          ディレクトリを追加
        </button>
        <div class="directory-list">
          {#each directories as dir (dir.id)}
            <div class="directory-item">
              <div class="directory-content">
                <div class="directory-name">{dir.name}</div>
                <div class="directory-path">{dir.path}</div>
              </div>
              <div class="directory-actions">
                <button 
                  class="rescan-directory-btn"
                  onclick={() => rescanDirectory(dir.id)}
                  title="ディレクトリを再スキャン"
                >
                  <RefreshCw size={14} />
                </button>
                <button 
                  class="remove-directory-btn"
                  onclick={() => removeDirectory(dir.id, dir.name)}
                  title="ディレクトリを登録から削除"
                >
                  <X size={14} />
                </button>
              </div>
            </div>
          {/each}
        </div>
      </div>

      <div class="sidebar-section">
        <h3>タグ</h3>
        <button onclick={createTag} class="add-button">
          <Tag size={16} />
          タグを作成
        </button>
        <div class="tag-list">
          {#each tags as tag (tag.id)}
            <div class="tag-item" style="border-left-color: {tag.color}">
              {tag.name}
            </div>
          {/each}
        </div>
      </div>
    </div>

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
                  onclick={() => selectCategory(category.key)}
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
                  onclick={goToFirstPage} 
                  disabled={currentPage === 1}
                >
                  ≪
                </button>
                <button 
                  class="pagination-btn" 
                  onclick={goToPreviousPage} 
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
                    onclick={() => goToPage(page)}
                  >
                    {page}
                  </button>
                {/each}
                
                <button 
                  class="pagination-btn" 
                  onclick={goToNextPage} 
                  disabled={currentPage === totalPages}
                >
                  ›
                </button>
                <button 
                  class="pagination-btn" 
                  onclick={goToLastPage} 
                  disabled={currentPage === totalPages}
                >
                  ≫
                </button>
              </div>
            {/if}

            <div class="file-list">
              {#each paginatedFiles as file (file.id)}
                <div class="file-item" onclick={() => selectFile(file)}>
                  <div class="file-icon">
                    {#if file.is_directory}
                      📁
                    {:else if file.mime_type?.startsWith('image/')}
                      🖼️
                    {:else if file.mime_type?.startsWith('video/')}
                      🎬
                    {:else if file.mime_type?.startsWith('audio/')}
                      🎵
                    {:else if file.mime_type?.includes('pdf')}
                      📄
                    {:else if file.mime_type?.includes('text')}
                      📝
                    {:else}
                      📄
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
              {#if paginatedFiles.length === 0 && totalFiles === 0}
                <div class="no-files">
                  ディレクトリを追加してファイルをスキャンしてください
                </div>
              {/if}
            </div>

            {#if totalPages > 1}
              <div class="pagination-controls pagination-bottom">
                <button 
                  class="pagination-btn" 
                  onclick={goToFirstPage} 
                  disabled={currentPage === 1}
                >
                  ≪
                </button>
                <button 
                  class="pagination-btn" 
                  onclick={goToPreviousPage} 
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
                    onclick={() => goToPage(page)}
                  >
                    {page}
                  </button>
                {/each}
                
                <button 
                  class="pagination-btn" 
                  onclick={goToNextPage} 
                  disabled={currentPage === totalPages}
                >
                  ›
                </button>
                <button 
                  class="pagination-btn" 
                  onclick={goToLastPage} 
                  disabled={currentPage === totalPages}
                >
                  ≫
                </button>
              </div>
            {/if}
          </div>
        {/if}

        {#if activeTab === "search"}
          <div class="search-view">
            <div class="search-header">
              <h2>ファイル検索</h2>
              {#if searchResults.length > 0}
                <div class="search-stats">
                  <span class="total-results">
                    {searchSelectedCategory === "all" ? "検索結果" : fileCategories.find(c => c.key === searchSelectedCategory)?.label}: 
                    {filteredSearchResults.length.toLocaleString()} 件
                  </span>
                  {#if searchTotalPages > 1}
                    <span class="page-info">
                      ページ {searchCurrentPage} / {searchTotalPages} 
                      ({((searchCurrentPage - 1) * itemsPerPage + 1).toLocaleString()} - {Math.min(searchCurrentPage * itemsPerPage, filteredSearchResults.length).toLocaleString()})
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
              <button onclick={searchFiles} class="search-button">
                <Search size={16} />
                検索
              </button>
            </div>

            <!-- 検索結果のファイル種別フィルター -->
            {#if searchResults.length > 0}
              <div class="file-category-filters">
                {#each fileCategories as category (category.key)}
                  <button
                    class="category-filter-btn {searchSelectedCategory === category.key ? 'active' : ''}"
                    onclick={() => selectSearchCategory(category.key)}
                    disabled={category.key !== "all" && searchCategoryCounts[category.key] === 0}
                  >
                    <span class="category-icon">{category.icon}</span>
                    <span class="category-label">{category.label}</span>
                    <span class="category-count">({searchCategoryCounts[category.key].toLocaleString()})</span>
                  </button>
                {/each}
              </div>
            {/if}

            {#if searchTotalPages > 1}
              <div class="pagination-controls">
                <button 
                  class="pagination-btn" 
                  onclick={goToSearchFirstPage} 
                  disabled={searchCurrentPage === 1}
                >
                  ≪
                </button>
                <button 
                  class="pagination-btn" 
                  onclick={goToSearchPreviousPage} 
                  disabled={searchCurrentPage === 1}
                >
                  ‹
                </button>
                
                {#each Array.from({length: Math.min(7, searchTotalPages)}, (_, i) => {
                  let start = Math.max(1, searchCurrentPage - 3);
                  let end = Math.min(searchTotalPages, start + 6);
                  start = Math.max(1, end - 6);
                  return start + i;
                }).filter(page => page <= searchTotalPages) as page}
                  <button 
                    class="pagination-btn {searchCurrentPage === page ? 'active' : ''}" 
                    onclick={() => goToSearchPage(page)}
                  >
                    {page}
                  </button>
                {/each}
                
                <button 
                  class="pagination-btn" 
                  onclick={goToSearchNextPage} 
                  disabled={searchCurrentPage === searchTotalPages}
                >
                  ›
                </button>
                <button 
                  class="pagination-btn" 
                  onclick={goToSearchLastPage} 
                  disabled={searchCurrentPage === searchTotalPages}
                >
                  ≫
                </button>
              </div>
            {/if}

            <div class="search-results">
              {#each paginatedSearchResults as result (result.file.id)}
                <div class="search-result-item" onclick={() => selectFile(result.file)}>
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

            {#if searchTotalPages > 1}
              <div class="pagination-controls pagination-bottom">
                <button 
                  class="pagination-btn" 
                  onclick={goToSearchFirstPage} 
                  disabled={searchCurrentPage === 1}
                >
                  ≪
                </button>
                <button 
                  class="pagination-btn" 
                  onclick={goToSearchPreviousPage} 
                  disabled={searchCurrentPage === 1}
                >
                  ‹
                </button>
                
                {#each Array.from({length: Math.min(7, searchTotalPages)}, (_, i) => {
                  let start = Math.max(1, searchCurrentPage - 3);
                  let end = Math.min(searchTotalPages, start + 6);
                  start = Math.max(1, end - 6);
                  return start + i;
                }).filter(page => page <= searchTotalPages) as page}
                  <button 
                    class="pagination-btn {searchCurrentPage === page ? 'active' : ''}" 
                    onclick={() => goToSearchPage(page)}
                  >
                    {page}
                  </button>
                {/each}
                
                <button 
                  class="pagination-btn" 
                  onclick={goToSearchNextPage} 
                  disabled={searchCurrentPage === searchTotalPages}
                >
                  ›
                </button>
                <button 
                  class="pagination-btn" 
                  onclick={goToSearchLastPage} 
                  disabled={searchCurrentPage === searchTotalPages}
                >
                  ≫
                </button>
              </div>
            {/if}
          </div>
        {/if}

        {#if activeTab === "tags"}
          <div class="tags-view">
            <h2>タグ管理</h2>
            <div class="tags-grid">
              {#each tags as tag (tag.id)}
                <div class="tag-card" style="border-color: {tag.color}">
                  <div class="tag-name">{tag.name}</div>
                  <div
                    class="tag-color"
                    style="background-color: {tag.color}"
                  ></div>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <!-- ファイル詳細モーダル -->
  {#if selectedFile}
    <div class="modal-overlay" onclick={closeFileDetails}>
      <div class="modal-content" onclick={(e) => e.stopPropagation()}>
        <div class="modal-header">
          <h3>ファイル詳細</h3>
          <button class="close-button" onclick={closeFileDetails}>
            <X size={20} />
          </button>
        </div>
        <div class="modal-body">
          <div class="file-detail-section">
            <h4>基本情報</h4>
            <div class="detail-grid">
              <div class="detail-item">
                <span class="detail-label">ファイル名:</span>
                <span class="detail-value">{selectedFile.name}</span>
              </div>
              <div class="detail-item">
                <span class="detail-label">パス:</span>
                <span class="detail-value">{selectedFile.path}</span>
              </div>
              <div class="detail-item">
                <span class="detail-label">サイズ:</span>
                <span class="detail-value">{formatFileSize(selectedFile.file_size || selectedFile.size)}</span>
              </div>
              <div class="detail-item">
                <span class="detail-label">種類:</span>
                <span class="detail-value">{selectedFile.is_directory ? 'ディレクトリ' : (selectedFile.mime_type || selectedFile.file_type || 'Unknown')}</span>
              </div>
            </div>
          </div>

          <div class="file-detail-section">
            <h4>日時情報</h4>
            <div class="detail-grid">
              <div class="detail-item">
                <span class="detail-label">作成日時:</span>
                <span class="detail-value">{formatDate(selectedFile.created_at)}</span>
              </div>
              <div class="detail-item">
                <span class="detail-label">更新日時:</span>
                <span class="detail-value">{formatDate(selectedFile.modified_at)}</span>
              </div>
              {#if selectedFile.birth_time}
                <div class="detail-item">
                  <span class="detail-label">作成日時 (birth_time):</span>
                  <span class="detail-value">{formatDate(selectedFile.birth_time)}</span>
                </div>
              {/if}
            </div>
          </div>

          <div class="file-detail-section">
            <h4>システム情報</h4>
            <div class="detail-grid">
              {#if selectedFile.permissions}
                <div class="detail-item">
                  <span class="detail-label">権限:</span>
                  <span class="detail-value">{selectedFile.permissions}</span>
                </div>
              {/if}
              {#if selectedFile.owner_uid !== null}
                <div class="detail-item">
                  <span class="detail-label">オーナー UID:</span>
                  <span class="detail-value">{selectedFile.owner_uid}</span>
                </div>
              {/if}
              {#if selectedFile.group_gid !== null}
                <div class="detail-item">
                  <span class="detail-label">グループ GID:</span>
                  <span class="detail-value">{selectedFile.group_gid}</span>
                </div>
              {/if}
              {#if selectedFile.inode !== null}
                <div class="detail-item">
                  <span class="detail-label">inode:</span>
                  <span class="detail-value">{selectedFile.inode}</span>
                </div>
              {/if}
              {#if selectedFile.hard_links !== null}
                <div class="detail-item">
                  <span class="detail-label">ハードリンク数:</span>
                  <span class="detail-value">{selectedFile.hard_links}</span>
                </div>
              {/if}
              {#if selectedFile.device_id !== null}
                <div class="detail-item">
                  <span class="detail-label">デバイス ID:</span>
                  <span class="detail-value">{selectedFile.device_id}</span>
                </div>
              {/if}
            </div>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

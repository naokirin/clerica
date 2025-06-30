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
            <h2>ファイル一覧</h2>
            <div class="file-list">
              {#each files as file (file.id)}
                <div class="file-item">
                  <div class="file-icon">
                    {#if file.is_directory}
                      📁
                    {:else}
                      📄
                    {/if}
                  </div>
                  <div class="file-details">
                    <div class="file-name">{file.name}</div>
                    <div class="file-info">
                      {#if !file.is_directory}
                        {(file.size / 1024).toFixed(1)} KB • {file.file_type || "Unknown"}
                      {:else}
                        ディレクトリ
                      {/if}
                    </div>
                    <div class="file-path">{file.path}</div>
                  </div>
                </div>
              {/each}
              {#if files.length === 0}
                <div class="no-files">
                  ディレクトリを追加してファイルをスキャンしてください
                </div>
              {/if}
            </div>
          </div>
        {/if}

        {#if activeTab === "search"}
          <div class="search-view">
            <h2>ファイル検索</h2>
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
            <div class="search-results">
              {#each searchResults as result (result.file.id)}
                <div class="search-result-item">
                  <div class="result-file-name">{result.file.name}</div>
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
              {/each}
            </div>
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
</div>

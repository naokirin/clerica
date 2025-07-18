<script lang="ts">
  import { FolderPlus, Tag, RefreshCw, X } from "lucide-svelte";
  import type { Directory, Tag as TagType } from "../types";
  import { t } from "$lib/i18n";
  import ShelfManager from "./ShelfManager.svelte";
  import type { AppViewModel } from "$lib/viewmodels/AppViewModel";

  interface Props {
    directories: Directory[];
    tags: TagType[];
    selectedDirectoryId: string | "all";
    onAddDirectory: () => void;
    onSelectDirectory: (id: string | "all") => void;
    onRescanDirectory: (id: string) => void;
    onRemoveDirectory: (id: string, name: string) => void;
    onCreateTag: () => void;
    onRescanAll: () => void;
    disabled?: boolean;
    appViewModel?: AppViewModel;
  }

  let {
    directories,
    tags,
    selectedDirectoryId,
    onAddDirectory,
    onSelectDirectory,
    onRescanDirectory,
    onRemoveDirectory,
    onCreateTag,
    onRescanAll,
    disabled = false,
    appViewModel,
  }: Props = $props();
</script>

<div class="sidebar" class:disabled>
  <!-- シェルフセクション -->
  <ShelfManager {appViewModel} />

  <div class="sidebar-section">
    <div class="section-header">
      <h3>{$t("common.sidebar.directories")}</h3>
      <button onclick={onAddDirectory} class="add-button" {disabled}>
        <FolderPlus size={16} />
        {$t("common.sidebar.addDirectory")}
      </button>
    </div>
    <div class="directory-list">
      <!-- すべてのファイルオプション -->
      <div
        class="directory-item {selectedDirectoryId === 'all' ? 'selected' : ''}"
        onclick={() => onSelectDirectory("all")}
      >
        <div class="directory-content">
          <div class="directory-name">{$t("common.sidebar.allFiles")}</div>
          <div class="directory-path">
            {$t("common.sidebar.allFilesDescription")}
          </div>
        </div>
        <div class="directory-actions">
          <button
            class="rescan-directory-btn"
            onclick={(e) => {
              e.stopPropagation();
              onRescanAll();
            }}
            title={$t("common.sidebar.rescanAll")}
          >
            <RefreshCw size={14} />
          </button>
        </div>
      </div>

      {#each directories as dir (dir.id)}
        <div
          class="directory-item {selectedDirectoryId === dir.id
            ? 'selected'
            : ''}"
          onclick={() => onSelectDirectory(dir.id)}
        >
          <div class="directory-content">
            <div class="directory-name">{dir.name}</div>
            <div class="directory-path">{dir.path}</div>
          </div>
          <div class="directory-actions">
            <button
              class="rescan-directory-btn"
              onclick={(e) => {
                e.stopPropagation();
                onRescanDirectory(dir.id);
              }}
              title={$t("common.sidebar.rescan")}
            >
              <RefreshCw size={14} />
            </button>
            <button
              class="remove-directory-btn"
              onclick={(e) => {
                e.stopPropagation();
                onRemoveDirectory(dir.id, dir.name);
              }}
              title={$t("common.sidebar.remove")}
            >
              <X size={14} />
            </button>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .sidebar.disabled {
    pointer-events: none;
    opacity: 0.7;
  }
</style>

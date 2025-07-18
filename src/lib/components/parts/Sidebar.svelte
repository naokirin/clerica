<script lang="ts">
  import { FolderPlus, Tag, RefreshCw, X } from "lucide-svelte";
  import type { Directory, Tag as TagType } from "../../types";
  import { t } from "$lib/i18n";
  import ShelfManager from "../modules/settings/ShelfManager.svelte";
  import type { AppViewModel } from "$lib/viewmodels/AppViewModel";
  import Button from "./Button.svelte";
  import IconButton from "./IconButton.svelte";

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

  // アイコン定義
  const folderPlusIcon =
    '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"/><line x1="12" y1="10" x2="12" y2="16"/><line x1="9" y1="13" x2="15" y2="13"/></svg>';
</script>

<div class="sidebar" class:disabled>
  <!-- シェルフセクション -->
  <ShelfManager {appViewModel} />

  <div class="sidebar-section">
    <div class="section-header">
      <h3>{$t("common.sidebar.directories")}</h3>
      <Button
        onclick={onAddDirectory}
        {disabled}
        icon={folderPlusIcon}
        text={$t("common.sidebar.addDirectory")}
      />
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
          <IconButton
            icon={RefreshCw}
            title={$t("common.sidebar.rescanAll")}
            onClick={() => onRescanAll()}
            class="green"
          />
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
            <IconButton
              icon={RefreshCw}
              title={$t("common.sidebar.rescan")}
              onClick={() => onRescanDirectory(dir.id)}
              class="green"
            />
            <IconButton
              icon={X}
              title={$t("common.sidebar.remove")}
              onClick={() => onRemoveDirectory(dir.id, dir.name)}
              class="red"
            />
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

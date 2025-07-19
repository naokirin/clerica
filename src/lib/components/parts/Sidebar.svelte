<script lang="ts">
  import { FolderPlus, Tag, RefreshCw, X, PlusIcon } from "@lucide/svelte";
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
        iconName="Plus"
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

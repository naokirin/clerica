<script lang="ts">
  import type { File, Tag } from "../../../types";
  import type { ViewMode } from "../../../stores/common";
  import {
    formatFileSize,
    formatDate,
    isImageFile,
    isVideoFile,
    isAudioFile,
    isArchiveFile,
    getImageUrl,
  } from "../../../utils";
  import { onMount } from "svelte";
  import { errorStore } from "../../../stores/error";
  import { selectedFileIds } from "../../../stores/files";

  interface Props {
    file: File;
    tags?: Tag[];
    viewMode: ViewMode;
    onSelectFile: (file: File) => void;
  }

  let { file, tags = [], viewMode, onSelectFile }: Props = $props();

  // 選択状態の管理
  let isSelected = $state(false);
  let lastSelectedId: number | null = null;

  // 選択状態の同期
  $effect(() => {
    $selectedFileIds.has(file.id);
    isSelected = $selectedFileIds.has(file.id);
  });

  // チェックボックスの変更ハンドラー
  const handleCheckboxChange = (event: Event) => {
    event.stopPropagation();
    const checked = (event.target as HTMLInputElement).checked;

    selectedFileIds.update((currentSelected) => {
      const newSelected = new Set(currentSelected);

      if (checked) {
        newSelected.add(file.id);
      } else {
        newSelected.delete(file.id);
      }

      return newSelected;
    });
  };

  // Shift+Clickでの範囲選択ハンドラー
  const handleItemClick = (event: MouseEvent) => {
    if (event.shiftKey && lastSelectedId !== null) {
      // Shift+Clickの場合は範囲選択
      event.preventDefault();
      selectFileRange(lastSelectedId, file.id);
    } else if (event.ctrlKey || event.metaKey) {
      // Ctrl/Cmd+Clickの場合は単一選択切り替え
      event.preventDefault();
      handleCheckboxChange({
        target: { checked: !isSelected },
        stopPropagation: () => {},
      } as any);
    } else {
      // 通常クリックの場合は詳細表示
      onSelectFile(file);
    }

    lastSelectedId = file.id;
  };

  // 範囲選択の実装
  const selectFileRange = (startId: number, endId: number) => {
    // ここではシンプルにID順で範囲選択を実装
    // 実際のファイル表示順序に合わせる場合は、親コンポーネントから
    // ファイルの順序情報を受け取る必要がある
    const minId = Math.min(startId, endId);
    const maxId = Math.max(startId, endId);

    selectedFileIds.update((currentSelected) => {
      const newSelected = new Set(currentSelected);

      for (let id = minId; id <= maxId; id++) {
        newSelected.add(id);
      }

      return newSelected;
    });
  };

  let imageUrlCache = new Map<string, string>();

  async function loadImageUrl(filePath: string): Promise<string> {
    if (imageUrlCache.has(filePath)) {
      return imageUrlCache.get(filePath)!;
    }

    const url = await getImageUrl(filePath);
    imageUrlCache.set(filePath, url);
    return url;
  }

  async function loadVideoThumbnail(filePath: string): Promise<string> {
    // 動画サムネイルを生成するTauri関数を呼び出す
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const thumbnailPath = await invoke("generate_video_thumbnail", {
        filePath,
      });
      return await getImageUrl(thumbnailPath);
    } catch (error) {
      console.error("Failed to generate video thumbnail:", error);
      errorStore.showError("動画のサムネイル生成に失敗しました");
      throw error;
    }
  }

  async function loadAlbumArt(filePath: string): Promise<string> {
    // 音声ファイルのアルバムアートを抽出するTauri関数を呼び出す
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const thumbnailPath = await invoke("extract_audio_album_art", {
        filePath,
      });
      return await getImageUrl(thumbnailPath);
    } catch (error) {
      console.error("Failed to extract album art:", error);
      errorStore.showError("音楽ファイルのアルバムアート抽出に失敗しました");
      throw error;
    }
  }

  async function loadArchiveThumbnail(filePath: string): Promise<string> {
    // アーカイブファイルのサムネイルを生成するTauri関数を呼び出す
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const thumbnailPath = await invoke("generate_archive_thumbnail", {
        filePath,
      });
      return await getImageUrl(thumbnailPath);
    } catch (error) {
      console.error("Failed to generate archive thumbnail:", error);
      errorStore.showError("アーカイブファイルのサムネイル生成に失敗しました");
      throw error;
    }
  }

  // アイコンコンポーネント
  function renderFileIcon() {
    if (file.is_directory) {
      return "📁";
    } else if (file.mime_type?.includes("pdf")) {
      return "📄";
    } else if (file.mime_type?.includes("text")) {
      return "📝";
    } else {
      return "📄";
    }
  }
</script>

<div
  class="file-item {viewMode}-item {isSelected ? 'selected' : ''}"
  onclick={handleItemClick}
>
  <div class="file-checkbox {viewMode}-checkbox">
    <input
      type="checkbox"
      checked={isSelected}
      onchange={handleCheckboxChange}
      onclick={(e) => e.stopPropagation()}
    />
  </div>

  <div class="file-icon {viewMode}-icon">
    {#if file.is_directory}
      <span class="icon-emoji">📁</span>
    {:else if isImageFile(file)}
      <div class="image-preview">
        {#await loadImageUrl(file.path)}
          <span class="icon-emoji loading">🖼️</span>
        {:then imageUrl}
          <img
            src={imageUrl}
            alt={file.name}
            class="thumbnail"
            onerror={(e) => {
              console.error("Failed to load image:", imageUrl);
              errorStore.showWarning("画像の読み込みに失敗しました");
              e.currentTarget.style.display = "none";
              e.currentTarget.nextElementSibling.style.display = "block";
            }}
          />
          <span class="icon-emoji fallback" style="display: none;">🖼️</span>
        {:catch}
          <span class="icon-emoji">🖼️</span>
        {/await}
      </div>
    {:else if isVideoFile(file)}
      <div class="video-preview">
        {#await loadVideoThumbnail(file.path)}
          <span class="icon-emoji loading">🎬</span>
        {:then thumbnailUrl}
          <img
            src={thumbnailUrl}
            alt={file.name}
            class="thumbnail video-thumbnail"
            onerror={(e) => {
              console.error("Failed to load video thumbnail:", thumbnailUrl);
              errorStore.showWarning("動画サムネイルの読み込みに失敗しました");
              e.currentTarget.style.display = "none";
              e.currentTarget.nextElementSibling.style.display = "block";
            }}
          />
          <div class="video-overlay">
            <span class="play-icon">▶️</span>
          </div>
          <span class="icon-emoji fallback" style="display: none;">🎬</span>
        {:catch}
          <span class="icon-emoji">🎬</span>
        {/await}
      </div>
    {:else if isAudioFile(file)}
      <div class="audio-preview">
        {#await loadAlbumArt(file.path)}
          <span class="icon-emoji loading">🎵</span>
        {:then albumArtUrl}
          <img
            src={albumArtUrl}
            alt={file.name}
            class="thumbnail album-art"
            onerror={(e) => {
              console.error("Failed to load album art:", albumArtUrl);
              errorStore.showWarning("アルバムアートの読み込みに失敗しました");
              e.currentTarget.style.display = "none";
              e.currentTarget.nextElementSibling.style.display = "block";
            }}
          />
          <div class="audio-overlay">
            <span class="music-icon">♪</span>
          </div>
          <span class="icon-emoji fallback" style="display: none;">🎵</span>
        {:catch}
          <span class="icon-emoji">🎵</span>
        {/await}
      </div>
    {:else if isArchiveFile(file)}
      <div class="archive-preview">
        {#await loadArchiveThumbnail(file.path)}
          <span class="icon-emoji loading">📦</span>
        {:then thumbnailUrl}
          <img
            src={thumbnailUrl}
            alt={file.name}
            class="thumbnail archive-thumbnail"
            onerror={(e) => {
              console.error("Failed to load archive thumbnail:", thumbnailUrl);
              errorStore.showWarning(
                "アーカイブサムネイルの読み込みに失敗しました",
              );
              e.currentTarget.style.display = "none";
              e.currentTarget.nextElementSibling.style.display = "block";
            }}
          />
          <div class="archive-overlay">
            <span class="archive-icon">📦</span>
          </div>
          <span class="icon-emoji fallback" style="display: none;">📦</span>
        {:catch}
          <span class="icon-emoji">📦</span>
        {/await}
      </div>
    {:else}
      <span class="icon-emoji">{renderFileIcon()}</span>
    {/if}
  </div>

  <div class="file-details {viewMode}-details">
    <div class="file-name">{file.name}</div>
    {#if viewMode === "list"}
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
    {:else}
      <!-- グリッド表示では簡潔な情報のみ -->
      <div class="file-info">
        {#if !file.is_directory}
          {formatFileSize(file.file_size || file.size)}
        {:else}
          ディレクトリ
        {/if}
      </div>
    {/if}
    {#if tags.length > 0}
      <div class="file-tags">
        {#each tags as tag (tag.id)}
          <span class="file-tag" style="background-color: {tag.color}">
            {tag.name}
          </span>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  /* 共通スタイル */
  .file-item {
    border-radius: 6px;
    cursor: pointer;
    transition: background-color 0.2s ease;
    border: 2px solid transparent;
  }

  .file-item:hover {
    background-color: #f8f9fa;
  }

  .file-item.selected {
    background-color: #e3f2fd;
    border-color: #2196f3;
  }

  .file-item.selected:hover {
    background-color: #bbdefb;
  }

  /* リスト表示スタイル */
  .list-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px;
  }

  .list-checkbox {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    flex-shrink: 0;
  }

  .list-icon {
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .list-details {
    flex: 1;
    min-width: 0;
  }

  /* グリッド表示スタイル */
  .grid-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 16px 12px;
    position: relative;
  }

  .grid-checkbox {
    position: absolute;
    top: 16px;
    left: 14px;
    width: 18px;
    height: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.9);
    border-radius: 3px;
    z-index: 1;
  }

  .grid-checkbox input[type="checkbox"] {
    width: 14px;
    height: 14px;
    -moz-transform: scale(1.4);
    -webkit-transform: scale(1.4);
    transform: scale(1.4);
  }

  .grid-icon {
    width: 80px;
    height: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 8px;
  }

  .grid-details {
    width: 100%;
  }

  .grid-details .file-name {
    font-size: 0.875rem;
    line-height: 1.3;
    margin-bottom: 4px;
    overflow: hidden;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
  }

  .grid-details .file-info {
    font-size: 0.75rem;
    color: #666;
  }

  /* チェックボックス共通スタイル */
  .file-checkbox input[type="checkbox"] {
    cursor: pointer;
  }

  .list-checkbox input[type="checkbox"] {
    width: 16px;
    height: 16px;
  }

  /* アイコン共通スタイル */
  .icon-emoji {
    line-height: 1;
  }

  .list-icon .icon-emoji {
    font-size: 32px;
  }

  .grid-icon .icon-emoji {
    font-size: 48px;
  }

  .image-preview,
  .video-preview,
  .audio-preview,
  .archive-preview {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    overflow: hidden;
    background-color: #f5f5f5;
    position: relative;
  }

  .grid-icon .image-preview,
  .grid-icon .video-preview,
  .grid-icon .audio-preview,
  .grid-icon .archive-preview {
    border-radius: 8px;
  }

  .thumbnail {
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform 0.2s ease;
  }

  .list-icon .thumbnail {
    border-radius: 4px;
  }

  .grid-icon .thumbnail {
    border-radius: 8px;
  }

  .thumbnail:hover {
    transform: scale(1.05);
  }

  .video-thumbnail {
    filter: brightness(0.8);
  }

  .video-overlay,
  .audio-overlay,
  .archive-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: rgba(0, 0, 0, 0.3);
    pointer-events: none;
    opacity: 0.7;
  }

  .list-icon .video-overlay,
  .list-icon .audio-overlay,
  .list-icon .archive-overlay {
    border-radius: 4px;
  }

  .grid-icon .video-overlay,
  .grid-icon .audio-overlay,
  .grid-icon .archive-overlay {
    border-radius: 8px;
  }

  .play-icon,
  .music-icon,
  .archive-icon {
    opacity: 0.9;
    color: white;
  }

  .list-icon .play-icon,
  .list-icon .music-icon,
  .list-icon .archive-icon {
    font-size: 12px;
  }

  .grid-icon .play-icon,
  .grid-icon .music-icon,
  .grid-icon .archive-icon {
    font-size: 20px;
  }

  .album-art,
  .archive-thumbnail {
    filter: brightness(0.8);
  }

  .loading {
    opacity: 0.6;
    animation: pulse 1.5s ease-in-out infinite alternate;
  }

  @keyframes pulse {
    from {
      opacity: 0.6;
    }
    to {
      opacity: 1;
    }
  }

  .fallback {
    color: #999;
  }

  /* ファイル詳細共通スタイル */
  .file-name {
    font-weight: 500;
    color: #333;
    word-break: break-word;
  }

  .file-info {
    font-size: 0.875rem;
    color: #666;
    margin-top: 2px;
  }

  .file-path {
    font-size: 0.75rem;
    color: #999;
    margin-top: 2px;
    word-break: break-all;
  }

  .file-meta {
    font-size: 0.75rem;
    color: #999;
    margin-top: 2px;
  }

  .file-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    margin-top: 4px;
  }

  .grid-details .file-tags {
    justify-content: center;
  }

  .file-tag {
    padding: 2px 6px;
    border-radius: 12px;
    font-size: 0.75rem;
    color: white;
    font-weight: 500;
  }
</style>

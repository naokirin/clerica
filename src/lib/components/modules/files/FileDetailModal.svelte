<script lang="ts">
  import { X, Trash2, Loader2, ExternalLink, Folder, Edit3 } from "lucide-svelte";
  import type { File, CustomMetadataKey, Tag } from "../../../types";
  import { formatFileSize, formatDate } from "../../../utils";
  import CustomMetadataEditor from "../metadata/CustomMetadataEditor.svelte";
  import TagInput from "../../parts/TagInput.svelte";
  import Dropdown from "../../parts/Dropdown.svelte";
  import * as exifApi from "../../../api/exif";
  import * as filesApi from "../../../api/files";
  import * as tagsApi from "../../../api/tags";
  import { errorStore } from "../../../stores/error";
  import { t } from "$lib/i18n";

  interface Props {
    file: File | null;
    isDeleting: boolean;
    customMetadataKeys: CustomMetadataKey[];
    onOpenFile: (filePath: string) => void;
    onRevealInFinder: (filePath: string) => void;
    onDeleteFile: (filePath: string, fileName: string) => void;
    onClose: () => void;
    onTagsUpdated?: () => void;
    onFileRenamed?: () => void;
    onOpenRenameModal?: (file: File) => void;
  }

  let {
    file,
    isDeleting,
    customMetadataKeys,
    onOpenFile,
    onRevealInFinder,
    onDeleteFile,
    onClose,
    onTagsUpdated,
    onFileRenamed,
    onOpenRenameModal,
  }: Props = $props();

  // タグの状態管理
  let currentTags = $state<Tag[]>([]);
  let originalTags: Tag[] = [];
  let isSavingTags = $state(false);
  let isLoadingTags = $state(false);
  let showSavedIndicator = $state(false);


  // EXIFメタデータの値を解釈する関数
  async function interpretExifValue(key: string, value: any): Promise<string> {
    if (value === null || value === undefined)
      return $t("common.fileDetail.unknown");

    switch (key) {
      case "Orientation":
        const orientationName = await exifApi.getOrientationName(value);
        return orientationName || $t("common.exif.unknownValue", { value });

      case "ColorSpace":
        const colorSpaceName = await exifApi.getColorSpaceName(value);
        return colorSpaceName || $t("common.exif.unknownValue", { value });

      case "MeteringMode":
        const meteringModeName = await exifApi.getMeteringModeName(value);
        return meteringModeName || $t("common.exif.unknownValue", { value });

      case "LightSource":
        const lightSourceName = await exifApi.getLightSourceName(value);
        return lightSourceName || $t("common.exif.unknownValue", { value });

      case "WhiteBalance":
        const whiteBalanceName = await exifApi.getWhiteBalanceName(value);
        return whiteBalanceName || $t("common.exif.unknownValue", { value });

      case "SceneCaptureType":
        const sceneCaptureTypeName =
          await exifApi.getSceneCaptureTypeName(value);
        return (
          sceneCaptureTypeName || $t("common.exif.unknownValue", { value })
        );

      case "Contrast":
      case "Saturation":
      case "Sharpness":
        const enhancementName = await exifApi.getEnhancementName(value);
        return enhancementName || $t("common.exif.unknownValue", { value });

      case "SubjectDistanceRange":
        const subjectDistanceRangeName =
          await exifApi.getSubjectDistanceRangeName(value);
        return (
          subjectDistanceRangeName || $t("common.exif.unknownValue", { value })
        );

      case "Flash":
        return (
          $t(`exif.flashModes.${value}`) ||
          $t("common.exif.unknownValue", { value })
        );

      case "WhiteBalance":
        return (
          $t(`exif.whiteBalanceModes.${value}`) ||
          $t("common.exif.unknownValue", { value })
        );

      case "ColorSpace":
        return (
          $t(`exif.colorSpaces.${value}`) ||
          $t("common.exif.unknownValue", { value })
        );

      case "MeteringMode":
        return (
          $t(`exif.meteringModes.${value}`) ||
          $t("common.exif.unknownValue", { value })
        );

      case "ExposureMode":
        return (
          $t(`exif.exposureModes.${value}`) ||
          $t("common.exif.unknownValue", { value })
        );

      case "SceneCaptureType":
        return (
          $t(`exif.sceneTypes.${value}`) ||
          $t("common.exif.unknownValue", { value })
        );

      case "ExposureProgram":
        return (
          $t(`exif.exposurePrograms.${value}`) ||
          $t("common.exif.unknownValue", { value })
        );

      case "FocalLength":
        if (Array.isArray(value) && value.length > 0) {
          return `${value[0]} ${$t("common.exif.units.mm")}`;
        }
        return `${value} ${$t("common.exif.units.mm")}`;

      case "ExposureTime":
        if (Array.isArray(value) && value.length > 0) {
          const seconds = value[0];
          if (seconds >= 1) {
            return `${seconds} ${$t("common.exif.units.seconds")}`;
          } else {
            return $t("common.exif.units.secondsShort", {
              value: Math.round(1 / seconds),
            });
          }
        }
        return `${value}`;

      case "FNumber":
        if (Array.isArray(value) && value.length > 0) {
          return `F/${value[0]}`;
        }
        return `F/${value}`;

      case "ISOSpeedRatings":
        if (Array.isArray(value)) {
          return value.join(", ");
        }
        return $t("common.exif.units.iso", { value });

      case "LightSource":
        return (
          $t(`exif.lightSources.${value}`) ||
          $t("common.exif.unknownValue", { value })
        );

      case "ResolutionUnit":
        return (
          $t(`exif.resolutionUnits.${value}`) ||
          $t("common.exif.unknownValue", { value })
        );

      case "YCbCrPositioning":
        return (
          $t(`exif.yCbCrPositions.${value}`) ||
          $t("common.exif.unknownValue", { value })
        );

      case "XResolution":
      case "YResolution":
        if (Array.isArray(value) && value.length > 0) {
          return `${value[0]} ${$t("common.exif.units.dpi")}`;
        }
        return `${value} ${$t("common.exif.units.dpi")}`;

      case "CompressedBitsPerPixel":
        if (Array.isArray(value) && value.length > 0) {
          return `${value[0]} ${$t("common.exif.units.bitsPerPixel")}`;
        }
        return `${value} ${$t("common.exif.units.bitsPerPixel")}`;

      case "ExifImageWidth":
      case "ExifImageHeight":
      case "PixelXDimension":
      case "PixelYDimension":
        return `${value} ${$t("common.exif.units.pixels")}`;

      case "ComponentsConfiguration":
        if (Array.isArray(value)) {
          const components = value.map((comp: number) => {
            switch (comp) {
              case 0:
                return "-";
              case 1:
                return "Y";
              case 2:
                return "Cb";
              case 3:
                return "Cr";
              case 4:
                return "R";
              case 5:
                return "G";
              case 6:
                return "B";
              default:
                return "?";
            }
          });
          return components.join("");
        }
        return String(value);

      default:
        if (typeof value === "object" && value !== null) {
          if (Array.isArray(value)) {
            return value.join(", ");
          } else {
            return JSON.stringify(value);
          }
        }
        return String(value);
    }
  }

  // EXIFタグ番号から名前を取得する関数
  async function getExifTagName(tagNumber: number): Promise<string> {
    const tagName = await exifApi.getTagName(tagNumber);
    return tagName || $t("common.fileDetail.tagNumber", { number: tagNumber });
  }

  // ファイルのタグを読み込む関数
  async function loadFileTags() {
    if (!file) return;

    isLoadingTags = true;
    try {
      const tags = await filesApi.getFileTags(file.id);
      currentTags = [...tags];
      originalTags = [...tags];
    } catch (error) {
      console.error($t("common.error.tagsLoadError"), error);
      errorStore.showError($t("common.fileDetail.tagsLoadError"));
      currentTags = [];
      originalTags = [];
    } finally {
      isLoadingTags = false;
    }
  }

  // タグの変更ハンドラー（自動保存付き）
  function handleTagsChange(tags: Tag[]) {
    console.log("TagInput change event:", tags);
    currentTags = tags;
    // 変更があった場合は自動保存
    if (hasTagsChanged()) {
      console.log("Tags changed, saving...");
      saveTagsChanges();
    } else {
      console.log("No changes detected");
    }
  }

  // タグが変更されているかチェック
  function hasTagsChanged(): boolean {
    if (currentTags.length !== originalTags.length) {
      console.log(
        "Tag count changed:",
        originalTags.length,
        "->",
        currentTags.length,
      );
      return true;
    }

    const currentIds = currentTags.map((t) => t.id).sort();
    const originalIds = originalTags.map((t) => t.id).sort();

    const hasChanges = currentIds.some(
      (id, index) => id !== originalIds[index],
    );
    console.log("Tag IDs comparison:", { currentIds, originalIds, hasChanges });
    return hasChanges;
  }

  // タグを保存する関数
  async function saveTagsChanges() {
    if (!file || !hasTagsChanged()) {
      console.log("No changes to save or no file selected");
      return;
    }

    console.log("Starting tag save process...");
    isSavingTags = true;
    try {
      // 全ての既存タグを取得
      console.log("Fetching all existing tags...");
      const allExistingTags = await tagsApi.getTags();
      console.log("All existing tags:", allExistingTags);

      // 新しいタグ（IDがtempで始まるもの）を処理
      const tagsToProcess = currentTags.filter((tag) =>
        tag.id.startsWith("temp_"),
      );
      console.log("Tags to process (new tags):", tagsToProcess);

      const processedTags: Tag[] = [];

      for (const tagToProcess of tagsToProcess) {
        console.log("Processing tag:", tagToProcess);
        // 既存タグから同じ名前のタグを検索
        const existingTag = allExistingTags.find(
          (existing) =>
            existing.name.toLowerCase() === tagToProcess.name.toLowerCase(),
        );

        if (existingTag) {
          console.log("Found existing tag:", existingTag);
          // 既存のタグが見つかった場合はそれを使用
          processedTags.push(existingTag);
        } else {
          console.log("Creating new tag:", tagToProcess.name);
          // 既存のタグが見つからない場合は新規作成
          try {
            const createdTag = await tagsApi.createTag(
              tagToProcess.name,
              tagToProcess.color,
            );
            console.log("Created new tag:", createdTag);
            processedTags.push(createdTag);
          } catch (error) {
            console.error($t("common.error.tagsSaveError"), error);
            errorStore.showError($t("common.fileDetail.tagsCreateError"));
            return; // エラーが発生した場合は処理を中断
          }
        }
      }

      // 既存のタグと処理されたタグのIDリストを作成
      const existingTagIds = currentTags
        .filter((tag) => !tag.id.startsWith("temp_"))
        .map((tag) => tag.id);
      const processedTagIds = processedTags.map((tag) => tag.id);
      const allTagIds = [...existingTagIds, ...processedTagIds];

      console.log("Tag IDs to save:", {
        existingTagIds,
        processedTagIds,
        allTagIds,
      });

      // ファイルのタグを更新
      console.log("Updating file tags...");
      await filesApi.updateFileTags(file.id, allTagIds);
      console.log("File tags updated successfully");

      // 状態を更新
      const updatedTags = [
        ...currentTags.filter((tag) => !tag.id.startsWith("temp_")),
        ...processedTags,
      ];
      currentTags = updatedTags;
      originalTags = [...updatedTags];

      console.log("Updated current and original tags:", updatedTags);

      // 保存完了の視覚的フィードバック
      showSavedIndicator = true;
      setTimeout(() => {
        showSavedIndicator = false;
      }, 2000); // 2秒後に非表示

      // 親コンポーネントにタグ更新を通知
      if (onTagsUpdated) {
        console.log("Notifying parent component of tag updates");
        onTagsUpdated();
      }
    } catch (error) {
      console.error($t("common.error.tagsSaveError"), error);
      errorStore.showError($t("common.fileDetail.tagsSaveError"));
    } finally {
      isSavingTags = false;
    }
  }

  // ファイルリネームモーダルを開く
  function handleRenameClick() {
    if (file && onOpenRenameModal) {
      onOpenRenameModal(file);
      onClose(); // ファイル詳細モーダルを閉じる
    }
  }

  // ファイルが変更されたときにタグを読み込む
  $effect(() => {
    if (file) {
      loadFileTags();
    }
  });
</script>

{#if file}
  <div class="modal-overlay" onclick={isDeleting ? undefined : onClose}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3>{$t("common.fileDetail.title")}</h3>
        <div class="modal-actions">
          <Dropdown disabled={isDeleting} position="left">
            {#snippet children()}
              <button
                class="dropdown-menu-item"
                onclick={() => onOpenFile(file.path)}
                role="menuitem"
              >
                <ExternalLink size={16} class="icon" />
                {$t("common.buttons.open")}
              </button>
              <button
                class="dropdown-menu-item"
                onclick={() => onRevealInFinder(file.path)}
                role="menuitem"
              >
                <Folder size={16} class="icon" />
                {$t("common.buttons.revealInFinder")}
              </button>
              {#if !file.is_directory}
                <button
                  class="dropdown-menu-item"
                  onclick={handleRenameClick}
                  disabled={isDeleting}
                  role="menuitem"
                >
                  <Edit3 size={16} class="icon" />
                  {$t("common.buttons.rename")}
                </button>
              {/if}
              <button
                class="dropdown-menu-item danger"
                onclick={() => onDeleteFile(file.path, file.name)}
                disabled={isDeleting}
                role="menuitem"
              >
                {#if isDeleting}
                  <Loader2 size={16} class="icon animate-spin" />
                  {$t("common.buttons.deleting")}
                {:else}
                  <Trash2 size={16} class="icon" />
                  {$t("common.buttons.delete")}
                {/if}
              </button>
            {/snippet}
          </Dropdown>
          <button class="close-button" onclick={onClose} disabled={isDeleting}>
            <X size={20} />
          </button>
        </div>
      </div>
      <div class="modal-body">
        <div class="file-detail-section">
          <h4>{$t("common.fileDetail.basicInfo")}</h4>
          <div class="detail-grid">
            <div class="detail-item">
              <span class="detail-label"
                >{$t("common.fileDetail.filename")}:</span
              >
              <span class="detail-value">{file.name}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">{$t("common.fileDetail.path")}:</span>
              <span class="detail-value">{file.path}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">{$t("common.fileDetail.size")}:</span>
              <span class="detail-value"
                >{formatFileSize(file.file_size || file.size)}</span
              >
            </div>
            <div class="detail-item">
              <span class="detail-label">{$t("common.fileDetail.type")}:</span>
              <span class="detail-value"
                >{file.is_directory
                  ? $t("common.fileDetail.directory")
                  : file.mime_type ||
                    file.file_type ||
                    $t("common.fileDetail.unknown")}</span
              >
            </div>
          </div>
        </div>


        <div class="file-detail-section">
          <h4>{$t("common.fileDetail.dateInfo")}</h4>
          <div class="detail-grid">
            <div class="detail-item">
              <span class="detail-label"
                >{$t("common.fileDetail.created")}:</span
              >
              <span class="detail-value">{formatDate(file.created_at)}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label"
                >{$t("common.fileDetail.modified")}:</span
              >
              <span class="detail-value">{formatDate(file.modified_at)}</span>
            </div>
            {#if file.birth_time}
              <div class="detail-item">
                <span class="detail-label"
                  >{$t("common.fileDetail.birthTime")}:</span
                >
                <span class="detail-value">{formatDate(file.birth_time)}</span>
              </div>
            {/if}
            {#if file.last_accessed}
              <div class="detail-item">
                <span class="detail-label"
                  >{$t("common.fileDetail.lastAccessed")}:</span
                >
                <span class="detail-value"
                  >{formatDate(file.last_accessed)}</span
                >
              </div>
            {/if}
          </div>
        </div>

        <div class="file-detail-section">
          <h4>{$t("common.fileDetail.systemInfo")}</h4>
          <div class="detail-grid">
            {#if file.permissions}
              <div class="detail-item">
                <span class="detail-label"
                  >{$t("common.fileDetail.permissions")}:</span
                >
                <span class="detail-value">{file.permissions}</span>
              </div>
            {/if}
            {#if file.owner_uid !== null}
              <div class="detail-item">
                <span class="detail-label"
                  >{$t("common.fileDetail.ownerUid")}:</span
                >
                <span class="detail-value">{file.owner_uid}</span>
              </div>
            {/if}
            {#if file.group_gid !== null}
              <div class="detail-item">
                <span class="detail-label"
                  >{$t("common.fileDetail.groupGid")}:</span
                >
                <span class="detail-value">{file.group_gid}</span>
              </div>
            {/if}
            {#if file.inode !== null}
              <div class="detail-item">
                <span class="detail-label"
                  >{$t("common.fileDetail.inode")}:</span
                >
                <span class="detail-value">{file.inode}</span>
              </div>
            {/if}
            {#if file.hard_links !== null}
              <div class="detail-item">
                <span class="detail-label"
                  >{$t("common.fileDetail.hardLinks")}:</span
                >
                <span class="detail-value">{file.hard_links}</span>
              </div>
            {/if}
            {#if file.device_id !== null}
              <div class="detail-item">
                <span class="detail-label"
                  >{$t("common.fileDetail.deviceId")}:</span
                >
                <span class="detail-value">{file.device_id}</span>
              </div>
            {/if}
          </div>
        </div>

        <!-- タグ管理 -->
        <div class="file-detail-section">
          <div class="tags-section-header">
            <h4>{$t("common.tags.title")}</h4>
            {#if isSavingTags}
              <div class="saving-indicator">
                <Loader2 size={14} class="animate-spin" />
                {$t("common.buttons.saving")}
              </div>
            {:else if showSavedIndicator}
              <div class="saved-indicator">
                ✓ {$t("common.buttons.saved")}
              </div>
            {/if}
          </div>

          {#if isLoadingTags}
            <div class="loading-tags">
              <Loader2 size={16} class="animate-spin" />
              {$t("common.fileDetail.loadingTags")}
            </div>
          {:else}
            <TagInput
              tags={currentTags}
              onchange={handleTagsChange}
              disabled={isSavingTags}
              placeholder={$t("common.fileDetail.tagsInputPlaceholder")}
            />
          {/if}
        </div>

        <!-- ファイルメタデータ (EXIF, 音声情報等) -->
        {#if file.metadata}
          <div class="file-detail-section">
            <h4>{$t("common.fileDetail.fileMetadata")}</h4>
            <div class="metadata-section">
              {#if (() => {
                try {
                  const parsed = JSON.parse(file.metadata);
                  return typeof parsed === "object" && parsed !== null;
                } catch {
                  return false;
                }
              })()}
                {#each Object.entries(JSON.parse(file.metadata)) as [category, data]}
                  <div class="metadata-category">
                    <h5>
                      {(() => {
                        switch (category) {
                          case "exif":
                            return $t("common.fileDetail.exifInfo");
                          case "audio":
                            return $t("common.fileDetail.audioInfo");
                          default:
                            return $t("common.fileDetail.categoryInfo", {
                              category: category.toUpperCase(),
                            });
                        }
                      })()}
                    </h5>

                    {#if category === "audio"}
                      <!-- 音声ファイル専用の表示 -->
                      <div class="detail-grid">
                        <!-- 基本情報 -->
                        {#if data.duration !== undefined}
                          <div class="detail-item">
                            <span class="detail-label"
                              >{$t("common.fileDetail.duration")}:</span
                            >
                            <span class="detail-value"
                              >{Math.floor(data.duration / 60)}:{(
                                data.duration % 60
                              )
                                .toString()
                                .padStart(2, "0")}</span
                            >
                          </div>
                        {/if}
                        {#if data.bitrate !== undefined && data.bitrate > 0}
                          <div class="detail-item">
                            <span class="detail-label"
                              >{$t("common.fileDetail.bitrate")}:</span
                            >
                            <span class="detail-value"
                              >{data.bitrate}
                              {$t("common.exif.units.kbps")}</span
                            >
                          </div>
                        {/if}
                        {#if data.sample_rate !== undefined && data.sample_rate > 0}
                          <div class="detail-item">
                            <span class="detail-label"
                              >{$t("common.fileDetail.sampleRate")}:</span
                            >
                            <span class="detail-value"
                              >{data.sample_rate}
                              {$t("common.exif.units.hz")}</span
                            >
                          </div>
                        {/if}
                        {#if data.channels !== undefined && data.channels > 0}
                          <div class="detail-item">
                            <span class="detail-label"
                              >{$t("common.fileDetail.channels")}:</span
                            >
                            <span class="detail-value"
                              >{data.channels === 1
                                ? $t("common.fileDetail.mono")
                                : data.channels === 2
                                  ? $t("common.fileDetail.stereo")
                                  : $t("common.fileDetail.channels_count", {
                                      count: data.channels,
                                    })}</span
                            >
                          </div>
                        {/if}

                        <!-- タグ情報 -->
                        {#if data.tags && typeof data.tags === "object"}
                          {#if data.tags.title}
                            <div class="detail-item">
                              <span class="detail-label"
                                >{$t("common.fileDetail.title")}:</span
                              >
                              <span class="detail-value">{data.tags.title}</span
                              >
                            </div>
                          {/if}
                          {#if data.tags.artist}
                            <div class="detail-item">
                              <span class="detail-label"
                                >{$t("common.fileDetail.artist")}:</span
                              >
                              <span class="detail-value"
                                >{data.tags.artist}</span
                              >
                            </div>
                          {/if}
                          {#if data.tags.album}
                            <div class="detail-item">
                              <span class="detail-label"
                                >{$t("common.fileDetail.album")}:</span
                              >
                              <span class="detail-value">{data.tags.album}</span
                              >
                            </div>
                          {/if}
                          {#if data.tags.year}
                            <div class="detail-item">
                              <span class="detail-label"
                                >{$t("common.fileDetail.year")}:</span
                              >
                              <span class="detail-value">{data.tags.year}</span>
                            </div>
                          {/if}
                          {#if data.tags.genre}
                            <div class="detail-item">
                              <span class="detail-label"
                                >{$t("common.fileDetail.genre")}:</span
                              >
                              <span class="detail-value">{data.tags.genre}</span
                              >
                            </div>
                          {/if}
                          {#if data.tags.track}
                            <div class="detail-item">
                              <span class="detail-label"
                                >{$t("common.fileDetail.track")}:</span
                              >
                              <span class="detail-value">{data.tags.track}</span
                              >
                            </div>
                          {/if}
                        {/if}
                      </div>
                    {:else}
                      <!-- その他のメタデータ（EXIF等）の表示 -->
                      <div class="detail-grid">
                        {#each Object.entries(data) as [key, value]}
                          {#snippet metadataItem()}
                            {#await Promise.all([(async () => {
                                // Tag(Exif, 数値)形式のキーを処理
                                if (key.startsWith("Tag(") && key.includes(",")) {
                                  const match = key.match(/Tag\((\w+),\s*(\d+)\)/);
                                  if (match) {
                                    const tagNumber = parseInt(match[2]);
                                    return await getExifTagName(tagNumber);
                                  }
                                }

                                // キー名を翻訳化
                                const keyTranslations: { [key: string]: string } = { Make: "make", Model: "model", DateTime: "dateTime", ExposureTime: "exposureTime", FNumber: "fNumber", ISOSpeedRatings: "isoSpeedRatings", FocalLength: "focalLength", Flash: "flash", WhiteBalance: "whiteBalance", ColorSpace: "colorSpace", ExifImageWidth: "exifImageWidth", ExifImageHeight: "exifImageHeight", Orientation: "orientation", MeteringMode: "meteringMode", ExposureMode: "exposureMode", ExposureProgram: "exposureProgram", SceneCaptureType: "sceneCaptureType", LightSource: "lightSource", FlashPixVersion: "flashPixVersion", ExifVersion: "exifVersion", ComponentsConfiguration: "componentsConfiguration", CompressedBitsPerPixel: "compressedBitsPerPixel", PixelXDimension: "pixelXDimension", PixelYDimension: "pixelYDimension", UserComment: "userComment", RelatedSoundFile: "relatedSoundFile", DateTimeOriginal: "dateTimeOriginal", DateTimeDigitized: "dateTimeDigitized", SubSecTime: "subSecTime", SubSecTimeOriginal: "subSecTimeOriginal", SubSecTimeDigitized: "subSecTimeDigitized", ImageDescription: "imageDescription", Software: "software", Artist: "artist", Copyright: "copyright", XResolution: "xResolution", YResolution: "yResolution", ResolutionUnit: "resolutionUnit", ImageWidth: "imageWidth", ImageLength: "imageLength", BitsPerSample: "bitsPerSample", Compression: "compression", PhotometricInterpretation: "photometricInterpretation", SamplesPerPixel: "samplesPerPixel", PlanarConfiguration: "planarConfiguration", TransferFunction: "transferFunction", WhitePoint: "whitePoint", PrimaryChromaticities: "primaryChromaticities", YCbCrCoefficients: "yCbCrCoefficients", YCbCrSubSampling: "yCbCrSubSampling", YCbCrPositioning: "yCbCrPositioning", ReferenceBlackWhite: "referenceBlackWhite" };
                                const translationKey = keyTranslations[key];
                                return translationKey ? $t(`exif.${translationKey}`) : key;
                              })(), interpretExifValue(key, value)])}
                              <div class="detail-item">
                                <span class="detail-label"
                                  >{$t("common.fileDetail.loading")}</span
                                >
                                <span class="detail-value"
                                  >{$t("common.fileDetail.loading")}</span
                                >
                              </div>
                            {:then [labelName, interpretedValue]}
                              <div class="detail-item">
                                <span class="detail-label">{labelName}:</span>
                                <span class="detail-value"
                                  >{interpretedValue}</span
                                >
                              </div>
                            {:catch error}
                              <div class="detail-item">
                                <span class="detail-label">{key}:</span>
                                <span class="detail-value"
                                  >{$t("common.fileDetail.errorMessage", {
                                    message: error.message,
                                  })}</span
                                >
                              </div>
                            {/await}
                          {/snippet}
                          {@render metadataItem()}
                        {/each}
                      </div>
                    {/if}
                  </div>
                {/each}
              {:else}
                <div class="error-message">
                  {$t("common.fileDetail.metadataParseError")}
                </div>
              {/if}
            </div>
          </div>
        {/if}

        <!-- カスタムメタデータ -->
        {#if customMetadataKeys.length > 0}
          <CustomMetadataEditor fileId={file.id} {customMetadataKeys} />
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
</style>

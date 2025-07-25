<script lang="ts">
  import { X } from "@lucide/svelte";
  import { onMount } from "svelte";
  import { getSettings, updateSettingBool, updateSettingInt, updateSettingString, getLanguageSetting } from "../../../api/settings";
  import { errorStore } from "../../../stores/error";
  import { t, locale } from "$lib/i18n";
  import ExclusionPatternsManager from "./ExclusionPatternsManager.svelte";
  import Checkbox from "../../parts/Checkbox.svelte";
  import Select from "../../parts/Select.svelte";

  export let isOpen = false;
  export let onClose: () => void;
  export let onSettingsChanged: (() => void) | undefined = undefined;

  // 設定の状態管理
  let darkMode = false;
  let detailViewDefault = false;
  let filesPerPage = 20;
  let showHiddenFiles = false;
  let showDirectories = true;
  let useFuzzySearch = true;
  let highlightSearchResults = true;
  let language = 'en';
  let isLoading = false;

  // 設定を読み込み
  onMount(async () => {
    try {
      const [settings, lang] = await Promise.all([
        getSettings(),
        getLanguageSetting()
      ]);
      showHiddenFiles = settings.show_hidden_files;
      filesPerPage = settings.files_per_page;
      language = lang;
    } catch (error) {
      console.error('設定の読み込みに失敗しました:', error);
      errorStore.showError($t("common.error.settingsLoadFailed"));
    }
  });

  // モーダルが開かれるたびに設定を再読み込み
  $: if (isOpen) {
    loadSettings();
  }

  const loadSettings = async () => {
    try {
      const [settings, lang] = await Promise.all([
        getSettings(),
        getLanguageSetting()
      ]);
      showHiddenFiles = settings.show_hidden_files;
      filesPerPage = settings.files_per_page;
      language = lang;
    } catch (error) {
      console.error('設定の読み込みに失敗しました:', error);
      errorStore.showError($t("common.error.settingsLoadFailed"));
    }
  };

  const handleClose = () => {
    onClose();
  };

  const handleOverlayClick = (event: MouseEvent) => {
    if (event.target === event.currentTarget) {
      handleClose();
    }
  };

  const handleSave = async () => {
    isLoading = true;
    try {
      await updateSettingBool('show_hidden_files', showHiddenFiles);
      await updateSettingBool('show_directories', showDirectories);
      await updateSettingInt('files_per_page', filesPerPage);
      await updateSettingString('language', language);
      
      // i18nライブラリのlocaleを即座に更新
      $locale = language;
      
      console.log("Settings saved:", {
        darkMode,
        detailViewDefault,
        filesPerPage,
        showHiddenFiles,
        showDirectories,
        useFuzzySearch,
        highlightSearchResults,
        language,
      });
      
      errorStore.showSuccess($t("common.settings.saved"));
      
      // 設定が変更されたことを通知
      if (onSettingsChanged) {
        onSettingsChanged();
      }
      
      handleClose();
    } catch (error) {
      console.error('設定の保存に失敗しました:', error);
      errorStore.showError($t("common.error.settingsSaveFailed"));
    } finally {
      isLoading = false;
    }
  };
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_interactive_supports_focus -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="modal-overlay"
    onclick={handleOverlayClick}
    role="dialog"
    aria-modal="true"
    aria-labelledby="settings-title"
    tabindex="-1"
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3 id="settings-title">{$t("common.settings.title")}</h3>
        <button
          class="close-button"
          onclick={handleClose}
          aria-label={$t("common.buttons.close")}
        >
          <X size={20} />
        </button>
      </div>
      <div class="modal-body">
        <div class="settings-section">
          <h4>{$t("common.settings.language")}</h4>
          <div class="setting-item">
            <Select
              label={$t("common.settings.language")}
              options={[
                { value: "ja", label: "日本語" },
                { value: "en", label: "English" }
              ]}
              bind:value={language}
              className="setting-select"
            />
          </div>
        </div>

        <div class="settings-section">
          <h4>{$t("common.settings.fileManagement")}</h4>
          <div class="setting-item">
            <Select
              label={$t("common.pagination.itemsPerPage")}
              options={[
                { value: 10, label: "10" },
                { value: 20, label: "20" },
                { value: 50, label: "50" },
                { value: 100, label: "100" },
                { value: 200, label: "200" }
              ]}
              bind:value={filesPerPage}
              className="setting-select"
            />
          </div>
          <div class="setting-item">
            <Checkbox
              bind:checked={showHiddenFiles}
              label={$t("common.settings.showHiddenFiles")}
            />
          </div>
          <div class="setting-item">
            <Checkbox
              bind:checked={showDirectories}
              label="ディレクトリを表示"
            />
          </div>
        </div>

        <div class="settings-section">
          <ExclusionPatternsManager />
        </div>

        <div class="settings-actions">
          <button class="save-button" onclick={handleSave} disabled={isLoading}>
            {isLoading ? $t("common.buttons.saving") : $t("common.buttons.save")}
          </button>
          <button class="cancel-button" onclick={handleClose} disabled={isLoading}>
            {$t("common.buttons.cancel")}
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  /* 設定モーダル固有のスタイル */
  .settings-section {
    margin-bottom: 2rem;
  }

  .settings-section:last-child {
    margin-bottom: 0;
  }

  .settings-section h4 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: #374151;
    border-bottom: 1px solid #e5e7eb;
    padding-bottom: 0.5rem;
  }

  .setting-item {
    margin-bottom: 0.75rem;
  }

  .setting-item:last-child {
    margin-bottom: 0;
  }


  .settings-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 2rem;
    padding-top: 1.5rem;
    border-top: 1px solid #e5e7eb;
  }

  .save-button,
  .cancel-button {
    padding: 0.75rem 1.5rem;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    border: 1px solid;
  }

  .save-button {
    background-color: #3b82f6;
    color: white;
    border-color: #3b82f6;
  }

  .save-button:hover:not(:disabled) {
    background-color: #2563eb;
    border-color: #2563eb;
  }

  .save-button:disabled {
    background-color: #9ca3af;
    border-color: #9ca3af;
    cursor: not-allowed;
    opacity: 0.6;
  }

  .cancel-button {
    background-color: white;
    color: #374151;
    border-color: #d1d5db;
  }

  .cancel-button:hover:not(:disabled) {
    background-color: #f9fafb;
    border-color: #9ca3af;
  }

  .cancel-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>

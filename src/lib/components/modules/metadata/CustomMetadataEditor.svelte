<script lang="ts">
  import {
    getCustomMetadataValuesByFile,
    setCustomMetadataValue,
  } from "../../../api/metadata";
  import type {
    CustomMetadataKey,
    CustomMetadataValue,
    CustomMetadataDataType,
    SetCustomMetadataValueRequest,
  } from "../../../types";
  import { errorStore } from "../../../stores/error";
  import { t } from "$lib/i18n";
  import Select from "../../parts/Select.svelte";

  interface Props {
    fileId: string;
    customMetadataKeys: CustomMetadataKey[];
  }

  let { fileId, customMetadataKeys }: Props = $props();

  // 状態管理
  let metadataValues: CustomMetadataValue[] = $state([]);
  let isLoading = $state(false);
  let savingKeyId: string | null = $state(null);
  let error = $state("");

  // 初期データ読み込み
  const loadMetadataValues = async () => {
    if (!fileId) return;

    isLoading = true;
    try {
      metadataValues = await getCustomMetadataValuesByFile(fileId);
    } catch (e) {
      console.error("カスタムメタデータ値の読み込みに失敗:", e);
      error = $t("common.error.metadataLoadFailed");
      errorStore.showError($t("common.error.customMetadataLoadFailed"));
    } finally {
      isLoading = false;
    }
  };

  // ファイルIDが変更されたら再読み込み
  $effect(() => {
    if (fileId) {
      loadMetadataValues();
    }
  });

  // キーに対応する値を取得
  const getValueForKey = (keyId: string): CustomMetadataValue | undefined => {
    return metadataValues.find((v) => v.key_id === keyId);
  };

  // 表示用の値を取得
  const getDisplayValue = (key: CustomMetadataKey): string => {
    const currentValue = getValueForKey(key.id);
    return currentValue?.value || key.default_value || "";
  };

  // 値のバリデーション
  const validateValue = (key: CustomMetadataKey, value: string): boolean => {
    // 必須チェック
    if (key.is_required && !value.trim()) {
      error = $t("common.error.requiredField", { field: key.display_name } as any);
      return false;
    }

    // データ型チェック
    if (value.trim()) {
      switch (key.data_type) {
        case "number":
          if (isNaN(Number(value))) {
            error = $t("common.error.invalidNumber", { field: key.display_name } as any);
            return false;
          }
          break;
        case "date":
          if (isNaN(Date.parse(value))) {
            error = $t("common.error.invalidDate", { field: key.display_name } as any);
            return false;
          }
          break;
        case "boolean":
          const lowerValue = value.toLowerCase();
          if (!["true", "false", "1", "0", "yes", "no"].includes(lowerValue)) {
            error = $t("common.error.invalidBoolean", { field: key.display_name } as any);
            return false;
          }
          break;
        case "json":
          try {
            JSON.parse(value);
          } catch {
            error = $t("common.error.invalidJson", { field: key.display_name } as any);
            return false;
          }
          break;
      }
    }

    // 正規表現バリデーション
    if (key.validation_pattern && value.trim()) {
      try {
        const regex = new RegExp(key.validation_pattern);
        if (!regex.test(value)) {
          error = $t("common.error.invalidFormat", { field: key.display_name } as any);
          return false;
        }
      } catch {
        console.warn(`無効な正規表現パターン: ${key.validation_pattern}`);
        errorStore.showWarning($t("common.error.invalidValidationPattern"));
      }
    }

    return true;
  };

  // 値を自動保存
  const saveValue = async (key: CustomMetadataKey, value: string) => {
    if (!validateValue(key, value)) {
      return false;
    }

    savingKeyId = key.id;
    error = "";

    try {
      const request: SetCustomMetadataValueRequest = {
        file_id: fileId,
        key_id: key.id,
        value: value.trim() || null,
      };

      await setCustomMetadataValue(request);

      // フォーカスを保持するため、loadMetadataValues()を呼ばず直接値を更新
      const existingValueIndex = metadataValues.findIndex(
        (v) => v.key_id === key.id,
      );
      if (existingValueIndex !== -1) {
        // 既存の値を更新
        metadataValues[existingValueIndex] = {
          ...metadataValues[existingValueIndex],
          value: value.trim() || null,
          updated_at: new Date().toISOString(),
        };
      } else {
        // 新しい値を追加
        const newValue: CustomMetadataValue = {
          id: crypto.randomUUID(),
          file_id: fileId,
          key_id: key.id,
          value: value.trim() || null,
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        };
        metadataValues.push(newValue);
      }

      return true;
    } catch (e) {
      console.error("値の保存に失敗:", e);
      error = typeof e === "string" ? e : $t("common.error.saveFailed");
      errorStore.showError($t("common.error.metadataSaveFailed"));
      return false;
    } finally {
      savingKeyId = null;
    }
  };

  // 値変更時の処理（デバウンス）
  const handleValueChange = async (key: CustomMetadataKey, value: string) => {
    // デバウンス用のタイマーを管理
    if (changeTimers[key.id]) {
      clearTimeout(changeTimers[key.id]);
    }

    changeTimers[key.id] = setTimeout(async () => {
      await saveValue(key, value);
      delete changeTimers[key.id];
    }, 1000); // 1秒後に自動保存
  };

  // デバウンス用のタイマー管理
  let changeTimers: Record<string, NodeJS.Timeout> = {};

  // データ型に応じた表示形式
  const formatValue = (
    value: string,
    dataType: CustomMetadataDataType,
  ): string => {
    if (!value) return "";

    switch (dataType) {
      case "boolean":
        const lowerValue = value.toLowerCase();
        if (["true", "1", "yes"].includes(lowerValue)) return $t("common.buttons.yes");
        if (["false", "0", "no"].includes(lowerValue)) return $t("common.buttons.no");
        return value;
      case "date":
        try {
          const date = new Date(value);
          return date.toLocaleString("ja-JP");
        } catch {
          return value;
        }
      case "json":
        try {
          return JSON.stringify(JSON.parse(value), null, 2);
        } catch {
          return value;
        }
      case "number":
        const num = Number(value);
        return isNaN(num) ? value : num.toLocaleString();
      default:
        return value;
    }
  };

  // データ型に応じた入力要素
  const getInputType = (dataType: CustomMetadataDataType): string => {
    switch (dataType) {
      case "number":
        return "number";
      case "date":
        return "datetime-local";
      default:
        return "text";
    }
  };

  // 入力要素のプレースホルダー
  const getPlaceholder = (key: CustomMetadataKey): string => {
    switch (key.data_type) {
      case "boolean":
        return $t("common.placeholder.boolean");
      case "date":
        return $t("common.placeholder.date");
      case "json":
        return $t("common.placeholder.json");
      case "number":
        return $t("common.placeholder.number");
      default:
        return $t("common.placeholder.input", { field: key.display_name } as any);
    }
  };
</script>

<div class="file-detail-section">
  <h4>{$t("common.metadata.customMetadata")}</h4>

  {#if isLoading}
    <div class="loading">{$t("common.loading.loading")}</div>
  {:else if customMetadataKeys.length === 0}
    <div class="empty-state">
      <p>{$t("common.metadata.noCustomKeys")}</p>
      <p>{$t("common.metadata.createKeyInstruction")}</p>
    </div>
  {:else}
    <div class="detail-grid">
      {#each customMetadataKeys as key (key.id)}
        {@const currentValue = getValueForKey(key.id)}
        {@const displayValue = getDisplayValue(key)}
        {@const isSaving = savingKeyId === key.id}

        <div class="detail-item">
          <span class="detail-label">
            <span
              class="label-text"
              title={key.description || key.display_name}
            >
              {key.display_name}:
            </span>
            {#if key.is_required}
              <span class="required">*</span>
            {/if}
            {#if isSaving}
              <span class="saving-indicator">{$t("common.buttons.saving")}</span>
            {/if}
          </span>
          <span class="detail-value">
            {#if key.data_type === "boolean"}
              <Select
                options={[
                  { value: "", label: $t("common.buttons.selectValue") },
                  { value: "true", label: $t("common.buttons.yes") },
                  { value: "false", label: $t("common.buttons.no") }
                ]}
                value={displayValue}
                on:change={(e) => handleValueChange(key, e.target.value)}
                className="inline-select"
              />
            {:else if key.data_type === "json"}
              <textarea
                value={displayValue}
                oninput={(e) =>
                  handleValueChange(
                    key,
                    (e.target as HTMLTextAreaElement).value,
                  )}
                placeholder={getPlaceholder(key)}
                rows="3"
                class="inline-textarea"
              ></textarea>
            {:else}
              <input
                type={getInputType(key.data_type)}
                value={displayValue}
                oninput={(e) =>
                  handleValueChange(key, (e.target as HTMLInputElement).value)}
                placeholder={getPlaceholder(key)}
                class="inline-input"
              />
            {/if}
            {#if error && savingKeyId === key.id}
              <div class="error-message">{error}</div>
            {/if}
          </span>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .file-detail-section {
    margin-top: 20px;
  }

  .file-detail-section h4 {
    margin: 0 0 12px 0;
    color: #333;
    font-size: 16px;
  }

  .detail-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 12px;
  }

  .detail-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  @media (min-width: 600px) {
    .detail-item {
      flex-direction: row;
      align-items: flex-start;
      gap: 12px;
    }

    .detail-label {
      min-width: 140px;
      flex-shrink: 0;
    }

    .detail-value {
      flex: 1;
    }
  }

  .detail-label {
    font-weight: 500;
    color: #666;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .required {
    color: #dc3545;
    font-size: 12px;
  }

  .saving-indicator {
    color: #007acc;
    font-size: 11px;
    font-weight: normal;
    font-style: italic;
  }

  .detail-value {
    color: #333;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .inline-input,
  .inline-textarea {
    border: 1px solid #ddd;
    border-radius: 4px;
    padding: 6px 8px;
    font-size: 14px;
    width: 100%;
    transition:
      border-color 0.2s,
      box-shadow 0.2s;
  }

  .inline-input:focus,
  .inline-textarea:focus {
    outline: none;
    border-color: #007acc;
    box-shadow: 0 0 0 2px rgba(0, 122, 204, 0.2);
  }

  .inline-textarea {
    resize: vertical;
    font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
    min-height: 60px;
  }


  .label-text {
    cursor: help;
    text-decoration: underline;
    text-decoration-style: dotted;
    text-decoration-color: #ccc;
  }

  .label-text:hover {
    text-decoration-color: #007acc;
  }

  .error-message {
    background: #f8d7da;
    color: #721c24;
    padding: 6px 8px;
    border-radius: 4px;
    font-size: 12px;
    border: 1px solid #f5c6cb;
    margin-top: 4px;
  }

  .loading {
    text-align: center;
    padding: 20px;
    color: #666;
  }

  .empty-state {
    text-align: center;
    padding: 20px;
    color: #666;
    background: #f8f9fa;
    border-radius: 8px;
  }

  .empty-state p {
    margin: 4px 0;
  }
</style>

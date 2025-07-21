<script lang="ts">
  import { confirm } from "@tauri-apps/plugin-dialog";
  import { Plus, Edit, Trash2, Save, X } from "@lucide/svelte";
  import {
    createCustomMetadataKey,
    updateCustomMetadataKey,
    deleteCustomMetadataKey,
  } from "../../../api/metadata";
  import type {
    CustomMetadataKey,
    CustomMetadataDataType,
    CreateCustomMetadataKeyRequest,
    UpdateCustomMetadataKeyRequest,
  } from "../../../types";
  import { errorStore } from "../../../stores/error";
  import { t } from "$lib/i18n";
  import IconButton from "../../parts/IconButton.svelte";
  import Button from "$lib/components/parts/Button.svelte";
  import TextInput from "../../parts/TextInput.svelte";
  import Checkbox from "../../parts/Checkbox.svelte";
  import Select from "../../parts/Select.svelte";

  interface Props {
    keys: CustomMetadataKey[];
    onKeysUpdated: () => void;
  }

  let { keys = $bindable(), onKeysUpdated }: Props = $props();

  // フォーム状態
  let showCreateForm = $state(false);
  let editingKeyId: string | null = $state(null);
  let formData = $state({
    name: "",
    display_name: "",
    data_type: "text" as CustomMetadataDataType,
    description: "",
    is_required: false,
    default_value: "",
    validation_pattern: "",
  });

  // エラー状態
  let error = $state("");
  let isSubmitting = $state(false);

  // データ型の選択肢
  const dataTypes = $derived([
    {
      value: "text" as CustomMetadataDataType,
      label: $t("common.metadata.dataTypes.text"),
      description: $t("common.metadata.dataTypeDescriptions.text"),
    },
    {
      value: "number" as CustomMetadataDataType,
      label: $t("common.metadata.dataTypes.number"),
      description: $t("common.metadata.dataTypeDescriptions.number"),
    },
    {
      value: "date" as CustomMetadataDataType,
      label: $t("common.metadata.dataTypes.date"),
      description: $t("common.metadata.dataTypeDescriptions.date"),
    },
    {
      value: "boolean" as CustomMetadataDataType,
      label: $t("common.metadata.dataTypes.boolean"),
      description: $t("common.metadata.dataTypeDescriptions.boolean"),
    },
    {
      value: "json" as CustomMetadataDataType,
      label: $t("common.metadata.dataTypes.json"),
      description: $t("common.metadata.dataTypeDescriptions.json"),
    },
  ]);

  // フォームリセット
  const resetForm = () => {
    formData = {
      name: "",
      display_name: "",
      data_type: "text",
      description: "",
      is_required: false,
      default_value: "",
      validation_pattern: "",
    };
    error = "";
  };

  // 新規作成フォームを表示
  const startCreate = () => {
    resetForm();
    showCreateForm = true;
    editingKeyId = null;
  };

  // 編集フォームを表示
  const startEdit = (key: CustomMetadataKey) => {
    formData = {
      name: key.name,
      display_name: key.display_name,
      data_type: key.data_type,
      description: key.description || "",
      is_required: key.is_required,
      default_value: key.default_value || "",
      validation_pattern: key.validation_pattern || "",
    };
    editingKeyId = key.id;
    showCreateForm = true;
    error = "";
  };

  // フォームキャンセル
  const cancelForm = () => {
    showCreateForm = false;
    editingKeyId = null;
    resetForm();
  };

  // バリデーション
  const validateForm = (): boolean => {
    if (!formData.name.trim()) {
      error = $t("common.error.nameRequired");
      return false;
    }
    if (!formData.display_name.trim()) {
      error = $t("common.error.displayNameRequired");
      return false;
    }
    if (!/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(formData.name)) {
      error = $t("common.error.invalidNameFormat");
      return false;
    }
    return true;
  };

  // カスタムメタデータキーを作成
  const createKey = async () => {
    if (!validateForm()) return;

    isSubmitting = true;
    try {
      const request: CreateCustomMetadataKeyRequest = {
        name: formData.name.trim(),
        display_name: formData.display_name.trim(),
        data_type: formData.data_type,
        description: formData.description.trim() || null,
        is_required: formData.is_required,
        default_value: formData.default_value.trim() || null,
        validation_pattern: formData.validation_pattern.trim() || null,
      };

      await createCustomMetadataKey(request);
      onKeysUpdated();
      cancelForm();
    } catch (e) {
      error = typeof e === "string" ? e : $t("common.error.createKeyFailed");
    } finally {
      isSubmitting = false;
    }
  };

  // カスタムメタデータキーを更新
  const updateKey = async () => {
    if (!validateForm() || !editingKeyId) return;

    isSubmitting = true;
    try {
      const request: UpdateCustomMetadataKeyRequest = {
        id: editingKeyId,
        display_name: formData.display_name.trim(),
        data_type: formData.data_type,
        description: formData.description.trim() || null,
        is_required: formData.is_required,
        default_value: formData.default_value.trim() || null,
        validation_pattern: formData.validation_pattern.trim() || null,
      };

      await updateCustomMetadataKey(request);
      onKeysUpdated();
      cancelForm();
    } catch (e) {
      error = typeof e === "string" ? e : $t("common.error.updateKeyFailed");
    } finally {
      isSubmitting = false;
    }
  };

  // カスタムメタデータキーを削除
  const deleteKey = async (keyId: string, keyName: string) => {
    try {
      const confirmed = await confirm(
        $t("common.dialog.confirmDeleteKey", { name: keyName } as any),
        {
          title: $t("common.dialog.confirm"),
          kind: "warning",
        },
      );

      if (!confirmed) {
        return;
      }

      await deleteCustomMetadataKey(keyId);
      onKeysUpdated();
    } catch (e) {
      console.error("削除処理でエラーが発生しました:", e);
      errorStore.showError(
        typeof e === "string" ? e : $t("common.error.deleteKeyFailed"),
      );
    }
  };

  // フォーム送信
  const handleSubmit = async (event: Event) => {
    event.preventDefault();
    if (editingKeyId) {
      await updateKey();
    } else {
      await createKey();
    }
  };

  // データ型の説明を取得
  const getDataTypeInfo = (dataType: CustomMetadataDataType) => {
    return dataTypes.find((dt) => dt.value === dataType);
  };
</script>

<div class="custom-metadata-manager">
  <div class="header">
    <h3>{$t("common.metadata.keyManagement")}</h3>
    <Button
      class="create-button"
      onclick={startCreate}
      iconName="Plus"
      text={$t("common.buttons.create")}
    />
  </div>

  <!-- 作成・編集フォーム -->
  {#if showCreateForm}
    <div class="form-section">
      <h4>
        {editingKeyId
          ? $t("common.metadata.editKey")
          : $t("common.metadata.createNewKey")}
      </h4>

      {#if error}
        <div class="error-message">{error}</div>
      {/if}

      <form onsubmit={handleSubmit}>
        <div class="form-grid">
          <div class="form-group">
            <label for="name">{$t("common.metadata.name")} *</label>
            <TextInput
              id="name"
              bind:value={formData.name}
              placeholder={$t("common.placeholder.keyName")}
              disabled={editingKeyId !== null || isSubmitting}
              required
            />
            <small>{$t("common.metadata.nameRule")}</small>
          </div>

          <div class="form-group">
            <label for="display_name"
              >{$t("common.metadata.displayName")} *</label
            >
            <TextInput
              id="display_name"
              bind:value={formData.display_name}
              placeholder={$t("common.placeholder.displayName")}
              disabled={isSubmitting}
              required
            />
          </div>

          <div class="form-group">
            <Select
              label="{$t('common.metadata.dataType')} *"
              options={dataTypes.map(dt => ({
                value: dt.value,
                label: `${dt.label} - ${dt.description}`
              }))}
              bind:value={formData.data_type}
              disabled={isSubmitting}
              id="data_type"
            />
          </div>

          <div class="form-group">
            <label for="description">{$t("common.metadata.description")}</label>
            <textarea
              id="description"
              bind:value={formData.description}
              placeholder={$t("common.placeholder.description")}
              disabled={isSubmitting}
              rows="3"
            ></textarea>
          </div>

          <div class="form-group">
            <label for="default_value"
              >{$t("common.metadata.defaultValue")}</label
            >
            <TextInput
              id="default_value"
              bind:value={formData.default_value}
              placeholder={$t("common.placeholder.defaultValue")}
              disabled={isSubmitting}
            />
          </div>

          <div class="form-group">
            <label for="validation_pattern"
              >{$t("common.metadata.validationPattern")}</label
            >
            <TextInput
              id="validation_pattern"
              bind:value={formData.validation_pattern}
              placeholder={$t("common.placeholder.validationPattern")}
              disabled={isSubmitting}
            />
            <small>{$t("common.metadata.validationDescription")}</small>
          </div>

          <div class="form-group checkbox-group">
            <Checkbox
              bind:checked={formData.is_required}
              disabled={isSubmitting}
              label={$t("common.metadata.required")}
            />
            <small>{$t("common.metadata.requiredDescription")}</small>
          </div>
        </div>

        <div class="form-actions">
          <button type="submit" class="save-button" disabled={isSubmitting}>
            <Save size={16} />
            {isSubmitting
              ? $t("common.buttons.saving")
              : editingKeyId
                ? $t("common.buttons.update")
                : $t("common.buttons.create")}
          </button>
          <button
            type="button"
            class="cancel-button"
            onclick={cancelForm}
            disabled={isSubmitting}
          >
            <X size={16} />
            {$t("common.buttons.cancel")}
          </button>
        </div>
      </form>
    </div>
  {/if}

  <!-- キー一覧 -->
  <div class="keys-list">
    {#if keys.length === 0}
      <div class="empty-state">
        <p>{$t("common.metadata.noKeysConfigured")}</p>
        <p>{$t("common.metadata.createFirstKey")}</p>
      </div>
    {:else}
      <div class="keys-grid">
        {#each keys as key (key.id)}
          <div class="key-card">
            <div class="key-header">
              <div class="key-info">
                <h4>{key.display_name}</h4>
                <code>{key.name}</code>
              </div>
              <div class="key-actions">
                <IconButton
                  icon={Edit}
                  title={$t("common.buttons.edit")}
                  onClick={() => startEdit(key)}
                  class="green"
                />
                <IconButton
                  icon={Trash2}
                  title={$t("common.buttons.delete")}
                  onClick={() => deleteKey(key.id, key.display_name)}
                  class="red"
                />
              </div>
            </div>

            <div class="key-details">
              <div class="detail-row">
                <span class="label">{$t("common.metadata.dataType")}:</span>
                <span class="value">
                  {getDataTypeInfo(key.data_type)?.label || key.data_type}
                  {key.is_required
                    ? ` (${$t("common.metadata.required")})`
                    : ""}
                </span>
              </div>

              {#if key.description}
                <div class="detail-row">
                  <span class="label">{$t("common.metadata.description")}:</span
                  >
                  <span class="value">{key.description}</span>
                </div>
              {/if}

              {#if key.default_value}
                <div class="detail-row">
                  <span class="label"
                    >{$t("common.metadata.defaultValue")}:</span
                  >
                  <span class="value">{key.default_value}</span>
                </div>
              {/if}

              {#if key.validation_pattern}
                <div class="detail-row">
                  <span class="label">{$t("common.metadata.validation")}:</span>
                  <code class="value">{key.validation_pattern}</code>
                </div>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .custom-metadata-manager {
    padding: 20px;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  .header h3 {
    margin: 0;
    color: #333;
  }


  .form-section {
    background: #f8f9fa;
    border: 1px solid #e9ecef;
    border-radius: 8px;
    padding: 20px;
    margin-bottom: 20px;
  }

  .form-section h4 {
    margin: 0 0 16px 0;
    color: #333;
  }

  .error-message {
    background: #f8d7da;
    color: #721c24;
    padding: 12px;
    border-radius: 4px;
    margin-bottom: 16px;
    border: 1px solid #f5c6cb;
  }

  .form-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    margin-bottom: 20px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
  }

  .form-group.checkbox-group {
    grid-column: 1 / -1;
  }


  .form-group label {
    margin-bottom: 4px;
    font-weight: 500;
    color: #333;
  }

  .form-group textarea {
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 14px;
  }

  .form-group textarea:focus {
    outline: none;
    border-color: #007acc;
    box-shadow: 0 0 0 2px rgba(0, 122, 204, 0.2);
  }

  .form-group small {
    margin-top: 4px;
    color: #666;
    font-size: 12px;
  }

  .form-actions {
    display: flex;
    gap: 12px;
  }

  .save-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: #28a745;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .save-button:hover:not(:disabled) {
    background: #218838;
  }

  .save-button:disabled {
    background: #6c757d;
    cursor: not-allowed;
  }

  .cancel-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: #6c757d;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .cancel-button:hover:not(:disabled) {
    background: #5a6268;
  }

  .empty-state {
    text-align: center;
    padding: 40px 20px;
    color: #666;
  }

  .empty-state p {
    margin: 8px 0;
  }

  .keys-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
    gap: 16px;
  }

  .key-card {
    background: white;
    border: 1px solid #e9ecef;
    border-radius: 8px;
    padding: 16px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .key-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 12px;
  }

  .key-info h4 {
    margin: 0 0 4px 0;
    color: #333;
  }

  .key-info code {
    background: #f8f9fa;
    padding: 2px 6px;
    border-radius: 3px;
    font-size: 12px;
    color: #e83e8c;
  }

  .key-actions {
    display: flex;
    gap: 4px;
  }

  .key-details {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .detail-row {
    display: flex;
    gap: 8px;
  }

  .detail-row .label {
    font-weight: 500;
    color: #666;
    min-width: 80px;
  }

  .detail-row .value {
    color: #333;
    flex: 1;
  }

  .detail-row code.value {
    background: #f8f9fa;
    padding: 2px 6px;
    border-radius: 3px;
    font-size: 12px;
    color: #e83e8c;
  }
</style>

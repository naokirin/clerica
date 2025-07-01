<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { confirm } from "@tauri-apps/plugin-dialog";
  import { Plus, Edit, Trash2, Save, X } from "lucide-svelte";
  import type { 
    CustomMetadataKey, 
    CustomMetadataDataType, 
    CreateCustomMetadataKeyRequest, 
    UpdateCustomMetadataKeyRequest 
  } from "../types.js";

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
    validation_pattern: ""
  });

  // エラー状態
  let error = $state("");
  let isSubmitting = $state(false);

  // データ型の選択肢
  const dataTypes: { value: CustomMetadataDataType; label: string; description: string }[] = [
    { value: "text", label: "テキスト", description: "文字列値" },
    { value: "number", label: "数値", description: "整数・小数点数" },
    { value: "date", label: "日付", description: "日付・時刻" },
    { value: "boolean", label: "真偽値", description: "true/false" },
    { value: "json", label: "JSON", description: "構造化データ" }
  ];

  // フォームリセット
  const resetForm = () => {
    formData = {
      name: "",
      display_name: "",
      data_type: "text",
      description: "",
      is_required: false,
      default_value: "",
      validation_pattern: ""
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
      validation_pattern: key.validation_pattern || ""
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
      error = "名前は必須です";
      return false;
    }
    if (!formData.display_name.trim()) {
      error = "表示名は必須です";
      return false;
    }
    if (!/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(formData.name)) {
      error = "名前は英数字とアンダースコアのみ使用可能で、数字から始められません";
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
        validation_pattern: formData.validation_pattern.trim() || null
      };

      await invoke("create_custom_metadata_key", { request });
      onKeysUpdated();
      cancelForm();
    } catch (e) {
      error = typeof e === "string" ? e : "カスタムメタデータキーの作成に失敗しました";
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
        validation_pattern: formData.validation_pattern.trim() || null
      };

      await invoke("update_custom_metadata_key", { request });
      onKeysUpdated();
      cancelForm();
    } catch (e) {
      error = typeof e === "string" ? e : "カスタムメタデータキーの更新に失敗しました";
    } finally {
      isSubmitting = false;
    }
  };

  // カスタムメタデータキーを削除
  const deleteKey = async (keyId: string, keyName: string) => {
    try {
      const confirmed = await confirm(
        `カスタムメタデータキー「${keyName}」を削除しますか？\n\n関連するすべての値も削除されます。\nこの操作は取り消すことができません。`,
        { 
          title: '確認', 
          kind: 'warning' 
        }
      );
      
      if (!confirmed) {
        return;
      }

      await invoke("delete_custom_metadata_key", { keyId });
      onKeysUpdated();
    } catch (e) {
      console.error("削除処理でエラーが発生しました:", e);
      // Tauri API のエラーの場合もalertで表示
      alert(typeof e === "string" ? e : "カスタムメタデータキーの削除に失敗しました");
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
    return dataTypes.find(dt => dt.value === dataType);
  };
</script>

<div class="custom-metadata-manager">
  <div class="header">
    <h3>カスタムメタデータキー管理</h3>
    <button class="create-button" onclick={startCreate}>
      <Plus size={16} />
      新規作成
    </button>
  </div>

  <!-- 作成・編集フォーム -->
  {#if showCreateForm}
    <div class="form-section">
      <h4>{editingKeyId ? "キーを編集" : "新しいキーを作成"}</h4>
      
      {#if error}
        <div class="error-message">{error}</div>
      {/if}

      <form onsubmit={handleSubmit}>
        <div class="form-grid">
          <div class="form-group">
            <label for="name">名前 *</label>
            <input
              id="name"
              type="text"
              bind:value={formData.name}
              placeholder="例: priority, category, project_name"
              disabled={editingKeyId !== null || isSubmitting}
              required
            />
            <small>英数字とアンダースコアのみ。数字から始められません。</small>
          </div>

          <div class="form-group">
            <label for="display_name">表示名 *</label>
            <input
              id="display_name"
              type="text"
              bind:value={formData.display_name}
              placeholder="例: 優先度, カテゴリ, プロジェクト名"
              disabled={isSubmitting}
              required
            />
          </div>

          <div class="form-group">
            <label for="data_type">データ型 *</label>
            <select id="data_type" bind:value={formData.data_type} disabled={isSubmitting}>
              {#each dataTypes as dataType}
                <option value={dataType.value}>
                  {dataType.label} - {dataType.description}
                </option>
              {/each}
            </select>
          </div>

          <div class="form-group">
            <label for="description">説明</label>
            <textarea
              id="description"
              bind:value={formData.description}
              placeholder="このカスタムメタデータキーの用途や説明"
              disabled={isSubmitting}
              rows="3"
            ></textarea>
          </div>

          <div class="form-group">
            <label for="default_value">デフォルト値</label>
            <input
              id="default_value"
              type="text"
              bind:value={formData.default_value}
              placeholder="新しいファイルに自動設定される値"
              disabled={isSubmitting}
            />
          </div>

          <div class="form-group">
            <label for="validation_pattern">バリデーションパターン</label>
            <input
              id="validation_pattern"
              type="text"
              bind:value={formData.validation_pattern}
              placeholder="正規表現パターン（任意）"
              disabled={isSubmitting}
            />
            <small>値の妥当性チェックに使用する正規表現</small>
          </div>

          <div class="form-group checkbox-group">
            <label class="checkbox-label">
              <input
                type="checkbox"
                bind:checked={formData.is_required}
                disabled={isSubmitting}
              />
              必須フィールド
            </label>
            <small>チェックすると、この値の入力が必須になります</small>
          </div>
        </div>

        <div class="form-actions">
          <button type="submit" class="save-button" disabled={isSubmitting}>
            <Save size={16} />
            {isSubmitting ? "保存中..." : editingKeyId ? "更新" : "作成"}
          </button>
          <button type="button" class="cancel-button" onclick={cancelForm} disabled={isSubmitting}>
            <X size={16} />
            キャンセル
          </button>
        </div>
      </form>
    </div>
  {/if}

  <!-- キー一覧 -->
  <div class="keys-list">
    {#if keys.length === 0}
      <div class="empty-state">
        <p>カスタムメタデータキーが設定されていません</p>
        <p>「新規作成」ボタンから最初のキーを作成してください</p>
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
                <button class="edit-btn" onclick={() => startEdit(key)} title="編集">
                  <Edit size={14} />
                </button>
                <button class="delete-btn" onclick={() => deleteKey(key.id, key.display_name)} title="削除">
                  <Trash2 size={14} />
                </button>
              </div>
            </div>
            
            <div class="key-details">
              <div class="detail-row">
                <span class="label">データ型:</span>
                <span class="value">
                  {getDataTypeInfo(key.data_type)?.label || key.data_type}
                  {key.is_required ? " (必須)" : ""}
                </span>
              </div>
              
              {#if key.description}
                <div class="detail-row">
                  <span class="label">説明:</span>
                  <span class="value">{key.description}</span>
                </div>
              {/if}
              
              {#if key.default_value}
                <div class="detail-row">
                  <span class="label">デフォルト値:</span>
                  <span class="value">{key.default_value}</span>
                </div>
              {/if}
              
              {#if key.validation_pattern}
                <div class="detail-row">
                  <span class="label">バリデーション:</span>
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

  .create-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: #007acc;
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 14px;
  }

  .create-button:hover {
    background: #005a9e;
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

  .checkbox-label {
    flex-direction: row;
    align-items: center;
    gap: 8px;
    margin-bottom: 4px;
  }

  .form-group label {
    margin-bottom: 4px;
    font-weight: 500;
    color: #333;
  }

  .form-group input,
  .form-group select,
  .form-group textarea {
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 14px;
  }

  .form-group input:focus,
  .form-group select:focus,
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

  .edit-btn,
  .delete-btn {
    padding: 4px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .edit-btn {
    background: #f8f9fa;
    color: #6c757d;
  }

  .edit-btn:hover {
    background: #e9ecef;
    color: #495057;
  }

  .delete-btn {
    background: #f8f9fa;
    color: #dc3545;
  }

  .delete-btn:hover {
    background: #f5c6cb;
    color: #721c24;
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
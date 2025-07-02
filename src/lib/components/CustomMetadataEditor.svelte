<script lang="ts">
  import { Save, X, Edit3, Trash2 } from "lucide-svelte";
  import { 
    getCustomMetadataValuesByFile, 
    setCustomMetadataValue, 
    deleteCustomMetadataValue 
  } from "../api/metadata.js";
  import type { 
    CustomMetadataKey, 
    CustomMetadataValue, 
    CustomMetadataDataType,
    SetCustomMetadataValueRequest 
  } from "../types.js";

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
      error = "メタデータ値の読み込みに失敗しました";
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
    return metadataValues.find(v => v.key_id === keyId);
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
      error = `${key.display_name}は必須項目です`;
      return false;
    }

    // データ型チェック
    if (value.trim()) {
      switch (key.data_type) {
        case "number":
          if (isNaN(Number(value))) {
            error = `${key.display_name}は数値で入力してください`;
            return false;
          }
          break;
        case "date":
          if (isNaN(Date.parse(value))) {
            error = `${key.display_name}は有効な日付で入力してください（例: 2024-01-01, 2024-01-01T10:00:00）`;
            return false;
          }
          break;
        case "boolean":
          const lowerValue = value.toLowerCase();
          if (!["true", "false", "1", "0", "yes", "no"].includes(lowerValue)) {
            error = `${key.display_name}はtrue/false、yes/no、1/0のいずれかで入力してください`;
            return false;
          }
          break;
        case "json":
          try {
            JSON.parse(value);
          } catch {
            error = `${key.display_name}は有効なJSON形式で入力してください`;
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
          error = `${key.display_name}の形式が正しくありません`;
          return false;
        }
      } catch {
        console.warn(`無効な正規表現パターン: ${key.validation_pattern}`);
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
        value: value.trim() || null
      };

      await setCustomMetadataValue(request);
      
      // フォーカスを保持するため、loadMetadataValues()を呼ばず直接値を更新
      const existingValueIndex = metadataValues.findIndex(v => v.key_id === key.id);
      if (existingValueIndex !== -1) {
        // 既存の値を更新
        metadataValues[existingValueIndex] = {
          ...metadataValues[existingValueIndex],
          value: value.trim() || null,
          updated_at: new Date().toISOString()
        };
      } else {
        // 新しい値を追加
        const newValue: CustomMetadataValue = {
          id: crypto.randomUUID(),
          file_id: fileId,
          key_id: key.id,
          value: value.trim() || null,
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString()
        };
        metadataValues.push(newValue);
      }
      
      return true;
    } catch (e) {
      error = typeof e === "string" ? e : "値の保存に失敗しました";
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
  const formatValue = (value: string, dataType: CustomMetadataDataType): string => {
    if (!value) return "";
    
    switch (dataType) {
      case "boolean":
        const lowerValue = value.toLowerCase();
        if (["true", "1", "yes"].includes(lowerValue)) return "はい";
        if (["false", "0", "no"].includes(lowerValue)) return "いいえ";
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
      case "number": return "number";
      case "date": return "datetime-local";
      default: return "text";
    }
  };

  // 入力要素のプレースホルダー
  const getPlaceholder = (key: CustomMetadataKey): string => {
    switch (key.data_type) {
      case "boolean": return "true/false, yes/no, 1/0";
      case "date": return "2024-01-01T10:00:00";
      case "json": return '{"key": "value"}';
      case "number": return "123.45";
      default: return `${key.display_name}を入力...`;
    }
  };
</script>

<div class="file-detail-section">
  <h4>カスタムメタデータ</h4>
  
  {#if isLoading}
    <div class="loading">読み込み中...</div>
  {:else if customMetadataKeys.length === 0}
    <div class="empty-state">
      <p>カスタムメタデータキーが定義されていません</p>
      <p>「メタデータ」タブから新しいキーを作成してください</p>
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
              <span class="saving-indicator">保存中...</span>
            {/if}
          </span>
          <span class="detail-value">
            {#if key.data_type === "boolean"}
              <select 
                value={displayValue}
                onchange={(e) => handleValueChange(key, (e.target as HTMLSelectElement).value)}
                class="inline-select"
              >
                <option value="">選択してください</option>
                <option value="true">はい</option>
                <option value="false">いいえ</option>
              </select>
            {:else if key.data_type === "json"}
              <textarea
                value={displayValue}
                oninput={(e) => handleValueChange(key, (e.target as HTMLTextAreaElement).value)}
                placeholder={getPlaceholder(key)}
                rows="3"
                class="inline-textarea"
              ></textarea>
            {:else}
              <input
                type={getInputType(key.data_type)}
                value={displayValue}
                oninput={(e) => handleValueChange(key, (e.target as HTMLInputElement).value)}
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

  .inline-input, .inline-select, .inline-textarea {
    border: 1px solid #ddd;
    border-radius: 4px;
    padding: 6px 8px;
    font-size: 14px;
    width: 100%;
    transition: border-color 0.2s, box-shadow 0.2s;
  }

  .inline-input:focus, .inline-select:focus, .inline-textarea:focus {
    outline: none;
    border-color: #007acc;
    box-shadow: 0 0 0 2px rgba(0, 122, 204, 0.2);
  }


  .inline-textarea {
    resize: vertical;
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    min-height: 60px;
  }

  .inline-select {
    background: white;
    cursor: pointer;
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
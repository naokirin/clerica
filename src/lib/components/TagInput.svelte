<script lang="ts">
  import type { Tag } from "../types";
  import { t } from "$lib/i18n";

  export let tags: Tag[] = [];
  export let placeholder = $t("common.tags.inputPlaceholder");
  export let disabled = false;
  export let onchange: (tags: Tag[]) => void = () => {};

  let inputValue = "";
  let inputRef: HTMLInputElement;
  let isComposing = false; // 日本語入力の変換中フラグ
  let pendingEnter = false; // 変換中にEnterが押されたかのフラグ

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === "Enter") {
      if (!isComposing && inputValue.trim()) {
        // 変換中でない場合は、通常のタグ追加処理
        event.preventDefault();
        addTag();
      }
    }
  }

  function handleCompositionStart() {
    isComposing = true;
  }

  function handleCompositionEnd() {
    // 変換終了後、pendingEnterがfalseで入力値がある場合のみタグを追加
    // これにより、変換確定のEnterではタグが追加されないようになる
    setTimeout(() => {
      isComposing = false;
    }, 10);
  }

  function addTag() {
    const tagName = inputValue.trim();
    if (!tagName) return;

    console.log("Adding tag:", tagName);

    // 重複チェック
    if (tags.find((tag) => tag.name.toLowerCase() === tagName.toLowerCase())) {
      console.log("Tag already exists:", tagName);
      inputValue = "";
      return;
    }

    // 新しいタグを作成（IDは仮で、実際の保存時にバックエンドで作成）
    const newTag: Tag = {
      id: `temp_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      name: tagName,
      color: "#3B82F6", // デフォルトブルー
      created_at: new Date().toISOString(),
    };

    const updatedTags = [...tags, newTag];
    console.log("Updated tags after adding:", updatedTags);
    tags = updatedTags;
    inputValue = "";
    console.log("Calling onchange with:", updatedTags);
    onchange(updatedTags);
  }

  function removeTag(tagToRemove: Tag) {
    console.log("Removing tag:", tagToRemove);
    const updatedTags = tags.filter((tag) => tag.id !== tagToRemove.id);
    console.log("Updated tags after removing:", updatedTags);
    tags = updatedTags;
    console.log("Calling onchange with:", updatedTags);
    onchange(updatedTags);
  }

  function onInputBlur() {
    if (inputValue.trim()) {
      addTag();
    }
  }
</script>

<div class="tag-input-container">
  <div class="tag-list">
    {#each tags as tag (tag.id)}
      <div
        class="tag-item"
        style="background-color: {tag.color}20; border-color: {tag.color};"
      >
        <span class="tag-name">{tag.name}</span>
        <button
          class="tag-remove"
          on:click={() => removeTag(tag)}
          {disabled}
          aria-label={$t("common.tags.removeAriaLabel", { name: tag.name })}
        >
          ✕
        </button>
      </div>
    {/each}
  </div>

  <input
    bind:this={inputRef}
    bind:value={inputValue}
    on:keydown={handleKeyDown}
    on:compositionstart={handleCompositionStart}
    on:compositionend={handleCompositionEnd}
    on:blur={onInputBlur}
    {placeholder}
    {disabled}
    class="tag-input"
    type="text"
  />
</div>

<style>
  .tag-input-container {
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    padding: 8px;
    background-color: white;
    min-height: 40px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .tag-input-container:focus-within {
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .tag-list {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .tag-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 8px;
    border: 1px solid;
    border-radius: 4px;
    font-size: 12px;
    background-color: rgba(59, 130, 246, 0.1);
    border-color: #3b82f6;
  }

  .tag-name {
    font-weight: 500;
    color: #374151;
  }

  .tag-remove {
    background: none;
    border: none;
    color: #6b7280;
    cursor: pointer;
    padding: 0;
    font-size: 14px;
    line-height: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    transition: all 0.2s;
  }

  .tag-remove:hover {
    background-color: rgba(239, 68, 68, 0.1);
    color: #ef4444;
  }

  .tag-remove:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  .tag-input {
    border: none;
    outline: none;
    padding: 4px 0;
    font-size: 14px;
    background: transparent;
    min-width: 120px;
    flex: 1;
  }

  .tag-input::placeholder {
    color: #9ca3af;
  }

  .tag-input:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }
</style>

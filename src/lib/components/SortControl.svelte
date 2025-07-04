<script lang="ts">
  import type { SortField, SortOrder, SortOptions } from "../types.js";
  import { ChevronUp, ChevronDown } from "lucide-svelte";

  interface Props {
    sortField: SortField;
    sortOrder: SortOrder;
    onSortChange: (options: SortOptions) => void;
  }

  let { sortField, sortOrder, onSortChange }: Props = $props();

  const sortFields: { value: SortField; label: string }[] = [
    { value: "modified_at", label: "更新日時" },
    { value: "name", label: "ファイル名" },
    { value: "size", label: "ファイルサイズ" },
    { value: "created_at", label: "作成日時" },
    { value: "last_accessed", label: "最終アクセス日時" },
    { value: "file_type", label: "ファイル種別" },
  ];

  function handleFieldChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    const newField = target.value as SortField;
    onSortChange({ field: newField, order: sortOrder });
  }

  function toggleSortOrder() {
    const newOrder: SortOrder = sortOrder === "asc" ? "desc" : "asc";
    onSortChange({ field: sortField, order: newOrder });
  }
</script>

<div class="sort-control">
  <select 
    value={sortField} 
    onchange={handleFieldChange} 
    class="sort-field-select"
  >
    {#each sortFields as field}
      <option value={field.value}>{field.label}</option>
    {/each}
  </select>
  <button 
    type="button"
    onclick={toggleSortOrder} 
    class="sort-order-btn"
    title={sortOrder === "asc" ? "昇順 (クリックで降順に変更)" : "降順 (クリックで昇順に変更)"}
  >
    {#if sortOrder === "asc"}
      <ChevronUp size={16} />
    {:else}
      <ChevronDown size={16} />
    {/if}
  </button>
</div>

<style>
  .sort-control {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .sort-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
    white-space: nowrap;
  }

  .sort-field-select {
    padding: 0.5rem 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    background-color: white;
    font-size: 0.875rem;
    color: #374151;
    min-width: 140px;
    cursor: pointer;
    transition: border-color 0.2s, box-shadow 0.2s;
  }

  .sort-field-select:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .sort-field-select:hover {
    border-color: #9ca3af;
  }

  .sort-order-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    padding: 0;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    background-color: white;
    color: #374151;
    cursor: pointer;
    transition: all 0.2s;
  }

  .sort-order-btn:hover {
    background-color: #f3f4f6;
    border-color: #9ca3af;
  }

  .sort-order-btn:active {
    background-color: #e5e7eb;
  }

  .sort-order-btn:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }
</style>
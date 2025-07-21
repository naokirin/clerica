<script lang="ts">
  import type { SortField, SortOrder, SortOptions } from "../../types";
  import { ChevronUp, ChevronDown } from "@lucide/svelte";
  import { t } from "$lib/i18n";
  import Select from "./Select.svelte";

  interface SortFieldOption {
    value: SortField;
    label: string;
  }

  interface Props {
    sortField: SortField;
    sortOrder: SortOrder;
    sortOptions: SortFieldOption[];
    onSortChange: (options: SortOptions) => Promise<void>;
  }

  let { sortField, sortOrder, sortOptions, onSortChange }: Props = $props();

  let sortFields = $derived(sortOptions);

  async function handleFieldChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    const newField = target.value as SortField;
    await onSortChange({ field: newField, order: sortOrder });
  }

  async function toggleSortOrder() {
    const newOrder: SortOrder = sortOrder === "asc" ? "desc" : "asc";
    await onSortChange({ field: sortField, order: newOrder });
  }
</script>

<div class="sort-control">
  <Select
    options={sortFields.map(field => ({
      value: field.value,
      label: field.label
    }))}
    value={sortField}
    on:change={handleFieldChange}
    className="sort-field-select"
  />
  <button 
    type="button"
    onclick={toggleSortOrder} 
    class="sort-order-btn"
    title={sortOrder === "asc" ? $t("common.files.sort.ascendingTooltip") : $t("common.files.sort.descendingTooltip")}
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
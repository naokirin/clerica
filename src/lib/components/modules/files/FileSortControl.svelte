<script lang="ts">
  import type { SortField, SortOrder, SortOptions } from "../../../types";
  import SortControl from "../../parts/SortControl.svelte";
  import { t } from "$lib/i18n";

  interface Props {
    sortField: SortField;
    sortOrder: SortOrder;
    onSortChange: (options: SortOptions) => Promise<void>;
  }

  let { sortField, sortOrder, onSortChange }: Props = $props();

  const fileSortOptions = [
    { value: "modified_at" as SortField, label: $t("common.files.sort.modified") },
    { value: "name" as SortField, label: $t("common.files.sort.name") },
    { value: "size" as SortField, label: $t("common.files.sort.size") },
    { value: "created_at" as SortField, label: $t("common.files.sort.created") },
    { value: "last_accessed" as SortField, label: $t("common.files.sort.lastAccessed") },
    { value: "file_type" as SortField, label: $t("common.files.sort.fileType") },
  ];
</script>

<SortControl 
  {sortField} 
  {sortOrder} 
  sortOptions={fileSortOptions}
  {onSortChange} 
/>
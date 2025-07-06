import { invoke } from "@tauri-apps/api/core";
import type { SearchResult, MetadataSearchFilter, MetadataSearchLogic, SortOptions } from "../types";

export async function searchFiles(
  query: string,
  tagIds: string[],
  metadataFilters: MetadataSearchFilter[],
  metadataLogic: MetadataSearchLogic = 'AND',
  directoryId?: string,
  sortOptions?: SortOptions
): Promise<SearchResult[]> {
  return await invoke("search_files", {
    query,
    tagIds,
    metadataFilters,
    metadataLogic,
    directoryId,
    sortField: sortOptions?.field || "modified_at",
    sortOrder: sortOptions?.order || "desc"
  });
}
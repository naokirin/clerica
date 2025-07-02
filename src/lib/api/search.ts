import { invoke } from "@tauri-apps/api/core";
import type { SearchResult, MetadataSearchFilter } from "../types.js";

export async function searchFiles(
  query: string,
  tagIds: string[],
  metadataFilters: MetadataSearchFilter[],
  directoryId?: string
): Promise<SearchResult[]> {
  return await invoke("search_files", {
    query,
    tagIds,
    metadataFilters,
    directoryId,
  });
}
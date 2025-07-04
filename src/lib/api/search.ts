import { invoke } from "@tauri-apps/api/core";
import type { SearchResult, MetadataSearchFilter, MetadataSearchLogic } from "../types.js";

export async function searchFiles(
  query: string,
  tagIds: string[],
  metadataFilters: MetadataSearchFilter[],
  metadataLogic: MetadataSearchLogic = 'AND',
  directoryId?: string
): Promise<SearchResult[]> {
  return await invoke("search_files", {
    query,
    tagIds,
    metadataFilters,
    metadataLogic,
    directoryId,
  });
}
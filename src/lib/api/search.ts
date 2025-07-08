import { invoke } from "@tauri-apps/api/core";
import type { SearchResult, PaginatedSearchResult, MetadataSearchFilter, MetadataSearchLogic, SortOptions, FileCategory } from "../types";

export async function searchFiles(
  query: string,
  tagIds: string[],
  metadataFilters: MetadataSearchFilter[],
  metadataLogic: MetadataSearchLogic = 'AND',
  directoryId?: string,
  sortOptions?: SortOptions,
  category?: FileCategory
): Promise<SearchResult[]> {
  return await invoke("search_files", {
    query,
    tagIds,
    metadataFilters,
    metadataLogic,
    directoryId,
    sortField: sortOptions?.field || "modified_at",
    sortOrder: sortOptions?.order || "desc",
    category
  });
}

export async function searchFilesPaginated(
  query: string,
  tagIds: string[],
  metadataFilters: MetadataSearchFilter[],
  metadataLogic: MetadataSearchLogic = 'AND',
  directoryId?: string,
  sortOptions?: SortOptions,
  limit: number = 20,
  offset: number = 0,
  category?: FileCategory
): Promise<PaginatedSearchResult> {
  return await invoke("search_files_paginated", {
    query,
    tagIds,
    metadataFilters,
    metadataLogic,
    directoryId,
    sortField: sortOptions?.field || "modified_at",
    sortOrder: sortOptions?.order || "desc",
    limit,
    offset,
    category
  });
}
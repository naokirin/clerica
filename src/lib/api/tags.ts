import { invoke } from "@tauri-apps/api/core";
import type { Tag, CustomMetadataKey } from "../types";

export async function getTags(): Promise<Tag[]> {
  return await invoke("get_tags");
}

export async function getTopTags(limit: number): Promise<Tag[]> {
  return await invoke("get_top_tags", { limit });
}

export async function searchTagsByName(query: string): Promise<Tag[]> {
  return await invoke("search_tags_by_name", { query });
}

export async function createTag(name: string, color: string): Promise<Tag> {
  return await invoke("create_tag", { name, color });
}

export async function updateTag(id: string, name: string, color: string): Promise<Tag> {
  return await invoke("update_tag", { id, name, color });
}

export async function deleteTag(id: string): Promise<void> {
  return await invoke("delete_tag", { id });
}

export async function getCustomMetadataKeys(): Promise<CustomMetadataKey[]> {
  return await invoke("get_custom_metadata_keys");
}
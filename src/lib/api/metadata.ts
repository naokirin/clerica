import { invoke } from "@tauri-apps/api/core";
import type {
  CustomMetadataValue,
  CreateCustomMetadataKeyRequest,
  UpdateCustomMetadataKeyRequest,
  SetCustomMetadataValueRequest
} from "../types";

export async function getCustomMetadataValuesByFile(fileId: string): Promise<CustomMetadataValue[]> {
  return await invoke("get_custom_metadata_values_by_file", { fileId });
}

export async function setCustomMetadataValue(request: SetCustomMetadataValueRequest): Promise<void> {
  return await invoke("set_custom_metadata_value", { request });
}

export async function deleteCustomMetadataValue(fileId: string, keyId: string): Promise<void> {
  return await invoke("delete_custom_metadata_value", { fileId, keyId });
}

export async function createCustomMetadataKey(request: CreateCustomMetadataKeyRequest): Promise<void> {
  return await invoke("create_custom_metadata_key", { request });
}

export async function updateCustomMetadataKey(request: UpdateCustomMetadataKeyRequest): Promise<void> {
  return await invoke("update_custom_metadata_key", { request });
}

export async function deleteCustomMetadataKey(keyId: string): Promise<void> {
  return await invoke("delete_custom_metadata_key", { keyId });
}
import { invoke } from "@tauri-apps/api/core";
import type { File } from "../types.js";

export async function getFiles(): Promise<File[]> {
  return await invoke("get_files");
}

export async function openFile(filePath: string): Promise<void> {
  return await invoke("open_file", { filePath });
}

export async function revealInFinder(filePath: string): Promise<void> {
  return await invoke("reveal_in_finder", { filePath });
}

export async function deleteFile(filePath: string): Promise<void> {
  return await invoke("delete_file", { filePath });
}
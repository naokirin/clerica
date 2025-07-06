import { invoke } from "@tauri-apps/api/core";
import type { Directory } from "../types";

export async function getDirectories(): Promise<Directory[]> {
  return await invoke("get_directories");
}

export async function addDirectory(path: string, name: string): Promise<void> {
  return await invoke("add_directory", { path, name });
}

export async function removeDirectory(id: string): Promise<void> {
  return await invoke("remove_directory", { id });
}

export async function rescanDirectory(directoryId: string): Promise<void> {
  return await invoke("rescan_directory", { directoryId });
}
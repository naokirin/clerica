import { invoke } from '@tauri-apps/api/core';

export interface Shelf {
  id: string;
  name: string;
  created_at: string;
}

export interface CreateShelfRequest {
  name: string;
}

export interface UpdateShelfRequest {
  id: string;
  name: string;
}

export const shelvesApi = {
  async getShelves(): Promise<Shelf[]> {
    return await invoke('get_shelves');
  },

  async getActiveShelfId(): Promise<string> {
    return await invoke('get_active_shelf_id');
  },

  async createShelf(request: CreateShelfRequest): Promise<Shelf> {
    return await invoke('create_shelf', { request });
  },

  async switchShelf(shelfId: string): Promise<void> {
    await invoke('switch_shelf', { shelfId });
  },

  async deleteShelf(shelfId: string): Promise<void> {
    await invoke('delete_shelf', { shelfId });
  },

  async updateShelfName(request: UpdateShelfRequest): Promise<void> {
    await invoke('update_shelf_name', { request });
  }
};
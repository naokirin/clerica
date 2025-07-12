import { invoke } from '@tauri-apps/api/core';

export interface Group {
  id: string;
  name: string;
  created_at: string;
}

export interface CreateGroupRequest {
  name: string;
}

export interface UpdateGroupRequest {
  id: string;
  name: string;
}

export const groupsApi = {
  async getGroups(): Promise<Group[]> {
    return await invoke('get_groups');
  },

  async getActiveGroupId(): Promise<string> {
    return await invoke('get_active_group_id');
  },

  async createGroup(request: CreateGroupRequest): Promise<Group> {
    return await invoke('create_group', { request });
  },

  async switchGroup(groupId: string): Promise<void> {
    await invoke('switch_group', { groupId });
  },

  async deleteGroup(groupId: string): Promise<void> {
    await invoke('delete_group', { groupId });
  },

  async updateGroupName(request: UpdateGroupRequest): Promise<void> {
    await invoke('update_group_name', { request });
  }
};
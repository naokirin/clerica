import { invoke } from '@tauri-apps/api/core';

export interface AppSettings {
  show_hidden_files: boolean;
  files_per_page: number;
}

export const getSettings = async (): Promise<AppSettings> => {
  return await invoke('get_settings');
};

export const updateSettingBool = async (key: string, value: boolean): Promise<void> => {
  return await invoke('update_setting_bool_cmd', { key, value });
};

export const updateSettingInt = async (key: string, value: number): Promise<void> => {
  return await invoke('update_setting_int_cmd', { key, value });
};
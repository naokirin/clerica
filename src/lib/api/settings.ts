import { invoke } from '@tauri-apps/api/core';
import type { Settings, SettingsUpdateRequest } from '../types/settings';

export interface AppSettings {
  show_hidden_files: boolean;
  files_per_page: number;
}

// 既存のAPI（互換性維持）
export const getSettings = async (): Promise<AppSettings> => {
  return await invoke('get_settings');
};

export const updateSettingBool = async (key: string, value: boolean): Promise<void> => {
  return await invoke('update_setting_bool_cmd', { key, value });
};

export const updateSettingInt = async (key: string, value: number): Promise<void> => {
  return await invoke('update_setting_int_cmd', { key, value });
};

export const updateSettingString = async (key: string, value: string): Promise<void> => {
  return await invoke('update_setting_string_cmd', { key, value });
};

export const getLanguageSetting = async (): Promise<string> => {
  return await invoke('get_language_setting');
};

// 新しい統一API
export async function getAppSettings(): Promise<Settings> {
  try {
    const [settings, language] = await Promise.all([
      invoke<AppSettings>('get_settings'),
      getLanguageSetting()
    ]);
    // 既存の設定を新しい型に変換
    return {
      itemsPerPage: settings.files_per_page,
      defaultSortKey: 'name',
      defaultSortOrder: 'asc',
      showHiddenFiles: settings.show_hidden_files,
      showDirectories: true,
      autoSave: true,
      language
    };
  } catch (error) {
    console.error('Failed to get settings:', error);
    // フォールバック設定を返す
    return {
      itemsPerPage: 20,
      defaultSortKey: 'name',
      defaultSortOrder: 'asc',
      showHiddenFiles: false,
      showDirectories: true,
      autoSave: true,
      language: 'ja'
    };
  }
}

export async function updateAppSettings(settings: SettingsUpdateRequest): Promise<void> {
  try {
    // 既存のAPIを使用して個別に更新
    if (settings.itemsPerPage !== undefined) {
      await updateSettingInt('files_per_page', settings.itemsPerPage);
    }
    if (settings.showHiddenFiles !== undefined) {
      await updateSettingBool('show_hidden_files', settings.showHiddenFiles);
    }
    if (settings.language !== undefined) {
      await updateSettingString('language', settings.language);
    }
    // TODO: 他の設定項目は将来のRustコマンド実装で対応
  } catch (error) {
    console.error('Failed to update settings:', error);
    throw error;
  }
}
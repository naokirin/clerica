/**
 * 設定関連の型定義
 */

export interface Settings {
  itemsPerPage: number;
  defaultSortKey: string;
  defaultSortOrder: 'asc' | 'desc';
  showHiddenFiles: boolean;
  autoSave: boolean;
  language: string;
}

export interface SettingsUpdateRequest {
  itemsPerPage?: number;
  defaultSortKey?: string;
  defaultSortOrder?: 'asc' | 'desc';
  showHiddenFiles?: boolean;
  autoSave?: boolean;
  language?: string;
}
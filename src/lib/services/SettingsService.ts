/**
 * 設定管理サービス
 * アプリケーション全体の設定を一元管理
 */

import { writable, type Readable } from 'svelte/store';
import { getAppSettings, updateAppSettings } from '../api/settings';
import type { Settings } from '../types/settings';

export interface SettingsService {
  settings: Readable<Settings>;
  isLoading: Readable<boolean>;
  error: Readable<string | null>;
  loadSettings(): Promise<void>;
  updateItemsPerPage(itemsPerPage: number): Promise<void>;
  updateSetting<K extends keyof Settings>(key: K, value: Settings[K]): Promise<void>;
  getSetting<K extends keyof Settings>(key: K): Settings[K];
}

class SettingsServiceImpl implements SettingsService {
  private _settings = writable<Settings>({
    itemsPerPage: 20,
    defaultSortKey: 'name',
    defaultSortOrder: 'asc',
    showHiddenFiles: false,
    showDirectories: true,
    autoSave: true,
    language: 'ja'
  });
  
  private _isLoading = writable<boolean>(false);
  private _error = writable<string | null>(null);

  public readonly settings = { subscribe: this._settings.subscribe };
  public readonly isLoading = { subscribe: this._isLoading.subscribe };
  public readonly error = { subscribe: this._error.subscribe };

  private isInitialized = false;

  async loadSettings(): Promise<void> {
    if (this.isInitialized) return;
    
    try {
      this._isLoading.set(true);
      this._error.set(null);
      
      const settings = await getAppSettings();
      this._settings.set(settings);
      this.isInitialized = true;
    } catch (error) {
      console.error('Failed to load settings:', error);
      this._error.set(error instanceof Error ? error.message : String(error));
    } finally {
      this._isLoading.set(false);
    }
  }

  async updateItemsPerPage(itemsPerPage: number): Promise<void> {
    await this.updateSetting('itemsPerPage', itemsPerPage);
  }

  async updateSetting<K extends keyof Settings>(key: K, value: Settings[K]): Promise<void> {
    try {
      this._isLoading.set(true);
      this._error.set(null);

      // 楽観的更新
      this._settings.update(current => ({ ...current, [key]: value }));

      // バックエンドに保存
      await updateAppSettings({ [key]: value } as Partial<Settings>);
    } catch (error) {
      console.error(`Failed to update setting ${String(key)}:`, error);
      this._error.set(error instanceof Error ? error.message : String(error));
      
      // エラー時は設定を再読み込み
      await this.loadSettings();
    } finally {
      this._isLoading.set(false);
    }
  }

  // 設定値を同期的に取得するヘルパーメソッド
  getCurrentSettings(): Settings {
    let currentSettings: Settings = {
      itemsPerPage: 20,
      defaultSortKey: 'name',
      defaultSortOrder: 'asc',
      showHiddenFiles: false,
      showDirectories: true,
      autoSave: true,
      language: 'ja'
    };
    
    const unsubscribe = this._settings.subscribe(settings => {
      currentSettings = settings;
    });
    unsubscribe();
    
    return currentSettings;
  }

  // 特定の設定値を同期的に取得
  getSetting<K extends keyof Settings>(key: K): Settings[K] {
    return this.getCurrentSettings()[key];
  }
}

// シングルトンインスタンス
export const settingsService: SettingsService = new SettingsServiceImpl();

// 便利な型エクスポート
export type { Settings } from '../types/settings';
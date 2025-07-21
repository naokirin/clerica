import type { LayoutLoad } from './$types';
import { loadTranslations } from '$lib/i18n';
import { invoke } from '@tauri-apps/api/core';

export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = async ({ url }) => {
  const { pathname } = url;
  
  let initLocale = 'ja'; // デフォルト言語は日本語
  
  try {
    // Tauriでデータベースから言語設定を取得
    const language = await invoke<string>('get_language_setting');
    if (language && ['ja', 'en'].includes(language)) {
      initLocale = language;
    }
  } catch (error) {
    console.warn('Failed to load language setting from database:', error);
    // フォールバック: localStorageから取得を試行
    try {
      const savedLocale = localStorage.getItem('locale');
      if (savedLocale && ['ja', 'en'].includes(savedLocale)) {
        initLocale = savedLocale;
      }
    } catch (storageError) {
      console.warn('Failed to load from localStorage:', storageError);
    }
  }

  await loadTranslations(initLocale, pathname);

  return { locale: initLocale };
};
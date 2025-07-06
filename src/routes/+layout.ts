import type { LayoutLoad } from './$types';
import { loadTranslations } from '$lib/i18n';

export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = async ({ url }) => {
  const { pathname } = url;
  
  // URLからロケールを取得、なければデフォルトを設定
  const defaultLocale = 'ja'; // デフォルト言語は日本語
  const localeFromUrl = pathname.split('/')[1];
  const initLocale = ['ja', 'en'].includes(localeFromUrl) ? localeFromUrl : defaultLocale;

  await loadTranslations(initLocale, pathname);

  return { locale: initLocale };
};
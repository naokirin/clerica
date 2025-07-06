// src/lib/i18n/index.ts
import i18n from 'sveltekit-i18n';
import type { Config } from 'sveltekit-i18n';

// サポートする言語のリスト
export const locales = ['ja', 'en'];

const config: Config = {
  fallbackLocale: 'ja',
  loaders: [
    {
      locale: 'ja',
      key: 'common',
      loader: async () => (await import('./ja/common.json')).default,
    },
    {
      locale: 'en',
      key: 'common',
      loader: async () => (await import('./en/common.json')).default,
    },
  ],
};

export const { t, locale, locales: availableLocales, loading, loadTranslations } = new i18n(config);
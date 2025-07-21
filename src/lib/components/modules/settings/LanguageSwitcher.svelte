<!-- src/lib/components/LanguageSwitcher.svelte -->
<script lang="ts">
  import { locale, locales } from '$lib/i18n';
  import { t } from '$lib/i18n';
  import Select from '../../parts/Select.svelte';

  const languages = [
    { code: 'ja', name: '日本語' },
    { code: 'en', name: 'English' }
  ];

  const switchLanguage = (lang: string) => {
    $locale = lang;
    // localStorageは削除してデータベース保存に一本化
  };
</script>

<div class="language-switcher">
  <Select
    label={$t('common.settings.language')}
    options={languages.map(lang => ({
      value: lang.code,
      label: lang.name
    }))}
    value={$locale}
    on:change={(e) => switchLanguage(e.target.value)}
    id="language-select"
  />
</div>

<style>
  .language-switcher {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

</style>
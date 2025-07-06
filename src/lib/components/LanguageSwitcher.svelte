<!-- src/lib/components/LanguageSwitcher.svelte -->
<script lang="ts">
  import { locale, locales } from '$lib/i18n';
  import { t } from '$lib/i18n';

  const languages = [
    { code: 'ja', name: '日本語' },
    { code: 'en', name: 'English' }
  ];

  const switchLanguage = (lang: string) => {
    $locale = lang;
    localStorage.setItem('locale', lang);
  };
</script>

<div class="language-switcher">
  <label for="language-select">{$t('common.settings.language')}</label>
  <select 
    id="language-select"
    value={$locale}
    onchange={(e) => switchLanguage(e.target.value)}
  >
    {#each languages as language}
      <option value={language.code}>
        {language.name}
      </option>
    {/each}
  </select>
</div>

<style>
  .language-switcher {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  label {
    font-weight: 500;
    color: #374151;
    font-size: 14px;
  }

  select {
    padding: 8px 12px;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    background: white;
    font-size: 14px;
    color: #374151;
  }

  select:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }
</style>
<script lang="ts">
  import { onMount } from 'svelte';
  import '../index.css';
  import Snackbar from '../lib/components/Snackbar.svelte';
  import { locale } from '$lib/i18n';
  import { errorStore } from '../lib/stores/error';
  import type { LayoutData } from './$types';

  export let data: LayoutData;

  // load関数から渡されたlocaleをストアに設定
  $locale = data.locale;

  onMount(async () => {
    // 中断されたディレクトリスキャンの監視
    const { listen } = await import('@tauri-apps/api/event');
    const unlisten = await listen('scan-interrupted', (event) => {
      if (event.payload) {
        errorStore.showWarning(event.payload as string, 10000);
      }
    });

    return () => {
      unlisten();
    };
  });
</script>

<main>
  <slot />
  <Snackbar />
</main>

<style>
  main {
    height: 100vh;
    width: 100vw;
  }
</style>
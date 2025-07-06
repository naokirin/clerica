<script lang="ts">
  import '../index.css';
  import Snackbar from '../lib/components/Snackbar.svelte';
  import { locale } from '$lib/i18n';
  import type { LayoutData } from './$types';

  export let data: LayoutData;

  // load関数から渡されたlocaleをストアに設定
  $locale = data.locale;
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
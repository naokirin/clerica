<script lang="ts">
  import '../index.css';
  import Snackbar from '../lib/components/Snackbar.svelte';
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
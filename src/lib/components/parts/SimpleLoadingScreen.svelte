<script lang="ts">
  import { Loader2 } from "lucide-svelte";
  import { t } from "$lib/i18n";

  interface Props {
    message?: string;
  }

  let { message = "" }: Props = $props();
</script>

<div class="loading-overlay">
  <div class="loading-container">
    <div class="loading-content">
      <div class="loading-spinner">
        <Loader2 size={48} class="animate-spin" />
      </div>

      {#if message}
        <p class="loading-message">{message}</p>
      {/if}

      <div class="warning-message">
        <p>{$t("common.loading.warningMessage")}</p>
      </div>
    </div>
  </div>
</div>

<style>
  .loading-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 9999;
  }

  .loading-container {
    background-color: white;
    border-radius: 8px;
    padding: 2rem;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    max-width: 420px;
    text-align: center;
  }

  .loading-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .loading-spinner {
    color: #3b82f6;
  }

  .loading-message {
    font-size: 1.125rem;
    font-weight: 500;
    color: #374151;
    margin: 0;
  }

  .warning-message {
    margin-top: 1rem;
    padding: 0.75rem;
    background-color: #fef3c7;
    border-radius: 4px;
    border-left: 4px solid #f59e0b;
  }

  .warning-message p {
    margin: 0;
    font-size: 0.875rem;
    color: #92400e;
  }

  :global(.animate-spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>

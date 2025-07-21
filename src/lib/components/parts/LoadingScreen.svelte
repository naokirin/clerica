<script lang="ts">
  import { Loader2 } from "@lucide/svelte";
  import type { LoadingSteps } from "../../types";
  import { t } from "$lib/i18n";

  interface Props {
    isVisible: boolean;
    progress: number;
    steps: LoadingSteps;
  }

  let { isVisible, progress, steps }: Props = $props();
</script>

{#if isVisible}
  <div class="loading-overlay">
    <div class="loading-container">
      <div class="loading-logo">
        <h1>Clerica</h1>
        <p>{$t("common.app.description")}</p>
      </div>

      <div class="loading-content">
        {#if progress < 100}
          <div class="loading-spinner">
            <Loader2 size={48} class="animate-spin" />
          </div>
        {:else}
          <div class="loading-complete">
            <p>{$t("common.loading.ready")}</p>
          </div>
        {/if}

        <div class="loading-progress">
          <div class="progress-bar">
            <div class="progress-fill" style="width: {progress}%"></div>
          </div>
          <div class="progress-text">{progress}%</div>
        </div>

        <div class="loading-steps">
          <div class="loading-step {steps.directories ? 'completed' : ''}">
            <div class="step-icon">
              {#if steps.directories}
                ✓
              {:else}
                <div class="step-dot"></div>
              {/if}
            </div>
            <span
              >{steps.directories
                ? $t("common.loading.directoriesComplete")
                : $t("common.loading.directoriesLoading")}</span
            >
          </div>

          <div class="loading-step {steps.tags ? 'completed' : ''}">
            <div class="step-icon">
              {#if steps.tags}
                ✓
              {:else}
                <div class="step-dot"></div>
              {/if}
            </div>
            <span
              >{steps.tags ? $t("common.loading.tagsComplete") : $t("common.loading.tagsLoading")}</span
            >
          </div>

          <div class="loading-step {steps.files ? 'completed' : ''}">
            <div class="step-icon">
              {#if steps.files}
                ✓
              {:else}
                <div class="step-dot"></div>
              {/if}
            </div>
            <span
              >{steps.files
                ? $t("common.loading.filesComplete")
                : $t("common.loading.filesLoading")}</span
            >
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .loading-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.3s ease-in-out;
  }

  .loading-container {
    text-align: center;
    max-width: 400px;
    padding: 2rem;
  }

  .loading-logo {
    margin-bottom: 3rem;
  }

  .loading-logo h1 {
    font-size: 3rem;
    font-weight: 700;
    color: white;
    margin: 0;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  }

  .loading-logo p {
    font-size: 1.1rem;
    color: rgba(255, 255, 255, 0.9);
    margin: 0.5rem 0 0 0;
  }

  .loading-content {
    background: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(10px);
    border-radius: 1rem;
    padding: 2rem;
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .loading-spinner {
    margin-bottom: 2rem;
    color: white;
  }

  .loading-complete {
    margin-bottom: 2.75rem;
    color: white;
  }

  .loading-progress {
    margin-bottom: 2rem;
  }

  .progress-bar {
    width: 100%;
    height: 8px;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 0.5rem;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #4ade80, #22c55e);
    border-radius: 4px;
    transition: width 0.3s ease-in-out;
    animation: progressShine 2s infinite;
  }

  .progress-text {
    color: white;
    font-size: 0.875rem;
    font-weight: 600;
  }

  .loading-steps {
    text-align: left;
  }

  .loading-step {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 0;
    color: rgba(255, 255, 255, 0.7);
    transition: color 0.3s ease-in-out;
  }

  .loading-step.completed {
    color: white;
  }

  .step-icon {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: 0.875rem;
  }

  .step-dot {
    width: 8px;
    height: 8px;
    background: rgba(255, 255, 255, 0.5);
    border-radius: 50%;
    animation: pulse 2s infinite;
  }

  .loading-step.completed .step-icon {
    color: #4ade80;
    font-size: 1rem;
  }

  /* アニメーション */
  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes progressShine {
    0% {
      background-position: -200px 0;
    }
    100% {
      background-position: 200px 0;
    }
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
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

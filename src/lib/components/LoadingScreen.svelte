<script lang="ts">
  import { Loader2 } from "lucide-svelte";
  import type { LoadingSteps } from "../types";
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

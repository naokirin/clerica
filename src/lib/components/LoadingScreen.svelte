<script lang="ts">
  import { Loader2 } from "lucide-svelte";
  import type { LoadingSteps } from "../types";

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
        <p>Mac向けファイル整理・検索ツール</p>
      </div>

      <div class="loading-content">
        {#if progress < 100}
          <div class="loading-spinner">
            <Loader2 size={48} class="animate-spin" />
          </div>
        {:else}
          <div class="loading-complete">
            <p>準備が完了しました！</p>
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
                ? "ディレクトリ読み込み完了"
                : "ディレクトリを読み込み中..."}</span
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
              >{steps.tags ? "タグ読み込み完了" : "タグを読み込み中..."}</span
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
                ? "ファイル読み込み完了"
                : "ファイルを読み込み中..."}</span
            >
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}

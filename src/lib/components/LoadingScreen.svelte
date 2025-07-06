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
        <div class="loading-spinner">
          <Loader2 size={48} class="animate-spin" />
        </div>
        
        <div class="loading-progress">
          <div class="progress-bar">
            <div 
              class="progress-fill" 
              style="width: {progress}%"
            ></div>
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
            <span>ディレクトリを読み込み中...</span>
          </div>
          
          <div class="loading-step {steps.tags ? 'completed' : ''}">
            <div class="step-icon">
              {#if steps.tags}
                ✓
              {:else}
                <div class="step-dot"></div>
              {/if}
            </div>
            <span>タグを読み込み中...</span>
          </div>
          
          <div class="loading-step {steps.files ? 'completed' : ''}">
            <div class="step-icon">
              {#if steps.files}
                ✓
              {:else}
                <div class="step-dot"></div>
              {/if}
            </div>
            <span>ファイルを読み込み中...</span>
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}
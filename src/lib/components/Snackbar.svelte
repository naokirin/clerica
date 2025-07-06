<script lang="ts">
  import { errorStore, type ErrorInfo } from '../stores/error.js';
  import { fly } from 'svelte/transition';
  import { X, AlertCircle, CheckCircle, Info, AlertTriangle } from 'lucide-svelte';

  let { errors } = errorStore;

  const getIcon = (type: ErrorInfo['type']) => {
    switch (type) {
      case 'error':
        return AlertCircle;
      case 'warning':
        return AlertTriangle;
      case 'info':
        return Info;
      case 'success':
        return CheckCircle;
      default:
        return Info;
    }
  };

  const getColorClass = (type: ErrorInfo['type']) => {
    switch (type) {
      case 'error':
        return 'snackbar-error';
      case 'warning':
        return 'snackbar-warning';
      case 'info':
        return 'snackbar-info';
      case 'success':
        return 'snackbar-success';
      default:
        return 'snackbar-info';
    }
  };

  const dismissError = (id: string) => {
    errorStore.dismissError(id);
  };
</script>

<div class="snackbar-container">
  {#each $errors as error (error.id)}
    <div
      class="snackbar {getColorClass(error.type)}"
      transition:fly="{{ y: -50, duration: 300 }}"
    >
      <div class="snackbar-content">
        <div class="snackbar-icon">
          <svelte:component this={getIcon(error.type)} size={20} />
        </div>
        <div class="snackbar-message">
          {error.message}
        </div>
        <button
          class="snackbar-close"
          onclick={() => dismissError(error.id)}
          aria-label="閉じる"
        >
          <X size={16} />
        </button>
      </div>
    </div>
  {/each}
</div>

<style>
  .snackbar-container {
    position: fixed;
    top: 20px;
    right: 20px;
    z-index: 1000;
    display: flex;
    flex-direction: column;
    gap: 8px;
    pointer-events: none;
  }

  .snackbar {
    min-width: 320px;
    max-width: 500px;
    padding: 12px 16px;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    pointer-events: auto;
    font-size: 14px;
    line-height: 1.4;
  }

  .snackbar-content {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .snackbar-icon {
    flex-shrink: 0;
    display: flex;
    align-items: center;
  }

  .snackbar-message {
    flex: 1;
    word-wrap: break-word;
  }

  .snackbar-close {
    flex-shrink: 0;
    background: none;
    border: none;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.2s ease;
  }

  .snackbar-close:hover {
    background-color: rgba(255, 255, 255, 0.1);
  }

  .snackbar-error {
    background-color: #fee2e2;
    color: #991b1b;
    border: 1px solid #fecaca;
  }

  .snackbar-warning {
    background-color: #fef3c7;
    color: #92400e;
    border: 1px solid #fed7aa;
  }

  .snackbar-info {
    background-color: #dbeafe;
    color: #1e40af;
    border: 1px solid #bfdbfe;
  }

  .snackbar-success {
    background-color: #dcfce7;
    color: #166534;
    border: 1px solid #bbf7d0;
  }

  .snackbar-error .snackbar-close:hover {
    background-color: rgba(153, 27, 27, 0.1);
  }

  .snackbar-warning .snackbar-close:hover {
    background-color: rgba(146, 64, 14, 0.1);
  }

  .snackbar-info .snackbar-close:hover {
    background-color: rgba(30, 64, 175, 0.1);
  }

  .snackbar-success .snackbar-close:hover {
    background-color: rgba(22, 101, 52, 0.1);
  }

  /* ダークモード対応 */
  @media (prefers-color-scheme: dark) {
    .snackbar-error {
      background-color: #7f1d1d;
      color: #fef2f2;
      border: 1px solid #991b1b;
    }

    .snackbar-warning {
      background-color: #78350f;
      color: #fef3c7;
      border: 1px solid #92400e;
    }

    .snackbar-info {
      background-color: #1e3a8a;
      color: #dbeafe;
      border: 1px solid #3b82f6;
    }

    .snackbar-success {
      background-color: #14532d;
      color: #dcfce7;
      border: 1px solid #166534;
    }
  }
</style>
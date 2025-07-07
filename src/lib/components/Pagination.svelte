<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let currentPage: number;
  export let totalPages: number;
  export let maxVisiblePages = 7;
  export let disabled = false;

  const dispatch = createEventDispatcher<{
    goToPage: { page: number };
    goToPreviousPage: {};
    goToNextPage: {};
    goToFirstPage: {};
    goToLastPage: {};
  }>();

  function goToPage(page: number) {
    if (page < 1 || page > totalPages || page === currentPage || disabled) return;
    dispatch('goToPage', { page });
  }

  function goToPreviousPage() {
    if (currentPage === 1 || disabled) return;
    dispatch('goToPreviousPage', {});
  }

  function goToNextPage() {
    if (currentPage === totalPages || disabled) return;
    dispatch('goToNextPage', {});
  }

  function goToFirstPage() {
    if (currentPage === 1 || disabled) return;
    dispatch('goToFirstPage', {});
  }

  function goToLastPage() {
    if (currentPage === totalPages || disabled) return;
    dispatch('goToLastPage', {});
  }

  $: visiblePages = (() => {
    const pages: number[] = [];
    let start = Math.max(1, currentPage - Math.floor(maxVisiblePages / 2));
    let end = Math.min(totalPages, start + maxVisiblePages - 1);
    start = Math.max(1, end - maxVisiblePages + 1);
    
    for (let i = start; i <= end; i++) {
      pages.push(i);
    }
    
    return pages;
  })();
</script>

{#if totalPages > 1}
  <div class="pagination-controls">
    <div class="pagination-buttons">
      <button
        class="pagination-btn"
        onclick={goToFirstPage}
        disabled={currentPage === 1 || disabled}
      >
        ≪
      </button>
      <button
        class="pagination-btn"
        onclick={goToPreviousPage}
        disabled={currentPage === 1 || disabled}
      >
        ‹
      </button>

      {#each visiblePages as page}
        <button
          class="pagination-btn {currentPage === page ? 'active' : ''}"
          onclick={() => goToPage(page)}
          disabled={disabled}
        >
          {page}
        </button>
      {/each}

      <button
        class="pagination-btn"
        onclick={goToNextPage}
        disabled={currentPage === totalPages || disabled}
      >
        ›
      </button>
      <button
        class="pagination-btn"
        onclick={goToLastPage}
        disabled={currentPage === totalPages || disabled}
      >
        ≫
      </button>
    </div>
  </div>
{/if}

<style>
  .pagination-controls {
    display: flex;
    justify-content: center;
    margin: 1rem 0;
  }

  .pagination-buttons {
    display: flex;
    gap: 0.5rem;
  }

  .pagination-btn {
    padding: 0.5rem 0.75rem;
    border: 1px solid #d1d5db;
    background: white;
    color: #374151;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.875rem;
    transition: all 0.2s ease;
    min-width: 40px;
  }

  .pagination-btn:hover:not(:disabled) {
    background: #f3f4f6;
    border-color: #9ca3af;
  }

  .pagination-btn.active {
    background: #3b82f6;
    color: white;
    border-color: #3b82f6;
  }

  .pagination-btn:disabled {
    background: #f9fafb;
    color: #9ca3af;
    border-color: #e5e7eb;
    cursor: not-allowed;
  }
</style>
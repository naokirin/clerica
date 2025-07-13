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
    align-items: center;
    margin: 1.5rem 0;
  }

  .pagination-buttons {
    display: flex;
    gap: 0.5rem;
  }

  .pagination-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 40px;
    height: 40px;
    padding: 0.5rem;
    border: 1px solid #d1d5db;
    background-color: white;
    color: #374151;
    font-size: 0.875rem;
    font-weight: 500;
    border-radius: 0;
    cursor: pointer;
    transition: all 0.2s;
    user-select: none;
    margin-left: -1px;
  }

  .pagination-btn:first-child {
    border-top-left-radius: 0.375rem;
    border-bottom-left-radius: 0.375rem;
    margin-left: 0;
  }

  .pagination-btn:last-child {
    border-top-right-radius: 0.375rem;
    border-bottom-right-radius: 0.375rem;
  }

  .pagination-btn:hover:not(:disabled) {
    background-color: #f9fafb;
    border-color: #9ca3af;
    z-index: 1;
    position: relative;
  }

  .pagination-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    background-color: #f9fafb;
    color: #9ca3af;
  }

  .pagination-btn.active {
    background-color: #3b82f6;
    border-color: #3b82f6;
    color: white;
    z-index: 1;
    position: relative;
  }

  .pagination-btn.active:hover {
    background-color: #2563eb;
    border-color: #2563eb;
  }

  .pagination-bottom {
    margin-top: 1.5rem;
    border-top: 1px solid #e5e7eb;
    padding-top: 1.5rem;
  }

  /* レスポンシブ対応 */
  @media (max-width: 768px) {
    .pagination-controls {
      flex-wrap: wrap;
      gap: 0.5rem;
    }

    .pagination-btn {
      min-width: 36px;
      height: 36px;
      font-size: 0.75rem;
    }
  }
</style>
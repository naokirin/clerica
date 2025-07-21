<!-- src/lib/components/parts/Select.svelte -->
<script lang="ts">
  export let options: { value: string | number; label: string }[] = [];
  export let value: string | number | undefined = undefined;
  export let placeholder: string = "選択してください";
  export let label: string | undefined = undefined;
  export let id: string = `select-${crypto.randomUUID()}`;
  export let className: string = "";
</script>

<div class="select-wrapper">
  {#if label}
    <label for={id} class="select-label">{label}</label>
  {/if}
  <div class="select-container">
    <select
      class="custom-select {className}"
      {id}
      bind:value
      on:change
      aria-label={label ? undefined : placeholder}
      {...$$restProps}
    >
      <option disabled selected={value === undefined} value={undefined}>
        {placeholder}
      </option>
      {#each options as option (option.value)}
        <option value={option.value}>{option.label}</option>
      {/each}
    </select>
    <div class="select-arrow" aria-hidden="true">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 20 20"
        fill="currentColor"
      >
        <path
          fill-rule="evenodd"
          d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
          clip-rule="evenodd"
        />
      </svg>
    </div>
  </div>
</div>

<style>
  .select-wrapper {
    display: flex;
    flex-direction: column;
    gap: 0.5rem; /* 8px */
    width: 100%;
  }

  .select-label {
    font-size: 0.875rem; /* 14px */
    font-weight: 500;
    color: var(--text-primary, #374151);
  }

  .select-container {
    position: relative;
    display: inline-block;
    width: 100%;
  }

  .custom-select {
    /* Reset browser default styles */
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;

    /* Custom styles */
    width: 100%;
    padding: 0.55rem 2.5rem 0.55rem 1rem; /* 12px 40px 12px 16px */
    font-size: 1rem; /* 16px */
    border: 1px solid var(--border-color, #d1d5db);
    border-radius: 0.375rem; /* 6px */
    background-color: var(--bg-primary, #ffffff);
    color: var(--text-primary, #111827);
    cursor: pointer;
    transition:
      border-color 0.2s ease-in-out,
      box-shadow 0.2s ease-in-out;
  }

  .custom-select:focus {
    outline: none;
    border-color: var(--primary-color, #2563eb);
    box-shadow: 0 0 0 3px var(--primary-color-alpha, rgba(59, 130, 246, 0.3));
  }

  .custom-select:disabled {
    background-color: var(--bg-disabled, #f3f4f6);
    color: var(--text-disabled, #6b7280);
    cursor: not-allowed;
  }

  /* Custom dropdown arrow */
  .select-arrow {
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    display: flex;
    align-items: center;
    padding-right: 0.75rem; /* 12px */
    pointer-events: none; /* Allows clicks to pass through to the select */
  }

  .select-arrow svg {
    width: 1.25rem; /* 20px */
    height: 1.25rem; /* 20px */
    color: var(--text-secondary, #6b7280);
  }
</style>

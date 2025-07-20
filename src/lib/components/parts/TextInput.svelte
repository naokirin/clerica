<script lang="ts">
  interface Props {
    id: string;
    value: string;
    placeholder?: string;
    disabled?: boolean;
    required?: boolean;
    class?: string;
    oninput?: (event: Event) => void;
  }

  let {
    id,
    value = $bindable(),
    placeholder = "",
    disabled = false,
    required = false,
    class: className = "",
    oninput,
    ...restProps
  }: Props = $props();
</script>

<input
  type="text"
  {id}
  bind:value
  {placeholder}
  {disabled}
  {required}
  class="base-input {className}"
  on:input={oninput}
  {...restProps}
/>

<style>
  .base-input {
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    line-height: 1.25rem;
    background-color: white;
    color: #374151;
    transition: border-color 0.2s ease-in-out, box-shadow 0.2s ease-in-out;
    box-sizing: border-box;
  }

  .base-input:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .base-input:disabled {
    background-color: #f9fafb;
    color: #9ca3af;
    cursor: not-allowed;
    opacity: 0.6;
  }

  .base-input::placeholder {
    color: #9ca3af;
  }
</style>
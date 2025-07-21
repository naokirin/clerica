<script lang="ts">
  interface Props {
    id: string;
    value: string;
    placeholder?: string;
    disabled?: boolean;
    required?: boolean;
    rows?: number;
    cols?: number;
    class?: string;
    monospace?: boolean;
    resize?: 'vertical' | 'horizontal' | 'both' | 'none';
    oninput?: (event: Event) => void;
  }

  let {
    id,
    value = $bindable(),
    placeholder = "",
    disabled = false,
    required = false,
    rows = 3,
    cols,
    class: className = "",
    monospace = false,
    resize = 'vertical',
    oninput,
    ...restProps
  }: Props = $props();
</script>

<textarea
  {id}
  bind:value
  {placeholder}
  {disabled}
  {required}
  {rows}
  {cols}
  class="base-textarea {className}"
  class:monospace={monospace}
  style:resize={resize}
  oninput={oninput}
  {...restProps}
></textarea>

<style>
  .base-textarea {
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
    font-family: inherit;
    resize: vertical;
  }

  .base-textarea:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .base-textarea:disabled {
    background-color: #f9fafb;
    color: #9ca3af;
    cursor: not-allowed;
    opacity: 0.6;
  }

  .base-textarea::placeholder {
    color: #9ca3af;
  }

  .base-textarea.monospace {
    font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
  }
</style>
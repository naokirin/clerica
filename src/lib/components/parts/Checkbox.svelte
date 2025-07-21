<script lang="ts">
  type Size = "small" | "medium" | "large" | "xlarge";

  interface Props {
    checked?: boolean;
    disabled?: boolean;
    label?: string;
    id?: string;
    class?: string;
    size?: Size;
    onchange?: (checked: boolean, event: Event) => void;
  }

  let {
    checked = $bindable(false),
    disabled = false,
    label,
    id,
    class: className = "",
    size = "medium",
    onchange,
    ...restProps
  }: Props = $props();

  // ユニークなIDを生成（指定されていない場合）
  const uniqueId =
    id || `checkbox-${Math.random().toString(36).substring(2, 11)}`;

  const handleChange = (event: Event) => {
    const target = event.target as HTMLInputElement;
    onchange?.(target.checked, event);
  };
</script>

<div class="checkbox-wrapper {size} {className}">
  <input
    type="checkbox"
    {disabled}
    bind:checked
    id={uniqueId}
    class="checkbox-input {size}"
    onchange={handleChange}
    {...restProps}
  />
  {#if label}
    <label for={uniqueId} class="checkbox-label {size}" class:disabled>
      {label}
    </label>
  {/if}
</div>

<style>
  .checkbox-wrapper {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .checkbox-input {
    accent-color: #3b82f6;
    cursor: pointer;
  }

  .checkbox-input:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  .checkbox-label {
    font-weight: 400;
    color: #374151;
    cursor: pointer;
    user-select: none;
    line-height: 1.25rem;
  }

  .checkbox-label.disabled {
    color: #9ca3af;
    cursor: not-allowed;
  }

  .checkbox-input:focus {
    outline: 2px solid #3b82f6;
    outline-offset: 2px;
  }

  .checkbox-input:focus:not(:focus-visible) {
    outline: none;
  }

  /* Size variants */
  .checkbox-input.small {
    width: 0.75rem;
    height: 0.75rem;
  }

  .checkbox-input.medium {
    width: 1rem;
    height: 1rem;
  }

  .checkbox-input.large {
    width: 1.25rem;
    height: 1.25rem;
    transform: scale(1.2);
  }

  .checkbox-input.xlarge {
    width: 1.25rem;
    height: 1.25rem;
    transform: scale(1.4);
  }

  .checkbox-label.small {
    font-size: 0.75rem;
  }

  .checkbox-label.medium {
    font-size: 0.875rem;
  }

  .checkbox-label.large {
    font-size: 1rem;
  }

  .checkbox-label.xlarge {
    font-size: 1.25rem;
  }
</style>

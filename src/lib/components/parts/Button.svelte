<script lang="ts">
  type Variant = "primary" | "secondary" | "danger";
  type Size = "small" | "medium" | "large";

  interface Props {
    variant?: Variant;
    size?: Size;
    disabled?: boolean;
    href?: string;
    type?: "button" | "submit" | "reset";
    class?: string;
    icon?: string; // SVGアイコン文字列
    text?: string; // ボタンテキスト
    onclick?: (event: MouseEvent) => void;
  }

  let {
    variant = "primary",
    size = "medium",
    disabled = false,
    href,
    type = "button",
    class: className = "",
    icon,
    text,
    onclick,
    ...restProps
  }: Props = $props();
</script>

{#if href}
  <a
    {href}
    class="btn {variant} {size} {className}"
    class:disabled
    role="button"
    aria-disabled={disabled}
    {...restProps}
    on:click={onclick}
  >
    {#if icon}
      {@html icon}
    {/if}
    {#if text}
      <span class="text-content">{text}</span>
    {/if}
    <slot name="leading-icon" />
    <span class="text-content">
      <slot />
    </span>
    <slot name="trailing-icon" />
  </a>
{:else}
  <button
    {type}
    {disabled}
    class="btn {variant} {size} {className}"
    {...restProps}
    on:click={onclick}
  >
    {#if icon}
      {@html icon}
    {/if}
    {#if text}
      <span class="text-content">{text}</span>
    {/if}
    <slot name="leading-icon" />
    <span class="text-content">
      <slot />
    </span>
    <slot name="trailing-icon" />
  </button>
{/if}

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    text-decoration: none;
    cursor: pointer;
    transition: background-color 0.2s;
    color: white;
    background-color: #3b82f6;
  }

  .btn:hover:not(:disabled) {
    background-color: #2563eb;
  }

  .btn:disabled {
    background-color: #9ca3af;
    cursor: not-allowed;
    opacity: 0.6;
  }

  /* Variants */
  .secondary {
    background-color: #6b7280;
    color: white;
  }

  .secondary:hover:not(:disabled) {
    background-color: #4b5563;
  }

  .danger {
    background-color: #ef4444;
    color: white;
  }

  .danger:hover:not(:disabled) {
    background-color: #dc2626;
  }

  /* Sizes */
  .small {
    font-size: 0.75rem;
    padding: 0.375rem 0.5rem;
  }

  .large {
    font-size: 1rem;
    padding: 0.625rem 1rem;
  }

  .text-content {
    flex-shrink: 0;
  }

  .btn.disabled {
    background-color: #9ca3af;
    cursor: not-allowed;
    opacity: 0.6;
  }
</style>

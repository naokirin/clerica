<script lang="ts">
  import type { Snippet } from "svelte";
  import * as icons from "@lucide/svelte";

  type Variant = "primary" | "secondary" | "danger" | "neutral" | "success";
  type Size = "small" | "medium" | "large";

  interface Props {
    id: string;
    variant?: Variant;
    size?: Size;
    disabled?: boolean;
    href?: string;
    type?: "button" | "submit" | "reset";
    class?: string;
    iconName?: keyof typeof icons; // @lucide/svelteアイコンコンポーネント
    iconSize?: number; // アイコンのサイズ
    text?: string; // ボタンテキスト
    onclick?: (event: MouseEvent) => void;
    children?: Snippet;
    leadingIcon?: Snippet;
    trailingIcon?: Snippet;
  }

  let {
    id,
    variant = "primary",
    size = "medium",
    disabled = false,
    href,
    type = "button",
    class: className = "",
    iconName,
    iconSize = 16, // デフォルトのアイコンサイズ
    text,
    onclick,
    children,
    leadingIcon,
    trailingIcon,
    ...restProps
  }: Props = $props();

  const Icon = $derived(iconName
    ? (icons[iconName] as any)
    : null);
</script>

{#if href}
  <a
    {id}
    {href}
    class="btn {variant} {size} {className}"
    class:disabled
    role="button"
    aria-disabled={disabled}
    {...restProps}
    onclick={onclick}
  >
    {#if Icon}
      <Icon size={iconSize} />
    {/if}
    {#if text}
      <span class="text-content">{text}</span>
    {/if}
    {#if leadingIcon}
      {@render leadingIcon()}
    {/if}
    {#if children}
      <span class="text-content">
        {@render children()}
      </span>
    {/if}
    {#if trailingIcon}
      {@render trailingIcon()}
    {/if}
  </a>
{:else}
  <button
    {id}
    {type}
    {disabled}
    class="btn {variant} {size} {className}"
    {...restProps}
    onclick={onclick}
  >
    {#if Icon}
      <Icon size={iconSize} />
    {/if}
    {#if text}
      <span class="text-content">{text}</span>
    {/if}
    {#if leadingIcon}
      {@render leadingIcon()}
    {/if}
    {#if children}
      <span class="text-content">
        {@render children()}
      </span>
    {/if}
    {#if trailingIcon}
      {@render trailingIcon()}
    {/if}
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

  .neutral {
    background-color: #f5f5f5;
    color: #333;
    border: 1px solid #ddd;
  }

  .neutral:hover:not(:disabled) {
    background-color: #e0e0e0;
    border-color: #bbb;
  }

  .success {
    background-color: #4caf50;
    color: white;
  }

  .success:hover:not(:disabled) {
    background-color: #45a049;
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

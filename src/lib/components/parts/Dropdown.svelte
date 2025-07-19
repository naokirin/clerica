<script lang="ts">
  import { onMount } from 'svelte';
  import { MoreVertical } from '@lucide/svelte';

  interface Props {
    disabled?: boolean;
    position?: 'left' | 'right';
    triggerClass?: string;
    menuClass?: string;
    children?: import('svelte').Snippet;
  }

  let {
    disabled = false,
    position = 'right',
    triggerClass = '',
    menuClass = '',
    children
  }: Props = $props();

  let isOpen = $state(false);
  let triggerElement: HTMLButtonElement;
  let dropdownElement = $state<HTMLDivElement>();

  const toggleDropdown = () => {
    if (!disabled) {
      isOpen = !isOpen;
    }
  };

  const closeDropdown = () => {
    isOpen = false;
  };

  const handleClickOutside = (event: MouseEvent) => {
    if (
      isOpen &&
      dropdownElement &&
      !dropdownElement.contains(event.target as Node) &&
      !triggerElement.contains(event.target as Node)
    ) {
      closeDropdown();
    }
  };

  const handleKeydown = (event: KeyboardEvent) => {
    if (event.key === 'Escape' && isOpen) {
      closeDropdown();
      triggerElement?.focus();
    }
  };

  onMount(() => {
    document.addEventListener('click', handleClickOutside);
    document.addEventListener('keydown', handleKeydown);

    return () => {
      document.removeEventListener('click', handleClickOutside);
      document.removeEventListener('keydown', handleKeydown);
    };
  });
</script>

<div class="dropdown" class:disabled>
  <button
    bind:this={triggerElement}
    class="dropdown-trigger {triggerClass}"
    onclick={toggleDropdown}
    aria-haspopup="menu"
    aria-expanded={isOpen}
    {disabled}
    type="button"
  >
    <MoreVertical size={16} />
  </button>

  {#if isOpen}
    <div 
      bind:this={dropdownElement}
      class="dropdown-menu {position} {menuClass}"
      role="menu"
      aria-orientation="vertical"
    >
      {#if children}
        {@render children()}
      {/if}
    </div>
  {/if}
</div>

<style>
  .dropdown {
    position: relative;
    display: inline-block;
  }

  .dropdown.disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  .dropdown-trigger {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: none;
    border-radius: 4px;
    background-color: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .dropdown-trigger:hover:not(:disabled) {
    background-color: var(--bg-hover);
    color: var(--text-primary);
  }

  .dropdown-trigger:focus {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }

  .dropdown-trigger:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  .dropdown-menu {
    position: absolute;
    top: 100%;
    min-width: 160px;
    background-color: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    padding: 4px 0;
    z-index: 1000;
    margin-top: 4px;
  }

  .dropdown-menu.left {
    right: 0;
  }

  .dropdown-menu.right {
    left: 0;
  }

  :global(.dropdown-menu-item) {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 8px 12px;
    border: none;
    background: none;
    color: var(--text-primary);
    cursor: pointer;
    font-size: 14px;
    text-align: left;
    transition: background-color 0.2s ease;
    gap: 8px;
  }

  :global(.dropdown-menu-item:hover) {
    background-color: var(--bg-hover);
  }

  :global(.dropdown-menu-item:focus) {
    background-color: var(--bg-hover);
    outline: none;
  }

  :global(.dropdown-menu-item.danger) {
    color: var(--color-danger);
  }

  :global(.dropdown-menu-item.danger:hover) {
    background-color: var(--color-danger);
    color: white;
  }

  :global(.dropdown-menu-item.danger:hover .icon) {
    color: white;
  }

  :global(.dropdown-menu-item:disabled) {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
  }

  :global(.dropdown-menu-item .icon) {
    flex-shrink: 0;
    width: 16px;
    height: 16px;
  }
</style>
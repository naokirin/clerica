<script lang="ts">
  interface Props {
    icon: any;
    title: string;
    onClick: (event: MouseEvent) => void;
    size?: number;
    class?: string;
    disabled?: boolean;
  }

  let {
    icon: Icon,
    title,
    onClick,
    size = 14,
    class: className = "",
    disabled = false,
  }: Props = $props();

  function handleClick(event: MouseEvent) {
    if (disabled) return;
    event.stopPropagation();
    onClick(event);
  }
</script>

<button
  class="icon-button {className}"
  onclick={handleClick}
  {title}
  {disabled}
>
  <Icon {size} />
</button>

<style>
  .icon-button {
    padding: 0.25rem;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.2s;
    padding: 0.25rem;
  }

  .icon-button.green {
    background-color: #10b981;
    color: white;
  }

  .icon-button.green:hover {
    background-color: #059669;
  }

  .icon-button.red {
    background-color: #ef4444;
    color: white;
  }

  .icon-button.red:hover {
    background-color: #dc2626;
  }

  .icon-button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
</style>

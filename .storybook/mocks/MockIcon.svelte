<script lang="ts">
  import * as lucideIcons from '@lucide/svelte';
  import type { SvelteComponent } from 'svelte';

  interface Props {
    size?: number;
    iconName?: string;
  }

  let { size = 16, iconName = 'Circle' }: Props = $props();

  // @lucide/svelte のアイコン名をPascalCaseに変換
  const normalizeIconName = (name: string): string => {
    return name
      .split('-')
      .map(word => word.charAt(0).toUpperCase() + word.slice(1))
      .join('');
  };

  const IconComponent = $derived(() => {
    const normalizedName = normalizeIconName(iconName);
    const icon = (lucideIcons as any)[normalizedName];
    return icon || lucideIcons.Circle;
  });
</script>

<svelte:component this={IconComponent()} {size} />
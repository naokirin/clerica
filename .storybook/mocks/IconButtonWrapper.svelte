<script lang="ts">
  import IconButton from '../../src/lib/components/parts/IconButton.svelte';
  import * as lucideIcons from 'lucide-svelte';

  interface Props {
    iconName?: string;
    title: string;
    onClick?: (event: MouseEvent) => void;
    size?: number;
    class?: string;
    disabled?: boolean;
  }

  let {
    iconName = 'circle',
    title,
    onClick = () => {},
    size = 14,
    class: className = "",
    disabled = false,
  }: Props = $props();

  // lucide-svelte のアイコン名をPascalCaseに変換
  const normalizeIconName = (name: string): string => {
    return name
      .split('-')
      .map(word => word.charAt(0).toUpperCase() + word.slice(1))
      .join('');
  };

  // Reactiveにアイコンコンポーネントを取得
  const IconComponent = $derived(() => {
    const normalizedName = normalizeIconName(iconName);
    const icon = (lucideIcons as any)[normalizedName];
    return icon || lucideIcons.Circle;
  });
</script>

<IconButton
  icon={IconComponent()}
  {title}
  {onClick}
  {size}
  class={className}
  {disabled}
/>
<script lang="ts">
  import RadioButtonGroup from '../lib/components/parts/RadioButtonGroup.svelte';

  interface Option {
    value: string;
    label: string;
  }

  interface Props {
    title: string;
    options: Option[];
    initialValue: string;
    disabled?: boolean;
    onValueChange?: (value: string) => void;
  }

  let { title, options, initialValue, disabled = false, onValueChange }: Props = $props();

  let currentValue = $state(initialValue);

  function handleValueChange(value: string) {
    currentValue = value;
    onValueChange?.(value);
  }
</script>

<RadioButtonGroup
  {title}
  {options}
  value={currentValue}
  {disabled}
  onValueChange={handleValueChange}
/>
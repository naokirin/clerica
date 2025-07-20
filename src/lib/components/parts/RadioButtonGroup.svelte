<script lang="ts">
  interface Option {
    value: string;
    label: string;
  }

  interface Props {
    title: string;
    options: Option[];
    value: string;
    disabled?: boolean;
    onValueChange: (value: string) => void;
  }

  let { title, options, value, disabled = false, onValueChange }: Props = $props();

  function handleRadioChange(event: Event) {
    const target = event.target as HTMLInputElement;
    onValueChange(target.value);
  }
</script>

<div class="radio-group-container">
  <h4>{title}</h4>
  <div class="radio-group">
    {#each options as option}
      <label class="radio-label">
        <input
          type="radio"
          value={option.value}
          checked={value === option.value}
          {disabled}
          onchange={handleRadioChange}
        />
        <span class="radio-text">{option.label}</span>
      </label>
    {/each}
  </div>
</div>

<style>
  .radio-group-container {
    margin-bottom: 1.5rem;
  }

  .radio-group-container h4 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: #495057;
  }

  .radio-group {
    display: flex;
    gap: 1.5rem;
  }

  .radio-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 0.875rem;
    color: #495057;
  }

  .radio-label input[type="radio"] {
    margin: 0;
  }

  .radio-text {
    font-weight: 500;
  }
</style>
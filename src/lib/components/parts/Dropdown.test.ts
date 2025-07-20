import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/svelte';
import Dropdown from './Dropdown.svelte';

describe('Dropdown', () => {
  it('renders without errors', () => {
    const { container } = render(Dropdown);
    expect(container).toBeTruthy();
  });

  it('renders with disabled state', () => {
    const { container } = render(Dropdown, {
      props: { disabled: true }
    });
    expect(container).toBeTruthy();
  });
});
import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/svelte';
import IconButton from './IconButton.svelte';

// Mock icon component
const MockIcon = () => 'mock-icon';

describe('IconButton', () => {
  it('renders without errors', () => {
    const { container } = render(IconButton, {
      props: {
        icon: MockIcon,
        title: 'Test Icon Button',
        onClick: vi.fn()
      }
    });
    expect(container).toBeTruthy();
  });

  it('renders with custom class', () => {
    const { container } = render(IconButton, {
      props: {
        icon: MockIcon,
        title: 'Test Button',
        onClick: vi.fn(),
        class: 'custom-class'
      }
    });
    expect(container).toBeTruthy();
  });
});
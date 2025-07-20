import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/svelte';
import Snackbar from './Snackbar.svelte';

// Mock i18n
vi.mock('$lib/i18n', () => ({
  t: vi.fn((key: string) => key)
}));

// Mock svelte/transition
vi.mock('svelte/transition', () => ({
  fly: () => ({})
}));

// Mock the error store to return empty errors by default
vi.mock('../../stores/error', () => ({
  errorStore: {
    errors: { subscribe: vi.fn((callback) => callback([])) },
    dismissError: vi.fn()
  }
}));

describe('Snackbar', () => {
  it('renders without errors', () => {
    const { container } = render(Snackbar);
    expect(container).toBeTruthy();
  });
});
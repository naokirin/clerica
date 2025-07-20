import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/svelte';
import SimpleLoadingScreen from './SimpleLoadingScreen.svelte';

// Mock i18n
vi.mock('$lib/i18n', () => ({
  t: vi.fn((key: string) => key)
}));

describe('SimpleLoadingScreen', () => {
  it('renders without errors', () => {
    const { container } = render(SimpleLoadingScreen);
    expect(container).toBeTruthy();
  });

  it('renders with custom message', () => {
    const { container } = render(SimpleLoadingScreen, {
      props: { message: 'Loading...' }
    });
    expect(container).toBeTruthy();
  });
});
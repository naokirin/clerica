import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/svelte';
import Button from './Button.svelte';

describe('Button', () => {
  it('renders without errors', () => {
    const { container } = render(Button, {
      props: { text: 'Click me' }
    });
    expect(container).toBeTruthy();
  });

  it('renders with href as link', () => {
    const { container } = render(Button, {
      props: { href: '/test', text: 'Link' }
    });
    expect(container).toBeTruthy();
  });

  it('renders with icon', () => {
    const { container } = render(Button, {
      props: { iconName: 'Plus', text: 'Add' }
    });
    expect(container).toBeTruthy();
  });
});
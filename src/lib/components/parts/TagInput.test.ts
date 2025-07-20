import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import TagInput from './TagInput.svelte';
import type { Tag } from '../../types';

// Mock i18n
vi.mock('$lib/i18n', () => ({
  t: vi.fn((key: string, params?: any) => {
    if (key === 'common.tags.removeAriaLabel' && params) {
      return `Remove tag ${params.name}`;
    }
    return key;
  })
}));

const createMockTag = (overrides: Partial<Tag> = {}): Tag => ({
  id: 'tag1',
  name: 'Test Tag',
  color: '#3B82F6',
  created_at: new Date().toISOString(),
  ...overrides
});

describe('TagInput', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Mock crypto.randomUUID if not available
    if (!global.crypto) {
      global.crypto = { randomUUID: vi.fn(() => 'mock-uuid') } as any;
    }
  });

  it('renders input field with placeholder', () => {
    render(TagInput, {
      tags: [],
      placeholder: 'Add tags...'
    });
    
    expect(screen.getByPlaceholderText('Add tags...')).toBeInTheDocument();
  });

  it('displays existing tags', () => {
    const tags = [
      createMockTag({ name: 'Tag 1' }),
      createMockTag({ id: 'tag2', name: 'Tag 2', color: '#EF4444' })
    ];
    
    render(TagInput, { tags });
    
    expect(screen.getByText('Tag 1')).toBeInTheDocument();
    expect(screen.getByText('Tag 2')).toBeInTheDocument();
  });

  it('adds new tag on Enter key press', async () => {
    const onchange = vi.fn();
    render(TagInput, {
      tags: [],
      onchange
    });
    
    const input = screen.getByRole('textbox');
    await fireEvent.input(input, { target: { value: 'New Tag' } });
    await fireEvent.keyDown(input, { key: 'Enter' });
    
    expect(onchange).toHaveBeenCalledWith(
      expect.arrayContaining([
        expect.objectContaining({
          name: 'New Tag',
          color: '#3B82F6'
        })
      ])
    );
  });

  it('adds new tag on input blur', async () => {
    const onchange = vi.fn();
    render(TagInput, {
      tags: [],
      onchange
    });
    
    const input = screen.getByRole('textbox');
    await fireEvent.input(input, { target: { value: 'Blur Tag' } });
    await fireEvent.blur(input);
    
    expect(onchange).toHaveBeenCalledWith(
      expect.arrayContaining([
        expect.objectContaining({
          name: 'Blur Tag'
        })
      ])
    );
  });

  it('does not add empty tags', async () => {
    const onchange = vi.fn();
    render(TagInput, {
      tags: [],
      onchange
    });
    
    const input = screen.getByRole('textbox');
    await fireEvent.input(input, { target: { value: '   ' } });
    await fireEvent.keyDown(input, { key: 'Enter' });
    
    expect(onchange).not.toHaveBeenCalled();
  });

  it('prevents duplicate tags (case insensitive)', async () => {
    const existingTags = [createMockTag({ name: 'Existing Tag' })];
    const onchange = vi.fn();
    
    render(TagInput, {
      tags: existingTags,
      onchange
    });
    
    const input = screen.getByRole('textbox');
    await fireEvent.input(input, { target: { value: 'EXISTING TAG' } });
    await fireEvent.keyDown(input, { key: 'Enter' });
    
    expect(onchange).not.toHaveBeenCalled();
  });

  it('removes tag when remove button is clicked', async () => {
    const tags = [
      createMockTag({ id: 'tag1', name: 'Tag 1' }),
      createMockTag({ id: 'tag2', name: 'Tag 2' })
    ];
    const onchange = vi.fn();
    
    render(TagInput, {
      tags,
      onchange
    });
    
    const removeButtons = screen.getAllByRole('button');
    await fireEvent.click(removeButtons[0]);
    
    expect(onchange).toHaveBeenCalledWith([
      expect.objectContaining({ id: 'tag2', name: 'Tag 2' })
    ]);
  });

  it('has correct accessibility attributes for remove buttons', () => {
    const tags = [createMockTag({ name: 'Test Tag' })];
    
    render(TagInput, { tags });
    
    const removeButton = screen.getByRole('button');
    expect(removeButton).toHaveAttribute('aria-label', 'Remove tag Test Tag');
  });

  it('handles disabled state correctly', () => {
    const tags = [createMockTag()];
    
    render(TagInput, {
      tags,
      disabled: true
    });
    
    const input = screen.getByRole('textbox');
    const removeButton = screen.getByRole('button');
    
    expect(input).toBeDisabled();
    expect(removeButton).toBeDisabled();
  });

  it('clears input after adding tag', async () => {
    const onchange = vi.fn();
    render(TagInput, {
      tags: [],
      onchange
    });
    
    const input = screen.getByRole('textbox') as HTMLInputElement;
    await fireEvent.input(input, { target: { value: 'New Tag' } });
    await fireEvent.keyDown(input, { key: 'Enter' });
    
    expect(input.value).toBe('');
  });

  it('handles composition events for Japanese input', async () => {
    const onchange = vi.fn();
    render(TagInput, {
      tags: [],
      onchange
    });
    
    const input = screen.getByRole('textbox');
    
    // Start composition
    await fireEvent.compositionStart(input);
    await fireEvent.input(input, { target: { value: 'テスト' } });
    await fireEvent.keyDown(input, { key: 'Enter' });
    
    // Should not add tag during composition
    expect(onchange).not.toHaveBeenCalled();
    
    // End composition
    await fireEvent.compositionEnd(input);
    
    // Wait for timeout
    await new Promise(resolve => setTimeout(resolve, 20));
    
    // Now Enter should work
    await fireEvent.keyDown(input, { key: 'Enter' });
    expect(onchange).toHaveBeenCalled();
  });

  it('trims whitespace from tag names', async () => {
    const onchange = vi.fn();
    render(TagInput, {
      tags: [],
      onchange
    });
    
    const input = screen.getByRole('textbox');
    await fireEvent.input(input, { target: { value: '  Trimmed Tag  ' } });
    await fireEvent.keyDown(input, { key: 'Enter' });
    
    expect(onchange).toHaveBeenCalledWith(
      expect.arrayContaining([
        expect.objectContaining({
          name: 'Trimmed Tag'
        })
      ])
    );
  });

  it('applies custom tag colors', () => {
    const tags = [
      createMockTag({ name: 'Red Tag', color: '#EF4444' })
    ];
    
    const { container } = render(TagInput, { tags });
    
    const tagItem = container.querySelector('.tag-item');
    expect(tagItem).toHaveStyle('background-color: #EF444420');
    expect(tagItem).toHaveStyle('border-color: #EF4444');
  });

  it('generates unique temporary IDs for new tags', async () => {
    const onchange = vi.fn();
    render(TagInput, {
      tags: [],
      onchange
    });
    
    const input = screen.getByRole('textbox');
    
    // Add first tag
    await fireEvent.input(input, { target: { value: 'Tag 1' } });
    await fireEvent.keyDown(input, { key: 'Enter' });
    
    // Add second tag
    await fireEvent.input(input, { target: { value: 'Tag 2' } });
    await fireEvent.keyDown(input, { key: 'Enter' });
    
    expect(onchange).toHaveBeenCalledTimes(2);
    
    // Check that IDs are different
    const firstCall = onchange.mock.calls[0][0];
    const secondCall = onchange.mock.calls[1][0];
    
    expect(firstCall[0].id).not.toBe(secondCall[1].id);
  });

  it('preserves existing tags when adding new ones', async () => {
    const existingTags = [createMockTag({ name: 'Existing' })];
    const onchange = vi.fn();
    
    render(TagInput, {
      tags: existingTags,
      onchange
    });
    
    const input = screen.getByRole('textbox');
    await fireEvent.input(input, { target: { value: 'New' } });
    await fireEvent.keyDown(input, { key: 'Enter' });
    
    expect(onchange).toHaveBeenCalledWith([
      existingTags[0],
      expect.objectContaining({ name: 'New' })
    ]);
  });

  it('uses default placeholder when none provided', () => {
    render(TagInput, { tags: [] });
    
    expect(screen.getByPlaceholderText('common.tags.inputPlaceholder')).toBeInTheDocument();
  });
});
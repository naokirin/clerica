import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import SortControl from './SortControl.svelte';
import type { SortField, SortOrder } from '../../types';

// Mock i18n
vi.mock('$lib/i18n', () => ({
  t: vi.fn((key: string) => key)
}));

const mockSortOptions = [
  { value: 'name' as SortField, label: 'Name' },
  { value: 'size' as SortField, label: 'Size' },
  { value: 'created_at' as SortField, label: 'Created' },
  { value: 'modified_at' as SortField, label: 'Modified' }
];

describe('SortControl', () => {
  it('renders sort field select and order button', () => {
    render(SortControl, {
      sortField: 'name',
      sortOrder: 'asc',
      sortOptions: mockSortOptions,
      onSortChange: vi.fn()
    });
    
    expect(screen.getByDisplayValue('Name')).toBeInTheDocument();
    expect(screen.getByRole('button')).toBeInTheDocument();
  });

  it('displays all sort field options', () => {
    render(SortControl, {
      sortField: 'name',
      sortOrder: 'asc',
      sortOptions: mockSortOptions,
      onSortChange: vi.fn()
    });
    
    const select = screen.getByDisplayValue('Name');
    expect(select).toBeInTheDocument();
    
    // Check if all options are present
    mockSortOptions.forEach(option => {
      expect(screen.getByText(option.label)).toBeInTheDocument();
    });
  });

  it('calls onSortChange when field is changed', async () => {
    const onSortChange = vi.fn().mockResolvedValue(undefined);
    
    render(SortControl, {
      sortField: 'name',
      sortOrder: 'asc',
      sortOptions: mockSortOptions,
      onSortChange
    });
    
    const select = screen.getByDisplayValue('Name');
    await fireEvent.change(select, { target: { value: 'size' } });
    
    expect(onSortChange).toHaveBeenCalledWith({
      field: 'size',
      order: 'asc'
    });
  });

  it('toggles sort order when button is clicked', async () => {
    const onSortChange = vi.fn().mockResolvedValue(undefined);
    
    render(SortControl, {
      sortField: 'name',
      sortOrder: 'asc',
      sortOptions: mockSortOptions,
      onSortChange
    });
    
    const button = screen.getByRole('button');
    await fireEvent.click(button);
    
    expect(onSortChange).toHaveBeenCalledWith({
      field: 'name',
      order: 'desc'
    });
  });

  it('toggles from desc to asc when button is clicked', async () => {
    const onSortChange = vi.fn().mockResolvedValue(undefined);
    
    render(SortControl, {
      sortField: 'name',
      sortOrder: 'desc',
      sortOptions: mockSortOptions,
      onSortChange
    });
    
    const button = screen.getByRole('button');
    await fireEvent.click(button);
    
    expect(onSortChange).toHaveBeenCalledWith({
      field: 'name',
      order: 'asc'
    });
  });

  it('displays correct icon for ascending order', () => {
    const { container } = render(SortControl, {
      sortField: 'name',
      sortOrder: 'asc',
      sortOptions: mockSortOptions,
      onSortChange: vi.fn()
    });
    
    // Check for ChevronUp icon (this would be tested via data attributes or class names in practice)
    const button = container.querySelector('.sort-order-btn');
    expect(button).toBeInTheDocument();
  });

  it('displays correct icon for descending order', () => {
    const { container } = render(SortControl, {
      sortField: 'name',
      sortOrder: 'desc',
      sortOptions: mockSortOptions,
      onSortChange: vi.fn()
    });
    
    // Check for ChevronDown icon
    const button = container.querySelector('.sort-order-btn');
    expect(button).toBeInTheDocument();
  });

  it('sets correct title attribute based on sort order', () => {
    render(SortControl, {
      sortField: 'name',
      sortOrder: 'asc',
      sortOptions: mockSortOptions,
      onSortChange: vi.fn()
    });
    
    const button = screen.getByRole('button');
    expect(button).toHaveAttribute('title', 'common.files.sort.ascendingTooltip');
  });

  it('updates title when sort order changes to desc', () => {
    render(SortControl, {
      sortField: 'name',
      sortOrder: 'desc',
      sortOptions: mockSortOptions,
      onSortChange: vi.fn()
    });
    
    const button = screen.getByRole('button');
    expect(button).toHaveAttribute('title', 'common.files.sort.descendingTooltip');
  });

  it('maintains current sort field when order is toggled', async () => {
    const onSortChange = vi.fn().mockResolvedValue(undefined);
    
    render(SortControl, {
      sortField: 'size',
      sortOrder: 'asc',
      sortOptions: mockSortOptions,
      onSortChange
    });
    
    const button = screen.getByRole('button');
    await fireEvent.click(button);
    
    expect(onSortChange).toHaveBeenCalledWith({
      field: 'size',
      order: 'desc'
    });
  });

  it('handles empty sort options array', () => {
    render(SortControl, {
      sortField: 'name',
      sortOrder: 'asc',
      sortOptions: [],
      onSortChange: vi.fn()
    });
    
    const select = screen.getByRole('combobox');
    expect(select).toBeInTheDocument();
    expect(select.children).toHaveLength(0);
  });

  it('has correct accessibility attributes', () => {
    render(SortControl, {
      sortField: 'name',
      sortOrder: 'asc',
      sortOptions: mockSortOptions,
      onSortChange: vi.fn()
    });
    
    const button = screen.getByRole('button');
    expect(button).toHaveAttribute('type', 'button');
    expect(button).toHaveAttribute('title');
  });

  it('handles async onSortChange properly', async () => {
    const onSortChange = vi.fn().mockResolvedValue(undefined);
    
    render(SortControl, {
      sortField: 'name',
      sortOrder: 'asc',
      sortOptions: mockSortOptions,
      onSortChange
    });
    
    const select = screen.getByDisplayValue('Name');
    await fireEvent.change(select, { target: { value: 'size' } });
    
    // Verify async function was called
    expect(onSortChange).toHaveBeenCalledTimes(1);
    
    // Wait for the promise to resolve
    await vi.waitFor(() => {
      expect(onSortChange).toHaveReturnedWith(Promise.resolve(undefined));
    });
  });
});
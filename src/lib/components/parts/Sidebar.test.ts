import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import Sidebar from './Sidebar.svelte';
import type { Directory, Tag as TagType } from '../../types';

// Mock i18n
vi.mock('$lib/i18n', () => ({
  t: vi.fn((key: string) => key)
}));

// Mock ShelfManager component
vi.mock('../modules/settings/ShelfManager.svelte', () => ({
  default: () => 'ShelfManager'
}));

const createMockDirectory = (overrides: Partial<Directory> = {}): Directory => ({
  id: 'dir1',
  name: 'Test Directory',
  path: '/test/path',
  created_at: new Date().toISOString(),
  updated_at: new Date().toISOString(),
  ...overrides
});

const createMockTag = (overrides: Partial<TagType> = {}): TagType => ({
  id: 'tag1',
  name: 'Test Tag',
  color: '#3B82F6',
  created_at: new Date().toISOString(),
  ...overrides
});

describe('Sidebar', () => {
  const defaultProps = {
    directories: [],
    tags: [],
    selectedDirectoryId: 'all' as const,
    onAddDirectory: vi.fn(),
    onSelectDirectory: vi.fn(),
    onRescanDirectory: vi.fn(),
    onRemoveDirectory: vi.fn(),
    onCreateTag: vi.fn(),
    onRescanAll: vi.fn()
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders sidebar with all sections', () => {
    render(Sidebar, defaultProps);
    
    expect(screen.getByText('common.sidebar.directories')).toBeInTheDocument();
    expect(screen.getByText('common.sidebar.addDirectory')).toBeInTheDocument();
  });

  it('renders "All Files" option', () => {
    render(Sidebar, defaultProps);
    
    expect(screen.getByText('common.sidebar.allFiles')).toBeInTheDocument();
    expect(screen.getByText('common.sidebar.allFilesDescription')).toBeInTheDocument();
  });

  it('highlights selected directory', () => {
    const directories = [createMockDirectory()];
    const { container } = render(Sidebar, {
      ...defaultProps,
      directories,
      selectedDirectoryId: 'dir1'
    });
    
    const selectedItem = container.querySelector('.directory-item.selected');
    expect(selectedItem).toBeInTheDocument();
    expect(selectedItem).toHaveTextContent('Test Directory');
  });

  it('highlights "All Files" when selected', () => {
    const { container } = render(Sidebar, {
      ...defaultProps,
      selectedDirectoryId: 'all'
    });
    
    const selectedItem = container.querySelector('.directory-item.selected');
    expect(selectedItem).toBeInTheDocument();
    expect(selectedItem).toHaveTextContent('common.sidebar.allFiles');
  });

  it('calls onAddDirectory when add button is clicked', async () => {
    const onAddDirectory = vi.fn();
    render(Sidebar, {
      ...defaultProps,
      onAddDirectory
    });
    
    const addButton = screen.getByText('common.sidebar.addDirectory');
    await fireEvent.click(addButton);
    
    expect(onAddDirectory).toHaveBeenCalledTimes(1);
  });

  it('calls onSelectDirectory when directory is clicked', async () => {
    const directories = [createMockDirectory()];
    const onSelectDirectory = vi.fn();
    
    render(Sidebar, {
      ...defaultProps,
      directories,
      onSelectDirectory
    });
    
    const directoryItem = screen.getByText('Test Directory');
    await fireEvent.click(directoryItem.closest('.directory-item')!);
    
    expect(onSelectDirectory).toHaveBeenCalledWith('dir1');
  });

  it('calls onSelectDirectory when "All Files" is clicked', async () => {
    const onSelectDirectory = vi.fn();
    
    render(Sidebar, {
      ...defaultProps,
      onSelectDirectory
    });
    
    const allFilesItem = screen.getByText('common.sidebar.allFiles');
    await fireEvent.click(allFilesItem.closest('.directory-item')!);
    
    expect(onSelectDirectory).toHaveBeenCalledWith('all');
  });

  it('calls onRescanAll when rescan all button is clicked', async () => {
    const onRescanAll = vi.fn();
    
    render(Sidebar, {
      ...defaultProps,
      onRescanAll
    });
    
    const rescanButton = screen.getByTitle('common.sidebar.rescanAll');
    await fireEvent.click(rescanButton);
    
    expect(onRescanAll).toHaveBeenCalledTimes(1);
  });

  it('calls onRescanDirectory when directory rescan button is clicked', async () => {
    const directories = [createMockDirectory()];
    const onRescanDirectory = vi.fn();
    
    render(Sidebar, {
      ...defaultProps,
      directories,
      onRescanDirectory
    });
    
    const rescanButton = screen.getByTitle('common.sidebar.rescan');
    await fireEvent.click(rescanButton);
    
    expect(onRescanDirectory).toHaveBeenCalledWith('dir1');
  });

  it('calls onRemoveDirectory when remove button is clicked', async () => {
    const directories = [createMockDirectory()];
    const onRemoveDirectory = vi.fn();
    
    render(Sidebar, {
      ...defaultProps,
      directories,
      onRemoveDirectory
    });
    
    const removeButton = screen.getByTitle('common.sidebar.remove');
    await fireEvent.click(removeButton);
    
    expect(onRemoveDirectory).toHaveBeenCalledWith('dir1', 'Test Directory');
  });

  it('renders multiple directories', () => {
    const directories = [
      createMockDirectory({ id: 'dir1', name: 'Directory 1', path: '/path1' }),
      createMockDirectory({ id: 'dir2', name: 'Directory 2', path: '/path2' })
    ];
    
    render(Sidebar, {
      ...defaultProps,
      directories
    });
    
    expect(screen.getByText('Directory 1')).toBeInTheDocument();
    expect(screen.getByText('Directory 2')).toBeInTheDocument();
    expect(screen.getByText('/path1')).toBeInTheDocument();
    expect(screen.getByText('/path2')).toBeInTheDocument();
  });

  it('displays directory name and path correctly', () => {
    const directories = [createMockDirectory({
      name: 'My Documents',
      path: '/Users/test/Documents'
    })];
    
    render(Sidebar, {
      ...defaultProps,
      directories
    });
    
    expect(screen.getByText('My Documents')).toBeInTheDocument();
    expect(screen.getByText('/Users/test/Documents')).toBeInTheDocument();
  });

  it('applies disabled state correctly', () => {
    const { container } = render(Sidebar, {
      ...defaultProps,
      disabled: true
    });
    
    expect(container.querySelector('.sidebar')).toHaveClass('disabled');
  });

  it('has proper accessibility structure', () => {
    const directories = [createMockDirectory()];
    
    render(Sidebar, {
      ...defaultProps,
      directories
    });
    
    expect(screen.getByRole('heading', { name: 'common.sidebar.directories' })).toBeInTheDocument();
  });

  it('renders directory actions for each directory', () => {
    const directories = [createMockDirectory()];
    
    render(Sidebar, {
      ...defaultProps,
      directories
    });
    
    expect(screen.getByTitle('common.sidebar.rescan')).toBeInTheDocument();
    expect(screen.getByTitle('common.sidebar.remove')).toBeInTheDocument();
  });

  it('has correct button styling classes', () => {
    const directories = [createMockDirectory()];
    const { container } = render(Sidebar, {
      ...defaultProps,
      directories
    });
    
    // Check for green class on rescan buttons
    const rescanButtons = container.querySelectorAll('.green');
    expect(rescanButtons.length).toBeGreaterThan(0);
    
    // Check for red class on remove button
    const removeButton = container.querySelector('.red');
    expect(removeButton).toBeInTheDocument();
  });

  it('handles empty directories list', () => {
    render(Sidebar, {
      ...defaultProps,
      directories: []
    });
    
    // Should still show "All Files" option
    expect(screen.getByText('common.sidebar.allFiles')).toBeInTheDocument();
    
    // Should not show any individual directories
    expect(screen.queryByText('/test/path')).not.toBeInTheDocument();
  });

  it('passes appViewModel to ShelfManager', () => {
    const mockAppViewModel = {} as any;
    
    render(Sidebar, {
      ...defaultProps,
      appViewModel: mockAppViewModel
    });
    
    // ShelfManager should be rendered (mocked as text)
    expect(screen.getByText('ShelfManager')).toBeInTheDocument();
  });

  it('prevents interactions when disabled', () => {
    const { container } = render(Sidebar, {
      ...defaultProps,
      disabled: true
    });
    
    const sidebar = container.querySelector('.sidebar');
    expect(sidebar).toHaveStyle('pointer-events: none');
    expect(sidebar).toHaveStyle('opacity: 0.7');
  });
});
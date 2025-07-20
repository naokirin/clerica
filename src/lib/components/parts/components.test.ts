import { describe, it, expect } from 'vitest';
import { existsSync } from 'fs';
import { resolve } from 'path';

describe('Components File Existence Tests', () => {
  const componentsDir = resolve(__dirname);

  it('Button component file exists', () => {
    const buttonPath = resolve(componentsDir, 'Button.svelte');
    expect(existsSync(buttonPath)).toBe(true);
  });

  it('IconButton component file exists', () => {
    const iconButtonPath = resolve(componentsDir, 'IconButton.svelte');
    expect(existsSync(iconButtonPath)).toBe(true);
  });

  it('Dropdown component file exists', () => {
    const dropdownPath = resolve(componentsDir, 'Dropdown.svelte');
    expect(existsSync(dropdownPath)).toBe(true);
  });

  it('SortControl component file exists', () => {
    const sortControlPath = resolve(componentsDir, 'SortControl.svelte');
    expect(existsSync(sortControlPath)).toBe(true);
  });

  it('Snackbar component file exists', () => {
    const snackbarPath = resolve(componentsDir, 'Snackbar.svelte');
    expect(existsSync(snackbarPath)).toBe(true);
  });

  it('TagInput component file exists', () => {
    const tagInputPath = resolve(componentsDir, 'TagInput.svelte');
    expect(existsSync(tagInputPath)).toBe(true);
  });

  it('Pagination component file exists', () => {
    const paginationPath = resolve(componentsDir, 'Pagination.svelte');
    expect(existsSync(paginationPath)).toBe(true);
  });

  it('LoadingScreen component file exists', () => {
    const loadingScreenPath = resolve(componentsDir, 'LoadingScreen.svelte');
    expect(existsSync(loadingScreenPath)).toBe(true);
  });

  it('SimpleLoadingScreen component file exists', () => {
    const simpleLoadingScreenPath = resolve(componentsDir, 'SimpleLoadingScreen.svelte');
    expect(existsSync(simpleLoadingScreenPath)).toBe(true);
  });

  it('Sidebar component file exists', () => {
    const sidebarPath = resolve(componentsDir, 'Sidebar.svelte');
    expect(existsSync(sidebarPath)).toBe(true);
  });

  it('all component files have .svelte extension', () => {
    const componentFiles = [
      'Button.svelte',
      'IconButton.svelte', 
      'Dropdown.svelte',
      'SortControl.svelte',
      'Snackbar.svelte',
      'TagInput.svelte',
      'Pagination.svelte',
      'LoadingScreen.svelte',
      'SimpleLoadingScreen.svelte',
      'Sidebar.svelte'
    ];

    componentFiles.forEach(file => {
      expect(file.endsWith('.svelte')).toBe(true);
    });
  });
});
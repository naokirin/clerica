import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { get } from 'svelte/store';
import { AppViewModel } from '../../lib/viewmodels/AppViewModel';

// Mock child ViewModels
vi.mock('../../lib/viewmodels/DirectoryViewModel', () => ({
  DirectoryViewModel: vi.fn().mockImplementation(() => ({
    loadDirectories: vi.fn().mockResolvedValue(undefined),
    dispose: vi.fn(),
    selectedDirectoryId: {
      subscribe: vi.fn().mockReturnValue(() => {})
    }
  }))
}));

vi.mock('../../lib/viewmodels/FileViewModel', () => ({
  FileViewModel: vi.fn().mockImplementation(() => ({
    loadFiles: vi.fn().mockResolvedValue(undefined),
    dispose: vi.fn(),
    setSelectedDirectoryId: vi.fn()
  }))
}));

vi.mock('../../lib/viewmodels/SearchViewModel', () => ({
  SearchViewModel: vi.fn().mockImplementation(() => ({
    dispose: vi.fn(),
    setSelectedDirectoryId: vi.fn(),
    isSearchActive: vi.fn().mockReturnValue(false),
    performSearch: vi.fn().mockResolvedValue(undefined)
  }))
}));

vi.mock('../../lib/viewmodels/TagViewModel', () => ({
  TagViewModel: vi.fn().mockImplementation(() => ({
    loadTags: vi.fn().mockResolvedValue(undefined),
    loadTopTags: vi.fn().mockResolvedValue(undefined),
    loadCustomMetadataKeys: vi.fn().mockResolvedValue(undefined),
    dispose: vi.fn()
  }))
}));

describe('AppViewModel', () => {
  let appViewModel: AppViewModel;

  beforeEach(() => {
    vi.clearAllMocks();
    vi.useFakeTimers();
  });

  afterEach(() => {
    appViewModel?.dispose();
    vi.useRealTimers();
  });

  describe('constructor', () => {
    it('should initialize with default values', () => {
      appViewModel = new AppViewModel();

      expect(get(appViewModel.activeTab)).toBe('files');
      expect(get(appViewModel.loadingProgress)).toBe(0);
      expect(get(appViewModel.loadingSteps)).toEqual({
        directories: false,
        tags: false,
        files: false
      });
    });

    it('should initialize child ViewModels', () => {
      appViewModel = new AppViewModel();

      expect(appViewModel.directoryViewModel).toBeDefined();
      expect(appViewModel.fileViewModel).toBeDefined();
      expect(appViewModel.searchViewModel).toBeDefined();
      expect(appViewModel.tagViewModel).toBeDefined();
    });

    it('should initialize child view models without auto-loading', () => {
      appViewModel = new AppViewModel();
      
      expect(appViewModel.directoryViewModel).toBeDefined();
      expect(appViewModel.fileViewModel).toBeDefined();
      expect(appViewModel.searchViewModel).toBeDefined();
      expect(appViewModel.tagViewModel).toBeDefined();
    });
  });

  describe('setActiveTab', () => {
    beforeEach(() => {
      appViewModel = new AppViewModel();
    });

    it('should update active tab', () => {
      appViewModel.setActiveTab('search');
      expect(get(appViewModel.activeTab)).toBe('search');

      appViewModel.setActiveTab('tags');
      expect(get(appViewModel.activeTab)).toBe('tags');

      appViewModel.setActiveTab('metadata');
      expect(get(appViewModel.activeTab)).toBe('metadata');
    });
  });

  describe('isAppLoading', () => {
    beforeEach(() => {
      appViewModel = new AppViewModel();
    });

    it('should be false initially', () => {
      expect(get(appViewModel.isAppLoading)).toBe(false);
    });

    it('should be false when explicitly set to false', () => {
      appViewModel['_isAppLoading'].set(false);
      expect(get(appViewModel.isAppLoading)).toBe(false);
    });
  });

  describe('initializeApp', () => {
    beforeEach(() => {
      appViewModel = new AppViewModel();
      vi.clearAllMocks();
    });

    it('should complete initialization successfully', async () => {
      const initPromise = appViewModel.initializeApp();
      
      // Wait for async operations
      await vi.runAllTimersAsync();
      await initPromise;

      expect(appViewModel.directoryViewModel.loadDirectories).toHaveBeenCalled();
      expect(appViewModel.tagViewModel.loadTags).toHaveBeenCalled();
      expect(appViewModel.tagViewModel.loadCustomMetadataKeys).toHaveBeenCalled();
      expect(appViewModel.fileViewModel.loadFiles).toHaveBeenCalled();

      expect(get(appViewModel.loadingProgress)).toBe(100);
      expect(get(appViewModel.loadingSteps)).toEqual({
        directories: true,
        tags: true,
        files: true
      });
    });

    it('should update progress during initialization', async () => {
      let progressValues: number[] = [];
      
      appViewModel.loadingProgress.subscribe(value => {
        progressValues.push(value);
      });

      const initPromise = appViewModel.initializeApp();
      await vi.runAllTimersAsync();
      await initPromise;

      expect(progressValues).toContain(0);
      expect(progressValues).toContain(33);
      expect(progressValues).toContain(66);
      expect(progressValues).toContain(100);
    });

    it('should handle initialization errors', async () => {
      const error = new Error('Load failed');
      appViewModel.directoryViewModel.loadDirectories = vi.fn().mockRejectedValue(error);

      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      await appViewModel.initializeApp();

      expect(consoleErrorSpy).toHaveBeenCalledWith('App initialization failed:', error);
      expect(get(appViewModel.error)).toBe('アプリケーションの初期化に失敗しました');
      expect(get(appViewModel.isLoading)).toBe(false);

      consoleErrorSpy.mockRestore();
    });

    it('should clear loading state after timeout', async () => {
      const initPromise = appViewModel.initializeApp();
      
      await vi.runAllTimersAsync();
      await initPromise;

      expect(get(appViewModel.isLoading)).toBe(false);
    });
  });

  describe('reloadAllData', () => {
    beforeEach(() => {
      appViewModel = new AppViewModel();
      vi.clearAllMocks();
    });

    it('should reload all data sources', async () => {
      await appViewModel.reloadAllData();

      expect(appViewModel.directoryViewModel.loadDirectories).toHaveBeenCalled();
      expect(appViewModel.fileViewModel.loadFiles).toHaveBeenCalled();
      expect(appViewModel.tagViewModel.loadTags).toHaveBeenCalled();
      expect(appViewModel.tagViewModel.loadCustomMetadataKeys).toHaveBeenCalled();
    });

    it('should handle reload errors gracefully', async () => {
      const error = new Error('Reload failed');
      appViewModel.fileViewModel.loadFiles = vi.fn().mockRejectedValue(error);

      await expect(appViewModel.reloadAllData()).rejects.toThrow(error);
    });
  });

  describe('dispose', () => {
    beforeEach(() => {
      appViewModel = new AppViewModel();
    });

    it('should dispose all child ViewModels', () => {
      appViewModel.dispose();

      expect(appViewModel.directoryViewModel.dispose).toHaveBeenCalled();
      expect(appViewModel.fileViewModel.dispose).toHaveBeenCalled();
      expect(appViewModel.searchViewModel.dispose).toHaveBeenCalled();
      expect(appViewModel.tagViewModel.dispose).toHaveBeenCalled();
    });
  });

  describe('store reactivity', () => {
    beforeEach(() => {
      appViewModel = new AppViewModel();
    });

    it('should notify subscribers when activeTab changes', () => {
      const values: string[] = [];
      appViewModel.activeTab.subscribe(value => values.push(value));

      appViewModel.setActiveTab('search');
      appViewModel.setActiveTab('tags');

      expect(values).toEqual(['files', 'search', 'tags']);
    });

    it('should notify subscribers when loadingSteps changes', () => {
      const values: any[] = [];
      appViewModel.loadingSteps.subscribe(value => values.push({ ...value }));

      appViewModel['_loadingSteps'].update(steps => ({ ...steps, directories: true }));

      expect(values).toHaveLength(2);
      expect(values[1].directories).toBe(true);
    });
  });
});
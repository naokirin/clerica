import { writable, derived, type Writable, type Readable } from 'svelte/store';
import { BaseViewModel } from './BaseViewModel.js';
import { DirectoryViewModel } from './DirectoryViewModel.js';
import { FileViewModel } from './FileViewModel.js';
import { SearchViewModel } from './SearchViewModel.js';
import { TagViewModel } from './TagViewModel.js';
import type { LoadingSteps } from '../types.js';

export type ActiveTab = "files" | "search" | "tags" | "metadata";

export class AppViewModel extends BaseViewModel {
  // 子ViewModelインスタンス
  public readonly directoryViewModel: DirectoryViewModel;
  public readonly fileViewModel: FileViewModel;
  public readonly searchViewModel: SearchViewModel;
  public readonly tagViewModel: TagViewModel;

  // アプリケーション全体の状態
  private _activeTab: Writable<ActiveTab> = writable("files");
  private _loadingSteps: Writable<LoadingSteps> = writable({
    directories: false,
    tags: false,
    files: false,
  });
  private _loadingProgress: Writable<number> = writable(0);

  public readonly activeTab = this._activeTab;
  public readonly loadingSteps = this._loadingSteps;
  public readonly loadingProgress = this._loadingProgress;

  // 全体の読み込み状態を管理
  public readonly isAppLoading: Readable<boolean> = derived(
    this._loadingProgress,
    (progress) => progress < 100
  );

  constructor() {
    super();
    
    // 子ViewModelを初期化
    this.directoryViewModel = new DirectoryViewModel();
    this.fileViewModel = new FileViewModel();
    this.searchViewModel = new SearchViewModel();
    this.tagViewModel = new TagViewModel();

    // アプリケーション初期化
    this.initializeApp();
  }

  public setActiveTab(tab: ActiveTab): void {
    this._activeTab.set(tab);
  }

  public async initializeApp(): Promise<void> {
    this.setLoading(true);
    this._loadingProgress.set(0);
    this._loadingSteps.set({ directories: false, tags: false, files: false });

    try {
      // ディレクトリ読み込み
      await this.directoryViewModel.loadDirectories();
      this._loadingSteps.update(steps => ({ ...steps, directories: true }));
      this._loadingProgress.set(33);

      // タグとメタデータキー読み込み
      await Promise.all([
        this.tagViewModel.loadTags(),
        this.tagViewModel.loadCustomMetadataKeys()
      ]);
      this._loadingSteps.update(steps => ({ ...steps, tags: true }));
      this._loadingProgress.set(66);

      // ファイル読み込み
      await this.fileViewModel.loadFiles();
      this._loadingSteps.update(steps => ({ ...steps, files: true }));
      this._loadingProgress.set(100);

      // 読み込み完了後、少し遅延してから状態をクリア
      setTimeout(() => {
        this.setLoading(false);
      }, 500);

    } catch (error) {
      console.error('App initialization failed:', error);
      this.setError('アプリケーションの初期化に失敗しました');
      this.setLoading(false);
    }
  }

  // データ再読み込み
  public async reloadAllData(): Promise<void> {
    await Promise.all([
      this.directoryViewModel.loadDirectories(),
      this.fileViewModel.loadFiles(),
      this.tagViewModel.loadTags(),
      this.tagViewModel.loadCustomMetadataKeys()
    ]);
  }

  // リソースクリーンアップ
  public dispose(): void {
    super.dispose();
    this.directoryViewModel.dispose();
    this.fileViewModel.dispose();
    this.searchViewModel.dispose();
    this.tagViewModel.dispose();
  }
}
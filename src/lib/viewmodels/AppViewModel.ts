import { writable, derived, type Writable, type Readable, type Unsubscriber } from 'svelte/store';
import { BaseViewModel } from './BaseViewModel';
import { DirectoryViewModel } from './DirectoryViewModel';
import { FileViewModel } from './FileViewModel';
import { SearchViewModel } from './SearchViewModel';
import { TagViewModel } from './TagViewModel';
import type { LoadingSteps } from '../types';

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
  private _isAppLoading: Writable<boolean> = writable(true);
  
  // サブスクリプション管理
  private _unsubscribers: Unsubscriber[] = [];

  public readonly activeTab = this._activeTab;
  public readonly loadingSteps = this._loadingSteps;
  public readonly loadingProgress = this._loadingProgress;
  public readonly isAppLoading = this._isAppLoading;

  constructor() {
    super();
    
    // 子ViewModelを初期化
    this.directoryViewModel = new DirectoryViewModel();
    this.fileViewModel = new FileViewModel();
    this.searchViewModel = new SearchViewModel();
    this.tagViewModel = new TagViewModel();

    // ディレクトリ選択の変更をFileViewModelとSearchViewModelに反映
    const unsubscriber = this.directoryViewModel.selectedDirectoryId.subscribe((directoryId) => {
      this.fileViewModel.setSelectedDirectoryId(directoryId);
      this.searchViewModel.setSelectedDirectoryId(directoryId);
    });
    this._unsubscribers.push(unsubscriber);

    // アプリケーション初期化
    this.initializeApp();
  }

  public setActiveTab(tab: ActiveTab): void {
    this._activeTab.set(tab);
  }

  public async initializeApp(): Promise<void> {
    this._isAppLoading.set(true);
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

      // 100%表示を確認してから遅延後にローディング画面を終了
      setTimeout(() => {
        this._isAppLoading.set(false);
        this.setLoading(false);
      }, 1000); // 1秒間100%を表示

    } catch (error) {
      console.error('App initialization failed:', error);
      this.setError('アプリケーションの初期化に失敗しました');
      this._isAppLoading.set(false);
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
    
    // サブスクリプションのクリーンアップ
    this._unsubscribers.forEach(unsubscriber => unsubscriber());
    this._unsubscribers = [];
    
    this.directoryViewModel.dispose();
    this.fileViewModel.dispose();
    this.searchViewModel.dispose();
    this.tagViewModel.dispose();
  }
}
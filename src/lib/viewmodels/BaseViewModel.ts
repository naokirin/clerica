import { writable, type Writable } from 'svelte/store';

export abstract class BaseViewModel {
  protected _isLoading: Writable<boolean> = writable(false);
  protected _error: Writable<string | null> = writable(null);

  public readonly isLoading = this._isLoading;
  public readonly error = this._error;

  protected setLoading(loading: boolean): void {
    this._isLoading.set(loading);
  }

  protected setError(error: string | null): void {
    this._error.set(error);
  }

  protected async executeAsync<T>(operation: () => Promise<T>): Promise<T | null> {
    try {
      this.setLoading(true);
      this.setError(null);
      return await operation();
    } catch (error) {
      console.error('ViewModel operation failed:', error);
      this.setError(error instanceof Error ? error.message : String(error));
      return null;
    } finally {
      this.setLoading(false);
    }
  }

  // リソースのクリーンアップ用
  public dispose(): void {
    // 継承先でオーバーライドして必要なクリーンアップを実装
  }
}
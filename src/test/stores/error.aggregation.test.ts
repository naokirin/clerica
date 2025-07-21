import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { errorStore } from '../../lib/stores/error';
import { get } from 'svelte/store';

describe('Error Store Aggregation', () => {
  beforeEach(() => {
    // 各テスト前にストアをクリア
    errorStore.clearAll();
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
    errorStore.clearAll();
  });

  it('should aggregate duplicate errors within time window', async () => {
    // 同じメッセージのエラーを短時間で複数回発生させる
    errorStore.showError('音楽ファイルのアルバムアート抽出に失敗しました', 10000);
    
    let state = get(errorStore);
    expect(state.errors).toHaveLength(1);
    expect(state.errors[0].message).toBe('音楽ファイルのアルバムアート抽出に失敗しました');
    expect(state.errors[0].count).toBe(1);

    // 2秒後に同じエラーを発生
    vi.advanceTimersByTime(2000);
    errorStore.showError('音楽ファイルのアルバムアート抽出に失敗しました', 10000);
    
    state = get(errorStore);
    expect(state.errors).toHaveLength(1); // エラー数は1つのまま
    expect(state.errors[0].message).toBe('音楽ファイルのアルバムアート抽出に失敗しました (2件)');
    expect(state.errors[0].count).toBe(2);

    // さらに1秒後に同じエラーを発生
    vi.advanceTimersByTime(1000);
    errorStore.showError('音楽ファイルのアルバムアート抽出に失敗しました', 10000);
    
    state = get(errorStore);
    expect(state.errors).toHaveLength(1);
    expect(state.errors[0].message).toBe('音楽ファイルのアルバムアート抽出に失敗しました (3件)');
    expect(state.errors[0].count).toBe(3);
  });

  it('should create separate errors after aggregation window expires', () => {
    // 最初のエラー
    errorStore.showError('音楽ファイルのアルバムアート抽出に失敗しました', 10000);
    
    let state = get(errorStore);
    expect(state.errors).toHaveLength(1);
    expect(state.errors[0].count).toBe(1);

    // 5秒経過後に同じエラーを発生（集約ウィンドウ外）
    vi.advanceTimersByTime(5001);
    errorStore.showError('音楽ファイルのアルバムアート抽出に失敗しました', 10000);
    
    state = get(errorStore);
    expect(state.errors).toHaveLength(2); // 新しいエラーが作成される
    expect(state.errors[1].count).toBe(1);
  });

  it('should aggregate different error types separately', () => {
    // エラーと警告を発生
    errorStore.showError('テストメッセージ', 10000);
    errorStore.showWarning('テストメッセージ', 10000);
    
    let state = get(errorStore);
    expect(state.errors).toHaveLength(2); // タイプが違うので別々に管理

    // 同じメッセージを再度発生
    errorStore.showError('テストメッセージ', 10000);
    errorStore.showWarning('テストメッセージ', 10000);
    
    state = get(errorStore);
    expect(state.errors).toHaveLength(2);
    expect(state.errors[0].count).toBe(2); // error type
    expect(state.errors[1].count).toBe(2); // warning type
  });

  it('should not aggregate different messages', () => {
    errorStore.showError('音楽ファイルのアルバムアート抽出に失敗しました', 10000);
    errorStore.showError('動画のサムネイル生成に失敗しました', 10000);
    
    const state = get(errorStore);
    expect(state.errors).toHaveLength(2); // 異なるメッセージなので別々
    expect(state.errors[0].count).toBe(1);
    expect(state.errors[1].count).toBe(1);
  });

  it('should clean up aggregation map when errors are dismissed', () => {
    errorStore.showError('テストエラー', 10000);
    
    let state = get(errorStore);
    const errorId = state.errors[0].id;
    expect(state.errors).toHaveLength(1);

    // エラーを削除
    errorStore.dismissError(errorId);
    
    state = get(errorStore);
    expect(state.errors).toHaveLength(0);

    // 同じメッセージを再発生させると新しいエラーとして扱われる
    errorStore.showError('テストエラー', 10000);
    state = get(errorStore);
    expect(state.errors).toHaveLength(1);
    expect(state.errors[0].count).toBe(1);
  });

  it('should clean up aggregation map when all errors are cleared', () => {
    errorStore.showError('テストエラー1', 10000);
    errorStore.showError('テストエラー2', 10000);
    
    let state = get(errorStore);
    expect(state.errors).toHaveLength(2);

    // 全エラーをクリア
    errorStore.clearAll();
    
    state = get(errorStore);
    expect(state.errors).toHaveLength(0);

    // 同じメッセージを再発生させると新しいエラーとして扱われる
    errorStore.showError('テストエラー1', 10000);
    state = get(errorStore);
    expect(state.errors).toHaveLength(1);
    expect(state.errors[0].count).toBe(1);
  });
});
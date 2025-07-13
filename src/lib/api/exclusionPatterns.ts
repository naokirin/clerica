import { invoke } from '@tauri-apps/api/core';

export interface ExclusionPattern {
  id: number;
  pattern: string;
  created_at: string;
}

export interface CreateExclusionPatternRequest {
  pattern: string;
}

export const exclusionPatternsApi = {
  /**
   * 除外パターン一覧を取得
   */
  async getExclusionPatterns(): Promise<ExclusionPattern[]> {
    return await invoke('get_exclusion_patterns');
  },

  /**
   * 除外パターンを追加
   */
  async addExclusionPattern(pattern: string): Promise<void> {
    return await invoke('add_exclusion_pattern', { pattern });
  },

  /**
   * 除外パターンを削除
   */
  async deleteExclusionPattern(id: number): Promise<void> {
    return await invoke('delete_exclusion_pattern', { id });
  },

  /**
   * 除外パターンをテスト
   */
  async testExclusionPattern(pattern: string, testPath: string): Promise<boolean> {
    return await invoke('test_exclusion_pattern', { pattern, testPath });
  },

  /**
   * 除外パターンの妥当性を検証
   */
  async validateExclusionPattern(pattern: string): Promise<boolean> {
    try {
      return await invoke('validate_exclusion_pattern', { pattern });
    } catch (error) {
      throw new Error(error as string);
    }
  }
};
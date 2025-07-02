// このファイルは API レイヤー専用の型定義です
// 各APIファイルで使用する型をここに定義します

export interface APIResponse<T> {
  data: T;
  success: boolean;
  message?: string;
}

export interface APIError {
  code: string;
  message: string;
  details?: any;
}

// API呼び出し時のオプション
export interface APIOptions {
  timeout?: number;
  retries?: number;
}
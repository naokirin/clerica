import { writable } from 'svelte/store';

export interface ErrorInfo {
  id: string;
  message: string;
  type: 'error' | 'warning' | 'info' | 'success';
  duration?: number;
  timestamp: number;
  count?: number; // 集約されたエラー数
}

export interface ErrorState {
  errors: ErrorInfo[];
}

const initialState: ErrorState = {
  errors: []
};

// エラー集約用の管理データ
interface ErrorAggregationInfo {
  errorId: string;
  lastTimestamp: number;
  count: number;
  timeoutId?: number;
}

const AGGREGATION_WINDOW_MS = 5000; // 5秒間のウィンドウで集約

function createErrorStore() {
  const { subscribe, set, update } = writable<ErrorState>(initialState);
  
  // メッセージ + タイプをキーとした集約管理マップ
  const aggregationMap = new Map<string, ErrorAggregationInfo>();

  // 共通のエラー追加関数（集約機能付き）
  const addErrorWithAggregation = (
    originalMessage: string, 
    type: 'error' | 'warning' | 'info' | 'success', 
    duration: number
  ) => {
    const now = Date.now();
    const aggregationKey = `${originalMessage}|${type}`;
    const existingAggregation = aggregationMap.get(aggregationKey);
    
    if (existingAggregation && (now - existingAggregation.lastTimestamp) <= AGGREGATION_WINDOW_MS) {
      // 既存エラーを集約する
      existingAggregation.count++;
      existingAggregation.lastTimestamp = now;
      
      // 既存のタイムアウトをクリア
      if (existingAggregation.timeoutId) {
        clearTimeout(existingAggregation.timeoutId);
      }
      
      // エラーメッセージを更新（件数表示付き）
      const aggregatedMessage = existingAggregation.count > 1 
        ? `${originalMessage} (${existingAggregation.count}件)`
        : originalMessage;
      
      update(state => ({
        errors: state.errors.map(e => 
          e.id === existingAggregation.errorId 
            ? { ...e, message: aggregatedMessage, timestamp: now, count: existingAggregation.count }
            : e
        )
      }));
      
      // 新しいタイムアウトを設定（集約ウィンドウ終了時にマップから削除）
      existingAggregation.timeoutId = setTimeout(() => {
        aggregationMap.delete(aggregationKey);
      }, AGGREGATION_WINDOW_MS);
      
      // エラー表示の自動削除タイマー
      if (duration > 0) {
        setTimeout(() => {
          update(state => ({
            errors: state.errors.filter(e => e.id !== existingAggregation.errorId)
          }));
          aggregationMap.delete(aggregationKey);
        }, duration);
      }
    } else {
      // 新しいエラーを作成
      const error: ErrorInfo = {
        id: crypto.randomUUID(),
        message: originalMessage,
        type,
        duration,
        timestamp: now,
        count: 1
      };
      
      // 集約情報を記録
      const newAggregation: ErrorAggregationInfo = {
        errorId: error.id,
        lastTimestamp: now,
        count: 1,
        timeoutId: setTimeout(() => {
          aggregationMap.delete(aggregationKey);
        }, AGGREGATION_WINDOW_MS)
      };
      aggregationMap.set(aggregationKey, newAggregation);
      
      update(state => ({
        errors: [...state.errors, error]
      }));
      
      // 指定時間後に自動的に削除
      if (duration > 0) {
        setTimeout(() => {
          update(state => ({
            errors: state.errors.filter(e => e.id !== error.id)
          }));
          aggregationMap.delete(aggregationKey);
        }, duration);
      }
    }
  };

  return {
    subscribe,
    
    // エラーメッセージを表示
    showError: (message: string, duration = 5000) => {
      addErrorWithAggregation(message, 'error', duration);
    },
    
    // 警告メッセージを表示
    showWarning: (message: string, duration = 4000) => {
      addErrorWithAggregation(message, 'warning', duration);
    },
    
    // 情報メッセージを表示
    showInfo: (message: string, duration = 3000) => {
      addErrorWithAggregation(message, 'info', duration);
    },
    
    // 成功メッセージを表示
    showSuccess: (message: string, duration = 3000) => {
      addErrorWithAggregation(message, 'success', duration);
    },
    
    // 特定のエラーを削除
    dismissError: (id: string) => {
      update(state => ({
        errors: state.errors.filter(e => e.id !== id)
      }));
      
      // 集約管理マップからも該当エラーを削除
      for (const [key, aggregation] of aggregationMap.entries()) {
        if (aggregation.errorId === id) {
          if (aggregation.timeoutId) {
            clearTimeout(aggregation.timeoutId);
          }
          aggregationMap.delete(key);
          break;
        }
      }
    },
    
    // 全てのエラーをクリア
    clearAll: () => {
      set(initialState);
      
      // 全てのタイムアウトをクリアし、集約管理マップを初期化
      for (const aggregation of aggregationMap.values()) {
        if (aggregation.timeoutId) {
          clearTimeout(aggregation.timeoutId);
        }
      }
      aggregationMap.clear();
    }
  };
}

export const errorStore = createErrorStore();
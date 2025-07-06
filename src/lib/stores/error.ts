import { writable } from 'svelte/store';

export interface ErrorInfo {
  id: string;
  message: string;
  type: 'error' | 'warning' | 'info' | 'success';
  duration?: number;
  timestamp: number;
}

export interface ErrorState {
  errors: ErrorInfo[];
}

const initialState: ErrorState = {
  errors: []
};

function createErrorStore() {
  const { subscribe, set, update } = writable<ErrorState>(initialState);

  return {
    subscribe,
    
    // エラーメッセージを表示
    showError: (message: string, duration = 5000) => {
      const error: ErrorInfo = {
        id: crypto.randomUUID(),
        message,
        type: 'error',
        duration,
        timestamp: Date.now()
      };
      
      update(state => ({
        errors: [...state.errors, error]
      }));
      
      // 指定時間後に自動的に削除
      if (duration > 0) {
        setTimeout(() => {
          update(state => ({
            errors: state.errors.filter(e => e.id !== error.id)
          }));
        }, duration);
      }
    },
    
    // 警告メッセージを表示
    showWarning: (message: string, duration = 4000) => {
      const error: ErrorInfo = {
        id: crypto.randomUUID(),
        message,
        type: 'warning',
        duration,
        timestamp: Date.now()
      };
      
      update(state => ({
        errors: [...state.errors, error]
      }));
      
      if (duration > 0) {
        setTimeout(() => {
          update(state => ({
            errors: state.errors.filter(e => e.id !== error.id)
          }));
        }, duration);
      }
    },
    
    // 情報メッセージを表示
    showInfo: (message: string, duration = 3000) => {
      const error: ErrorInfo = {
        id: crypto.randomUUID(),
        message,
        type: 'info',
        duration,
        timestamp: Date.now()
      };
      
      update(state => ({
        errors: [...state.errors, error]
      }));
      
      if (duration > 0) {
        setTimeout(() => {
          update(state => ({
            errors: state.errors.filter(e => e.id !== error.id)
          }));
        }, duration);
      }
    },
    
    // 成功メッセージを表示
    showSuccess: (message: string, duration = 3000) => {
      const error: ErrorInfo = {
        id: crypto.randomUUID(),
        message,
        type: 'success',
        duration,
        timestamp: Date.now()
      };
      
      update(state => ({
        errors: [...state.errors, error]
      }));
      
      if (duration > 0) {
        setTimeout(() => {
          update(state => ({
            errors: state.errors.filter(e => e.id !== error.id)
          }));
        }, duration);
      }
    },
    
    // 特定のエラーを削除
    dismissError: (id: string) => {
      update(state => ({
        errors: state.errors.filter(e => e.id !== id)
      }));
    },
    
    // 全てのエラーをクリア
    clearAll: () => {
      set(initialState);
    }
  };
}

export const errorStore = createErrorStore();
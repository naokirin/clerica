import { contextBridge, ipcRenderer, type IpcRendererEvent } from 'electron';

// レンダラープロセスに公開するAPIの型定義
interface ElectronAPI {
  sendMessage: (message: string) => void;
  onMessage: (callback: (event: IpcRendererEvent, response: string) => void) => void;
  openFileDialog: () => Promise<{ canceled: boolean; filePaths: string[] }>;
  getAppInfo: () => Promise<{ name: string; version: string; electronVersion: string }>;
  minimizeWindow: () => void;
  toggleMaximize: () => void;
  closeWindow: () => void;
}

// レンダラープロセスに公開するAPI
contextBridge.exposeInMainWorld('electronAPI', {
  // メインプロセスにメッセージを送信
  sendMessage: (message: string): void => {
    ipcRenderer.send('message', message);
  },

  // メインプロセスからの応答を受信
  onMessage: (callback: (event: IpcRendererEvent, response: string) => void): void => {
    ipcRenderer.on('message-response', callback);
  },

  // ファイルダイアログを開く
  openFileDialog: async (): Promise<{ canceled: boolean; filePaths: string[] }> => {
    return await ipcRenderer.invoke('open-file-dialog');
  },

  // アプリケーション情報を取得
  getAppInfo: async (): Promise<{ name: string; version: string; electronVersion: string }> => {
    return await ipcRenderer.invoke('get-app-info');
  },

  // ウィンドウを最小化
  minimizeWindow: (): void => {
    ipcRenderer.send('minimize-window');
  },

  // ウィンドウを最大化/復元
  toggleMaximize: (): void => {
    ipcRenderer.send('toggle-maximize');
  },

  // ウィンドウを閉じる
  closeWindow: (): void => {
    ipcRenderer.send('close-window');
  }
} as ElectronAPI);

// セキュリティのため、Node.jsのAPIは直接公開しない
console.log('Preload script loaded');

// 型定義をグローバルに公開
declare global {
  interface Window {
    electronAPI: ElectronAPI;
  }
} 
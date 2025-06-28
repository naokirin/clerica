import { app, BrowserWindow, ipcMain, dialog, type IpcMainEvent } from 'electron';
import * as path from 'node:path';

// メインプロセスウィンドウの参照を保持
let mainWindow: BrowserWindow | null = null;

interface AppInfo {
  name: string;
  version: string;
  electronVersion: string;
}

function createWindow(): void {
  // メインウィンドウを作成
  mainWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      preload: path.join(__dirname, 'preload.js')
    },
    icon: path.join(__dirname, '../assets/icon.png'), // アイコンファイルがある場合
    show: false // 準備ができるまで非表示
  });

  // レンダラープロセスのHTMLファイルを読み込み
  mainWindow.loadFile(path.join(__dirname, '../renderer/index.html'));

  // ウィンドウが準備できたら表示
  mainWindow.once('ready-to-show', () => {
    if (mainWindow) {
      mainWindow.show();
    }
  });

  // ウィンドウが閉じられたときの処理
  mainWindow.on('closed', () => {
    mainWindow = null;
  });

  // 開発モードの場合はDevToolsを開く
  if (process.argv.includes('--dev')) {
    mainWindow.webContents.openDevTools();
  }
}

// IPCハンドラーの設定
function setupIpcHandlers(): void {
  // メッセージ受信ハンドラー
  ipcMain.on('message', (event: IpcMainEvent, message: string) => {
    console.log('Rendererからメッセージを受信:', message);
    // レンダラープロセスに応答を送信
    event.reply('message-response', `メッセージを受信しました: ${message}`);
  });

  // ファイルダイアログを開く
  ipcMain.handle('open-file-dialog', async () => {
    if (!mainWindow) {
      throw new Error('メインウィンドウが存在しません');
    }

    const result = await dialog.showOpenDialog(mainWindow, {
      properties: ['openFile', 'multiSelections'],
      filters: [
        { name: 'すべてのファイル', extensions: ['*'] },
        { name: 'テキストファイル', extensions: ['txt', 'md'] },
        { name: '画像ファイル', extensions: ['jpg', 'jpeg', 'png', 'gif'] }
      ]
    });
    return result;
  });

  // アプリケーション情報を取得
  ipcMain.handle('get-app-info', (): AppInfo => {
    return {
      name: app.getName(),
      version: app.getVersion(),
      electronVersion: process.versions.electron
    };
  });

  // ウィンドウを最小化
  ipcMain.on('minimize-window', () => {
    if (mainWindow) {
      mainWindow.minimize();
    }
  });

  // ウィンドウを最大化/復元
  ipcMain.on('toggle-maximize', () => {
    if (mainWindow) {
      if (mainWindow.isMaximized()) {
        mainWindow.unmaximize();
      } else {
        mainWindow.maximize();
      }
    }
  });

  // ウィンドウを閉じる
  ipcMain.on('close-window', () => {
    if (mainWindow) {
      mainWindow.close();
    }
  });
}

// アプリケーションが準備できたときにウィンドウを作成
app.whenReady().then(() => {
  setupIpcHandlers();
  createWindow();

  // macOSでは、アプリケーションがアクティブになったときにウィンドウを再作成
  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

// すべてのウィンドウが閉じられたときにアプリケーションを終了
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

// セキュリティのため、新規ウィンドウの作成を制限
app.on('web-contents-created', (_event, contents) => {
  contents.on('will-navigate', (event) => {
    event.preventDefault();
  });
}); 
// DOM要素の取得
const fileBtn = document.getElementById('file-btn') as HTMLButtonElement;
const infoBtn = document.getElementById('info-btn') as HTMLButtonElement;
const messageBtn = document.getElementById('message-btn') as HTMLButtonElement;
const output = document.getElementById('output') as HTMLDivElement;
const minimizeBtn = document.getElementById('minimize-btn') as HTMLButtonElement;
const maximizeBtn = document.getElementById('maximize-btn') as HTMLButtonElement;
const closeBtn = document.getElementById('close-btn') as HTMLButtonElement;

// 出力エリアにメッセージを表示する関数
function showOutput(message: string, type: 'info' | 'success' | 'error' = 'info'): void {
  if (output) {
    output.innerHTML = `<p>${message}</p>`;
    output.className = `output-area ${type}`;
  }
}

// ファイルを開くボタンのイベント
fileBtn?.addEventListener('click', async () => {
  try {
    const result = await window.electronAPI.openFileDialog();
    if (result.canceled) {
      showOutput('ファイル選択がキャンセルされました。', 'info');
    } else {
      showOutput(`選択されたファイル: ${result.filePaths.join(', ')}`, 'success');
    }
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : '不明なエラー';
    showOutput(`エラーが発生しました: ${errorMessage}`, 'error');
  }
});

// アプリ情報ボタンのイベント
infoBtn?.addEventListener('click', async () => {
  try {
    const appInfo = await window.electronAPI.getAppInfo();
    showOutput(`アプリ名: ${appInfo.name}<br>バージョン: ${appInfo.version}<br>Electron: ${appInfo.electronVersion}`, 'success');
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : '不明なエラー';
    showOutput(`エラーが発生しました: ${errorMessage}`, 'error');
  }
});

// メッセージ送信ボタンのイベント
messageBtn?.addEventListener('click', () => {
  const message = `Hello from renderer! Time: ${new Date().toLocaleString()}`;
  window.electronAPI.sendMessage(message);
  showOutput(`メッセージを送信しました: ${message}`, 'info');
});

// メインプロセスからの応答を受信
window.electronAPI.onMessage((_event, response) => {
  showOutput(`メインプロセスからの応答: ${response}`, 'success');
});

// ウィンドウコントロールボタンのイベント
minimizeBtn?.addEventListener('click', () => {
  window.electronAPI.minimizeWindow();
});

maximizeBtn?.addEventListener('click', () => {
  window.electronAPI.toggleMaximize();
});

closeBtn?.addEventListener('click', () => {
  window.electronAPI.closeWindow();
});

// アプリケーションの初期化
document.addEventListener('DOMContentLoaded', () => {
  showOutput('Clericaアプリケーションが起動しました！', 'success');

  // キーボードショートカット
  document.addEventListener('keydown', (event: KeyboardEvent) => {
    // Cmd/Ctrl + Q でアプリケーションを終了
    if ((event.metaKey || event.ctrlKey) && event.key === 'q') {
      window.electronAPI.closeWindow();
    }

    // Cmd/Ctrl + R でリロード
    if ((event.metaKey || event.ctrlKey) && event.key === 'r') {
      location.reload();
    }
  });
});

// エラーハンドリング
window.addEventListener('error', (event: ErrorEvent) => {
  showOutput(`JavaScriptエラー: ${event.error?.message || '不明なエラー'}`, 'error');
});

// 未処理のPromise拒否をキャッチ
window.addEventListener('unhandledrejection', (event: PromiseRejectionEvent) => {
  showOutput(`未処理のPromise拒否: ${event.reason}`, 'error');
}); 
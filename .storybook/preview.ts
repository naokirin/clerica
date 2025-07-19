import type { Preview } from '@storybook/sveltekit'
import { mockIPC } from '@tauri-apps/api/mocks';
import { invoke } from './mocks/tauri';

// グローバルCSSをインポート
import '../src/index.css';
import '../src/lib/App.css';
// Storybook用のスクロール修正CSS
import './storybook-fix.css';

// TauriのIPC(プロセス間通信)をモックする
mockIPC((cmd, args) => {
  return invoke(cmd, args);
});

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
       color: /(background|color)$/i,
       date: /Date$/i,
      },
    },
    docs: {
      autodocs: 'tag',
    },
  },
};

export default preview;
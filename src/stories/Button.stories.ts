import type { Meta, StoryObj } from '@storybook/svelte';
import Button from '../lib/components/parts/Button.svelte';

const meta = {
  title: 'Components/Button',
  component: Button,
  parameters: {
    layout: 'centered',
  },
  argTypes: {
    variant: {
      control: 'select',
      options: ['primary', 'secondary', 'danger'],
      description: 'Button variant'
    },
    size: {
      control: 'select',
      options: ['small', 'medium', 'large'],
      description: 'Button size'
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the button is disabled'
    },
    href: {
      control: 'text',
      description: 'URL for link button'
    },
    type: {
      control: 'select',
      options: ['button', 'submit', 'reset'],
      description: 'Button type'
    },
    icon: {
      control: 'text',
      description: 'SVG icon string'
    },
    text: {
      control: 'text',
      description: 'Button text'
    }
  }
} satisfies Meta<Button>;

export default meta;
type Story = StoryObj<typeof meta>;

// アイコン定義
const icons = {
  folderPlus: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"/><line x1="12" y1="10" x2="12" y2="16"/><line x1="9" y1="13" x2="15" y2="13"/></svg>',
  settings: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.38a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>',
  trash: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3,6 5,6 21,6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>',
  refresh: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M3 21v-5h5"/></svg>',
  search: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>',
  x: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>'
};

// プライマリボタン（アイコン + テキスト）
export const Primary: Story = {
  args: {
    variant: 'primary',
    size: 'medium',
    disabled: false,
    icon: icons.folderPlus,
    text: 'ディレクトリを追加'
  }
};

// セカンダリボタン（アイコン + テキスト）
export const Secondary: Story = {
  args: {
    variant: 'secondary',
    size: 'medium',
    disabled: false,
    icon: icons.settings,
    text: '設定'
  }
};

// 危険なボタン（アイコン + テキスト）
export const Danger: Story = {
  args: {
    variant: 'danger',
    size: 'medium',
    disabled: false,
    icon: icons.trash,
    text: '削除'
  }
};

// 無効化されたボタン
export const Disabled: Story = {
  args: {
    variant: 'primary',
    size: 'medium',
    disabled: true,
    icon: icons.folderPlus,
    text: '無効化されたボタン'
  }
};

// テキストのみのボタン
export const TextOnly: Story = {
  args: {
    variant: 'primary',
    size: 'medium',
    disabled: false,
    text: 'テキストのみのボタン'
  }
};

// アイコンのみのボタン
export const IconOnly: Story = {
  args: {
    variant: 'secondary',
    size: 'medium',
    disabled: false,
    icon: icons.x
  }
};

// 小さいボタン
export const Small: Story = {
  args: {
    variant: 'primary',
    size: 'small',
    disabled: false,
    icon: icons.folderPlus,
    text: '小さいボタン'
  }
};

// 大きいボタン
export const Large: Story = {
  args: {
    variant: 'primary',
    size: 'large',
    disabled: false,
    icon: icons.refresh,
    text: '再スキャン'
  }
};

// リンクボタン
export const LinkButton: Story = {
  args: {
    variant: 'primary',
    size: 'medium',
    disabled: false,
    href: '#',
    icon: icons.search,
    text: '検索'
  }
};
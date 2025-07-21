import type { Meta, StoryObj } from '@storybook/svelte';
import Button from '../lib/components/parts/Button.svelte';

const meta: Meta<typeof Button> = {
  title: 'Components/Button',
  component: Button,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: `
# Button Component

再利用可能なボタンコンポーネントです。アイコン、テキスト、またはその両方を表示できます。

## 特徴
- 3つの異なるバリアント（primary、secondary、danger）
- 3つのサイズ（small、medium、large）
- アイコンとテキストの組み合わせ
- リンクボタンとしても使用可能
- アクセシビリティ対応

## 使用方法
\`\`\`svelte
<script>
  import Button from '$lib/components/parts/Button.svelte';
</script>

<Button
  variant="primary"
  size="medium"
  text="クリック"
  onclick={handleClick}
/>
\`\`\`
        `
      }
    }
  },
  tags: ['autodocs'],
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
    iconName: {
      control: 'select',
      options: ['FolderPlus', 'Settings', 'Trash2', 'RotateCcw', 'Search', 'X'],
      description: 'Lucide icon name'
    },
    iconSize: {
      control: 'number',
      description: 'Icon size in pixels'
    },
    text: {
      control: 'text',
      description: 'Button text'
    }
  }
};

export default meta;
type Story = StoryObj<typeof meta>;


// プライマリボタン（アイコン + テキスト）
export const Primary: Story = {
  args: {
    variant: 'primary',
    size: 'medium',
    disabled: false,
    iconName: "FolderPlus",
    iconSize: 16,
    text: 'ディレクトリを追加'
  } as any
};

// セカンダリボタン（アイコン + テキスト）
export const Secondary: Story = {
  args: {
    variant: 'secondary',
    size: 'medium',
    disabled: false,
    iconName: 'Settings',
    iconSize: 16,
    text: '設定'
  } as any
};

// 危険なボタン（アイコン + テキスト）
export const Danger: Story = {
  args: {
    variant: 'danger',
    size: 'medium',
    disabled: false,
    iconName: 'Trash2',
    iconSize: 16,
    text: '削除'
  } as any
};

// 無効化されたボタン
export const Disabled: Story = {
  args: {
    variant: 'primary',
    size: 'medium',
    disabled: true,
    iconName: 'FolderPlus',
    iconSize: 16,
    text: '無効化されたボタン'
  } as any
};

// テキストのみのボタン
export const TextOnly: Story = {
  args: {
    variant: 'primary',
    size: 'medium',
    disabled: false,
    text: 'テキストのみのボタン'
  } as any
};

// アイコンのみのボタン
export const IconOnly: Story = {
  args: {
    variant: 'secondary',
    size: 'medium',
    disabled: false,
    iconName: 'X',
    iconSize: 16
  } as any
};

// 小さいボタン
export const Small: Story = {
  args: {
    variant: 'primary',
    size: 'small',
    disabled: false,
    iconName: 'FolderPlus',
    iconSize: 14,
    text: '小さいボタン'
  } as any
};

// 大きいボタン
export const Large: Story = {
  args: {
    variant: 'primary',
    size: 'large',
    disabled: false,
    iconName: 'RotateCcw',
    iconSize: 18,
    text: '再スキャン'
  } as any
};

// リンクボタン
export const LinkButton: Story = {
  args: {
    variant: 'primary',
    size: 'medium',
    disabled: false,
    href: '#',
    iconName: 'Search',
    iconSize: 16,
    text: '検索'
  } as any
};
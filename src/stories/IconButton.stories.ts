import type { Meta, StoryObj } from '@storybook/svelte';
import IconButtonWrapper from '../../.storybook/mocks/IconButtonWrapper.svelte';

const meta: Meta<any> = {
  title: 'Components/IconButton',
  component: IconButtonWrapper as any,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: `
# IconButton Component

アイコンのみを表示する小さなボタンコンポーネントです。UI要素の操作に使用されます。

## 特徴
- 複数のサイズ（small、medium、large）
- カスタムアイコンSVG対応
- ツールチップ表示可能
- アクセシビリティ対応

## 使用方法
\`\`\`svelte
<script>
  import IconButton from '$lib/components/parts/IconButton.svelte';
  
  const icon = '<svg>...</svg>';
</script>

<IconButton
  {icon}
  size="medium"
  tooltip="ボタンの説明"
  on:click={handleClick}
/>
\`\`\`
        `
      }
    }
  },
  tags: ['autodocs'],
  argTypes: {
    iconName: {
      control: 'select',
      options: [
        'folder-plus', 'settings', 'trash-2', 'refresh-cw', 'search', 
        'x', 'edit-3', 'check', 'save', 'plus', 'minus', 'eye', 'eye-off',
        'file-text', 'tag', 'star', 'heart', 'home', 'user', 'mail',
        'phone', 'calendar', 'clock', 'download', 'upload', 'info',
        'alert-circle', 'check-circle', 'x-circle', 'arrow-left',
        'arrow-right', 'arrow-up', 'arrow-down', 'chevron-left',
        'chevron-right', 'chevron-up', 'chevron-down'
      ],
      description: 'Lucide icon name (kebab-case)'
    },
    title: {
      control: 'text',
      description: 'Tooltip text'
    },
    onClick: {
      action: 'clicked',
      description: 'Click event handler'
    },
    size: {
      control: { type: 'number', min: 10, max: 32, step: 2 },
      description: 'Icon size in pixels'
    },
    class: {
      control: 'text',
      description: 'Additional CSS classes'
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the button is disabled'
    }
  } as any
} satisfies Meta<IconButtonWrapper>;

export default meta;
type Story = StoryObj<typeof meta>;

// デフォルトのアイコンボタン
export const Default: Story = {
  args: {
    iconName: 'circle',
    title: 'デフォルトアイコン',
    size: 14,
    disabled: false
  } as any
};

// フォルダ追加ボタン
export const FolderPlus: Story = {
  args: {
    iconName: 'folder-plus',
    title: 'ディレクトリを追加',
    size: 16,
    disabled: false
  } as any
};

// 設定ボタン
export const Settings: Story = {
  args: {
    iconName: 'settings',
    title: '設定',
    size: 16,
    disabled: false
  } as any
};

// 削除ボタン（赤色）
export const Delete: Story = {
  args: {
    iconName: 'trash-2',
    title: '削除',
    size: 16,
    class: 'red',
    disabled: false
  } as any
};

// 保存ボタン（緑色）
export const Save: Story = {
  args: {
    iconName: 'save',
    title: '保存',
    size: 16,
    class: 'green',
    disabled: false
  } as any
};

// 検索ボタン
export const Search: Story = {
  args: {
    iconName: 'search',
    title: '検索',
    size: 16,
    disabled: false
  } as any
};

// 編集ボタン
export const Edit: Story = {
  args: {
    iconName: 'edit-3',
    title: '編集',
    size: 16,
    disabled: false
  } as any
};

// 更新ボタン
export const Refresh: Story = {
  args: {
    iconName: 'refresh-cw',
    title: '更新',
    size: 16,
    disabled: false
  } as any
};

// 閉じるボタン
export const Close: Story = {
  args: {
    iconName: 'x',
    title: '閉じる',
    size: 16,
    disabled: false
  } as any
};

// チェックボタン
export const Check: Story = {
  args: {
    iconName: 'check',
    title: '完了',
    size: 16,
    class: 'green',
    disabled: false
  } as any
};

// 無効化されたボタン
export const Disabled: Story = {
  args: {
    iconName: 'folder-plus',
    title: '無効化されたボタン',
    size: 16,
    disabled: true
  } as any
};

// 小さいサイズ
export const Small: Story = {
  args: {
    iconName: 'star',
    title: '小さいアイコン',
    size: 12,
    disabled: false
  } as any
};

// 大きいサイズ
export const Large: Story = {
  args: {
    iconName: 'heart',
    title: '大きいアイコン',
    size: 24,
    disabled: false
  } as any
};

// アイコン一覧（Storybookでアイコン選択のテスト用）
export const IconShowcase: Story = {
  args: {
    iconName: 'star',
    title: 'アイコンを選択してテスト',
    size: 20,
    disabled: false
  } as any
};
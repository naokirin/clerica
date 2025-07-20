import type { Meta, StoryObj } from '@storybook/svelte';
import TextInput from '../lib/components/parts/TextInput.svelte';

const meta = {
  title: 'Components/TextInput',
  component: TextInput,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: `
# TextInput Component

再利用可能なテキスト入力コンポーネントです。フォームの標準的なテキスト入力として使用できます。

## 特徴
- TypeScript完全対応
- アクセシビリティ対応（id、label連携）
- フォーカス状態の視覚的フィードバック
- 無効化状態の適切な表示
- プレースホルダーテキスト対応
- カスタムスタイリング対応

## 使用方法
\`\`\`svelte
<script>
  import TextInput from '$lib/components/parts/TextInput.svelte';
  
  let value = '';
</script>

<label for="username">ユーザー名</label>
<TextInput
  id="username"
  bind:value
  placeholder="ユーザー名を入力"
  required
/>
\`\`\`

## アクセシビリティ
- 必ずidプロパティを指定してください
- labelとの関連付けに使用されます
- スクリーンリーダーでの適切な読み上げに必要です
        `
      }
    }
  },
  tags: ['autodocs'],
  argTypes: {
    id: {
      control: 'text',
      description: 'Required unique identifier for the input (for label association)'
    },
    value: {
      control: 'text',
      description: 'Current value of the input'
    },
    placeholder: {
      control: 'text',
      description: 'Placeholder text shown when input is empty'
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the input is disabled'
    },
    required: {
      control: 'boolean',
      description: 'Whether the input is required'
    },
    class: {
      control: 'text',
      description: 'Additional CSS classes to apply'
    }
  }
} satisfies Meta<TextInput>;

export default meta;
type Story = StoryObj<typeof meta>;

// 基本的なテキスト入力
export const Default: Story = {
  args: {
    id: 'default-input',
    value: '',
    placeholder: 'テキストを入力してください'
  }
};

// 初期値ありのテキスト入力
export const WithValue: Story = {
  args: {
    id: 'with-value-input',
    value: 'デフォルト値',
    placeholder: 'テキストを入力してください'
  }
};

// 必須項目のテキスト入力
export const Required: Story = {
  args: {
    id: 'required-input',
    value: '',
    placeholder: '必須項目です',
    required: true
  }
};

// 無効化されたテキスト入力
export const Disabled: Story = {
  args: {
    id: 'disabled-input',
    value: '編集できません',
    placeholder: 'テキストを入力してください',
    disabled: true
  }
};

// 検索フィールド
export const SearchField: Story = {
  args: {
    id: 'search-input',
    value: '',
    placeholder: 'ファイルやフォルダを検索...',
    class: 'search-input'
  }
};

// ユーザー名入力
export const UsernameField: Story = {
  args: {
    id: 'username-input',
    value: '',
    placeholder: 'ユーザー名を入力',
    required: true
  }
};

// メール入力
export const EmailField: Story = {
  args: {
    id: 'email-input',
    value: '',
    placeholder: 'example@domain.com'
  }
};

// パスワード入力（テキストタイプだが用途例として）
export const PasswordField: Story = {
  args: {
    id: 'password-input',
    value: '',
    placeholder: 'パスワードを入力',
    required: true
  }
};

// ファイル名入力
export const FilenameField: Story = {
  args: {
    id: 'filename-input',
    value: 'document.txt',
    placeholder: 'ファイル名を入力'
  }
};

// カスタムスタイル付きテキスト入力
export const CustomStyled: Story = {
  args: {
    id: 'custom-input',
    value: '',
    placeholder: 'カスタムスタイル',
    class: 'custom-border'
  },
  parameters: {
    docs: {
      description: {
        story: 'classプロパティを使用してカスタムスタイルを適用した例'
      }
    }
  }
};
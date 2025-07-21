import type { Meta, StoryObj } from '@storybook/svelte';
import Textarea from '../lib/components/parts/Textarea.svelte';

const meta: Meta<any> = {
  title: 'Components/Textarea',
  component: Textarea as any,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: `
# Textarea Component

再利用可能なテキストエリアコンポーネントです。複数行のテキスト入力として使用できます。

## 特徴
- TypeScript完全対応
- アクセシビリティ対応（id、label連携）
- フォーカス状態の視覚的フィードバック
- 無効化状態の適切な表示
- プレースホルダーテキスト対応
- カスタムスタイリング対応
- 等幅フォント（monospace）オプション
- リサイズ方向の制御可能

## 使用方法
\`\`\`svelte
<script>
  import Textarea from '$lib/components/parts/Textarea.svelte';
  
  let value = '';
</script>

<label for="description">説明</label>
<Textarea
  id="description"
  bind:value
  placeholder="説明を入力"
  rows={4}
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
      description: 'Required unique identifier for the textarea (for label association)'
    },
    value: {
      control: 'text',
      description: 'Current value of the textarea'
    },
    placeholder: {
      control: 'text',
      description: 'Placeholder text shown when textarea is empty'
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the textarea is disabled'
    },
    required: {
      control: 'boolean',
      description: 'Whether the textarea is required'
    },
    rows: {
      control: 'number',
      description: 'Number of visible text lines'
    },
    cols: {
      control: 'number',
      description: 'Visible width of the text control (in average character widths)'
    },
    monospace: {
      control: 'boolean',
      description: 'Use monospace font (useful for code or JSON input)'
    },
    resize: {
      control: 'select',
      options: ['vertical', 'horizontal', 'both', 'none'],
      description: 'Controls the resize behavior'
    },
    class: {
      control: 'text',
      description: 'Additional CSS classes to apply'
    }
  } as any
} satisfies Meta<Textarea>;

export default meta;
type Story = StoryObj<typeof meta>;

// 基本的なテキストエリア
export const Default: Story = {
  args: {
    id: 'default-textarea',
    value: '',
    placeholder: 'メッセージを入力してください'
  } as any
};

// 初期値ありのテキストエリア
export const WithValue: Story = {
  args: {
    id: 'with-value-textarea',
    value: 'これはサンプルテキストです。\n複数行のテキストが入力されています。',
    placeholder: 'メッセージを入力してください',
    rows: 4
  } as any
};

// 必須項目のテキストエリア
export const Required: Story = {
  args: {
    id: 'required-textarea',
    value: '',
    placeholder: '必須項目です（コメントを入力してください）',
    required: true,
    rows: 3
  } as any
};

// 無効化されたテキストエリア
export const Disabled: Story = {
  args: {
    id: 'disabled-textarea',
    value: '編集できないテキストです。\nこの内容は変更できません。',
    placeholder: 'メッセージを入力してください',
    disabled: true,
    rows: 3
  } as any
};

// 等幅フォント（JSON入力用）
export const MonospaceJSON: Story = {
  args: {
    id: 'json-textarea',
    value: '{\n  "name": "example",\n  "value": 123,\n  "active": true\n}',
    placeholder: 'JSON形式でデータを入力してください',
    monospace: true,
    rows: 6,
    resize: 'vertical'
  } as any,
  parameters: {
    docs: {
      description: {
        story: 'JSON入力に適した等幅フォントとリサイズ設定の例'
      }
    }
  }
};

// コード入力用
export const CodeInput: Story = {
  args: {
    id: 'code-textarea',
    value: 'function hello() {\n  console.log("Hello, World!");\n}',
    placeholder: 'コードを入力してください',
    monospace: true,
    rows: 5,
    resize: 'both'
  } as any,
  parameters: {
    docs: {
      description: {
        story: 'コード入力に適した設定。等幅フォントと両方向リサイズが可能'
      }
    }
  }
};

// 説明文入力
export const DescriptionField: Story = {
  args: {
    id: 'description-textarea',
    value: '',
    placeholder: '商品やサービスの詳細な説明を入力してください',
    rows: 4,
    resize: 'vertical'
  } as any
};

// コメント入力
export const CommentField: Story = {
  args: {
    id: 'comment-textarea',
    value: '',
    placeholder: 'コメントを入力してください...',
    rows: 3,
    resize: 'none'
  } as any,
  parameters: {
    docs: {
      description: {
        story: 'リサイズを無効にしたコメント入力フィールドの例'
      }
    }
  }
};

// 長い文章入力
export const LongTextInput: Story = {
  args: {
    id: 'long-text-textarea',
    value: '',
    placeholder: '記事の本文や長い文章を入力してください',
    rows: 8,
    resize: 'vertical'
  } as any
};

// カスタムスタイル付きテキストエリア
export const CustomStyled: Story = {
  args: {
    id: 'custom-textarea',
    value: '',
    placeholder: 'カスタムスタイルのテキストエリア',
    rows: 4,
    class: 'custom-border'
  } as any,
  parameters: {
    docs: {
      description: {
        story: 'classプロパティを使用してカスタムスタイルを適用した例'
      }
    }
  }
};

// 水平リサイズ可能
export const HorizontalResize: Story = {
  args: {
    id: 'horizontal-resize-textarea',
    value: '',
    placeholder: '水平方向にリサイズできます',
    rows: 3,
    resize: 'horizontal'
  } as any,
  parameters: {
    docs: {
      description: {
        story: '水平方向のみリサイズ可能な設定'
      }
    }
  }
};
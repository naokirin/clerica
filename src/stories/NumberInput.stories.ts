import type { Meta, StoryObj } from '@storybook/svelte';
import NumberInput from '../lib/components/parts/NumberInput.svelte';

const meta = {
  title: 'Components/NumberInput',
  component: NumberInput,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: `
# NumberInput Component

再利用可能な数値入力コンポーネントです。フォームの数値入力として使用できます。

## 特徴
- TypeScript完全対応
- 数値入力専用（type="number"）
- min、max、stepなどの数値制約対応
- アクセシビリティ対応（id、label連携）
- フォーカス状態の視覚的フィードバック
- 無効化状態の適切な表示
- プレースホルダーテキスト対応
- カスタムスタイリング対応

## 使用方法
\`\`\`svelte
<script>
  import NumberInput from '$lib/components/parts/NumberInput.svelte';
  
  let count = 0;
</script>

<label for="count">カウント</label>
<NumberInput
  id="count"
  bind:value={count}
  min={0}
  max={100}
  step={1}
  placeholder="数値を入力"
  required
/>
\`\`\`

## アクセシビリティ
- 必ずidプロパティを指定してください
- labelとの関連付けに使用されます
- スクリーンリーダーでの適切な読み上げに必要です

## 数値制約
- min: 最小値を設定
- max: 最大値を設定  
- step: 増減のステップ値を設定（整数の場合は1、小数点の場合はanyなど）
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
      control: 'number',
      description: 'Current numeric value of the input'
    },
    min: {
      control: 'number',
      description: 'Minimum allowed value'
    },
    max: {
      control: 'number',
      description: 'Maximum allowed value'
    },
    step: {
      control: 'number',
      description: 'Step value for increment/decrement (use 1 for integers, "any" for decimals)'
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
} satisfies Meta<NumberInput>;

export default meta;
type Story = StoryObj<typeof meta>;

// 基本的な数値入力
export const Default: Story = {
  args: {
    id: 'default-number',
    value: undefined,
    placeholder: '数値を入力してください'
  }
};

// 初期値ありの数値入力
export const WithValue: Story = {
  args: {
    id: 'with-value-number',
    value: 42,
    placeholder: '数値を入力してください'
  }
};

// 最小値・最大値制約付き
export const MinMaxConstraints: Story = {
  args: {
    id: 'minmax-number',
    value: 50,
    min: 0,
    max: 100,
    placeholder: '0-100の範囲で入力'
  }
};

// ステップ値指定（整数のみ）
export const IntegerStep: Story = {
  args: {
    id: 'integer-step',
    value: 10,
    min: 0,
    max: 100,
    step: 5,
    placeholder: '5刻みで入力'
  }
};

// 小数点入力対応
export const DecimalInput: Story = {
  args: {
    id: 'decimal-number',
    value: 3.14,
    min: 0,
    step: 0.01,
    placeholder: '小数点を入力'
  }
};

// 必須項目の数値入力
export const Required: Story = {
  args: {
    id: 'required-number',
    value: undefined,
    placeholder: '必須項目です',
    required: true,
    min: 1
  }
};

// 無効化された数値入力
export const Disabled: Story = {
  args: {
    id: 'disabled-number',
    value: 123,
    placeholder: '数値を入力してください',
    disabled: true
  }
};

// 年齢入力フィールド
export const AgeField: Story = {
  args: {
    id: 'age-input',
    value: undefined,
    min: 0,
    max: 150,
    step: 1,
    placeholder: '年齢を入力',
    required: true
  }
};

// 価格入力フィールド
export const PriceField: Story = {
  args: {
    id: 'price-input',
    value: undefined,
    min: 0,
    step: 0.01,
    placeholder: '価格を入力（例: 1500.00）'
  }
};

// ファイルサイズ入力（MB単位）
export const FileSizeField: Story = {
  args: {
    id: 'filesize-input',
    value: undefined,
    min: 0.1,
    step: 0.1,
    placeholder: 'ファイルサイズ（MB）'
  }
};

// 連番開始番号（BatchRename用途）
export const SequenceStart: Story = {
  args: {
    id: 'sequence-start',
    value: 1,
    min: 0,
    step: 1,
    placeholder: '開始番号'
  }
};

// 連番増分（BatchRename用途）
export const SequenceStep: Story = {
  args: {
    id: 'sequence-step',
    value: 1,
    min: 1,
    step: 1,
    placeholder: '増分'
  }
};

// 連番桁数（BatchRename用途）
export const SequencePadding: Story = {
  args: {
    id: 'sequence-padding',
    value: 3,
    min: 1,
    max: 10,
    step: 1,
    placeholder: '桁数'
  }
};

// カスタムスタイル付き数値入力
export const CustomStyled: Story = {
  args: {
    id: 'custom-number',
    value: undefined,
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
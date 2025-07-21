import type { Meta, StoryObj } from '@storybook/svelte';
import DateInput from '../lib/components/parts/DateInput.svelte';

const meta: Meta<any> = {
  title: 'Components/DateInput',
  component: DateInput as any,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: `
# DateInput Component

再利用可能な日付入力コンポーネントです。フォームの日付入力として使用できます。

## 特徴
- TypeScript完全対応
- HTML5のネイティブdate input要素を使用
- アクセシビリティ対応（id、label連携）
- フォーカス状態の視覚的フィードバック
- 無効化状態の適切な表示
- min/max属性による日付範囲制限
- カスタムスタイリング対応

## 使用方法
\`\`\`svelte
<script>
  import DateInput from '$lib/components/parts/DateInput.svelte';
  
  let value = '';
</script>

<label for="birth-date">生年月日</label>
<DateInput
  id="birth-date"
  bind:value
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
      description: 'Current value of the input (YYYY-MM-DD format)'
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the input is disabled'
    },
    required: {
      control: 'boolean',
      description: 'Whether the input is required'
    },
    min: {
      control: 'text',
      description: 'Minimum date (YYYY-MM-DD format)'
    },
    max: {
      control: 'text',
      description: 'Maximum date (YYYY-MM-DD format)'
    },
    class: {
      control: 'text',
      description: 'Additional CSS classes to apply'
    }
  } as any
} satisfies Meta<DateInput>;

export default meta;
type Story = StoryObj<typeof meta>;

// 基本的な日付入力
export const Default: Story = {
  args: {
    id: 'default-date-input',
    value: ''
  } as any
};

// 初期値ありの日付入力
export const WithValue: Story = {
  args: {
    id: 'with-value-date-input',
    value: '2025-07-21'
  } as any
};

// 必須項目の日付入力
export const Required: Story = {
  args: {
    id: 'required-date-input',
    value: '',
    required: true
  } as any
};

// 無効化された日付入力
export const Disabled: Story = {
  args: {
    id: 'disabled-date-input',
    value: '2025-07-21',
    disabled: true
  } as any
};

// 日付範囲制限付き（生年月日）
export const BirthDate: Story = {
  args: {
    id: 'birth-date-input',
    value: '',
    min: '1900-01-01',
    max: '2025-12-31',
    required: true
  } as any,
  parameters: {
    docs: {
      description: {
        story: '生年月日入力の例。1900年から現在まで選択可能な範囲制限付き'
      }
    }
  }
};

// 未来の日付のみ（予約日）
export const FutureDate: Story = {
  args: {
    id: 'future-date-input',
    value: '',
    min: '2025-07-21',
    max: '2026-12-31'
  } as any,
  parameters: {
    docs: {
      description: {
        story: '予約日入力の例。今日以降の日付のみ選択可能'
      }
    }
  }
};

// 過去の日付のみ（完了日）
export const PastDate: Story = {
  args: {
    id: 'past-date-input',
    value: '',
    min: '2020-01-01',
    max: '2025-07-21'
  } as any,
  parameters: {
    docs: {
      description: {
        story: '完了日入力の例。今日以前の日付のみ選択可能'
      }
    }
  }
};

// メタデータ検索フィルター用
export const MetadataFilter: Story = {
  args: {
    id: 'metadata-filter-date-input',
    value: '',
    class: 'metadata-value-input'
  } as any,
  parameters: {
    docs: {
      description: {
        story: 'メタデータ検索フィルターでの使用例。カスタムクラス付き'
      }
    }
  }
};

// 開始日と終了日のペア例（開始日）
export const StartDate: Story = {
  args: {
    id: 'start-date-input',
    value: '2025-01-01'
  } as any,
  parameters: {
    docs: {
      description: {
        story: '期間指定の開始日入力例'
      }
    }
  }
};

// 開始日と終了日のペア例（終了日）
export const EndDate: Story = {
  args: {
    id: 'end-date-input',
    value: '2025-12-31',
    min: '2025-01-01'
  } as any,
  parameters: {
    docs: {
      description: {
        story: '期間指定の終了日入力例。開始日以降の制限付き'
      }
    }
  }
};
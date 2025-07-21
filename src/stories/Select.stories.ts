import type { Meta, StoryObj } from '@storybook/svelte';
import Select from '../lib/components/parts/Select.svelte';

const meta: Meta<typeof Select> = {
  title: 'Components/Select',
  component: Select,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: `
# Select Component

再利用可能なSelectコンポーネントです。一貫したUI/UXとアクセシビリティを提供します。

## 特徴
- アクセシビリティ対応（ラベルとselectの適切な関連付け）
- カスタマイズ可能なスタイル
- CSS変数を使用したテーマ対応
- 様々なデータ型に対応（string、number、boolean）

## 使用方法
\`\`\`svelte
<script>
  import Select from '$lib/components/parts/Select.svelte';
  
  let selectedValue = '';
  const options = [
    { value: 'option1', label: 'オプション1' },
    { value: 'option2', label: 'オプション2' }
  ];
</script>

<Select
  label="選択してください"
  options={options}
  bind:value={selectedValue}
/>
\`\`\`
        `,
      },
    },
  },
  tags: ['autodocs'],
  argTypes: {
    options: {
      control: 'object',
      description: '選択肢の配列 { value: string | number, label: string }[]',
    },
    value: {
      control: 'text',
      description: '選択されている値',
    },
    placeholder: {
      control: 'text',
      description: 'プレースホルダーテキスト',
    },
    label: {
      control: 'text',
      description: 'ラベルテキスト（省略可）',
    },
    id: {
      control: 'text',
      description: 'HTMLのid属性（省略時は自動生成）',
    },
    className: {
      control: 'text',
      description: '追加するCSSクラス',
    },
    disabled: {
      control: 'boolean',
      description: '無効状態',
    },
    required: {
      control: 'boolean',
      description: '必須入力',
    }
  },
  args: {
    options: [
      { value: 'apple', label: 'りんご' },
      { value: 'banana', label: 'バナナ' },
      { value: 'orange', label: 'オレンジ' },
      { value: 'grape', label: 'ぶどう' }
    ],
    placeholder: '果物を選択してください',
    value: undefined,
    label: undefined,
    className: '',
    disabled: false,
    required: false
  }
};

export default meta;
type Story = StoryObj<typeof meta>;

// 基本的な使用例
export const Default: Story = {
  args: {},
};

// ラベル付き
export const WithLabel: Story = {
  args: {
    label: '好きな果物',
  },
};

// デフォルト値設定済み
export const WithValue: Story = {
  args: {
    label: '好きな果物',
    value: 'apple',
  },
};

// 無効状態
export const Disabled: Story = {
  args: {
    label: '好きな果物',
    value: 'apple',
    disabled: true,
  },
};

// 必須入力
export const Required: Story = {
  args: {
    label: '好きな果物',
    required: true,
  },
};

// 数値を値として使用
export const NumberValues: Story = {
  args: {
    label: 'ページあたりの表示件数',
    options: [
      { value: 10, label: '10件' },
      { value: 20, label: '20件' },
      { value: 50, label: '50件' },
      { value: 100, label: '100件' },
      { value: 200, label: '200件' }
    ],
    value: 20,
    placeholder: '表示件数を選択',
  },
};

// Boolean値の選択
export const BooleanSelect: Story = {
  args: {
    label: '公開状態',
    options: [
      { value: '', label: '選択してください' },
      { value: 'true', label: '公開' },
      { value: 'false', label: '非公開' }
    ],
    placeholder: '公開状態を選択',
  },
};

// 多数の選択肢
export const LargeOptions: Story = {
  args: {
    label: '都道府県',
    options: [
      { value: 'hokkaido', label: '北海道' },
      { value: 'aomori', label: '青森県' },
      { value: 'iwate', label: '岩手県' },
      { value: 'miyagi', label: '宮城県' },
      { value: 'akita', label: '秋田県' },
      { value: 'yamagata', label: '山形県' },
      { value: 'fukushima', label: '福島県' },
      { value: 'ibaraki', label: '茨城県' },
      { value: 'tochigi', label: '栃木県' },
      { value: 'gunma', label: '群馬県' },
      { value: 'saitama', label: '埼玉県' },
      { value: 'chiba', label: '千葉県' },
      { value: 'tokyo', label: '東京都' },
      { value: 'kanagawa', label: '神奈川県' },
    ],
    placeholder: '都道府県を選択してください',
  },
};

// カスタムクラス名
export const CustomClassName: Story = {
  args: {
    label: 'カスタムスタイル',
    className: 'custom-select-style',
  },
};

// 選択肢が空の場合
export const NoOptions: Story = {
  args: {
    label: '選択肢なし',
    options: [],
    placeholder: '選択肢がありません',
  },
};
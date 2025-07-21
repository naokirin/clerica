import type { Meta, StoryObj } from '@storybook/svelte';
import Select from '../lib/components/parts/Select.svelte';

const meta = {
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
    }
  }
} satisfies Meta<typeof Select>;

export default meta;
type Story = StoryObj<typeof meta>;

// デフォルト設定
const defaultArgs = {
  options: [
    { value: 'apple', label: 'りんご' },
    { value: 'banana', label: 'バナナ' },
    { value: 'orange', label: 'オレンジ' },
    { value: 'grape', label: 'ぶどう' }
  ],
  placeholder: '果物を選択してください'
};

// デフォルト（ラベルなし）
export const Default: Story = {
  args: {
    ...defaultArgs
  },
};

// ラベル付き
export const WithLabel: Story = {
  args: {
    ...defaultArgs,
    label: '好きな果物',
  },
};

// デフォルト値設定済み
export const WithValue: Story = {
  args: {
    ...defaultArgs,
    label: '好きな果物',
    value: 'apple',
  },
};

// 無効状態
export const Disabled: Story = {
  args: {
    ...defaultArgs,
    label: '好きな果物',
    value: 'apple',
  },
};

// 必須入力
export const Required: Story = {
  args: {
    ...defaultArgs,
    label: '好きな果物',
    placeholder: '必須項目です',
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

// Boolean値の選択（文字列として扱う）
export const BooleanSelect: Story = {
  args: {
    label: '公開状態',
    options: [
      { value: 'true', label: '公開' },
      { value: 'false', label: '非公開' }
    ],
    placeholder: '公開状態を選択',
  },
};

// 都道府県選択（多くの選択肢の例）
export const ManyOptions: Story = {
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
      { value: 'gunma', label: '群馬県' }
    ],
    placeholder: '都道府県を選択',
  },
};

// カスタムスタイル
export const CustomStyle: Story = {
  args: {
    ...defaultArgs,
    label: 'カスタムスタイル',
    className: 'custom-select-style',
  },
};

// 選択肢なし（エラーハンドリング）
export const EmptyOptions: Story = {
  args: {
    label: '選択肢なし',
    options: [],
    placeholder: '選択肢がありません',
  },
};
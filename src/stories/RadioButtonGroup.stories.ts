import type { Meta, StoryObj } from '@storybook/svelte';
import RadioButtonGroupWrapper from './RadioButtonGroupWrapper.svelte';

const meta = {
  title: 'Components/RadioButtonGroup',
  component: RadioButtonGroupWrapper,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: `
# RadioButtonGroup Component

再利用可能なラジオボタングループコンポーネントです。複数の選択肢から1つを選択する際に使用します。

## 特徴
- タイトル付きのラジオボタングループ
- カスタマイズ可能な選択肢
- 無効化対応
- アクセシビリティ対応

## 使用方法
\`\`\`svelte
<script>
  import RadioButtonGroup from '$lib/components/parts/RadioButtonGroup.svelte';
  
  const options = [
    { value: "option1", label: "選択肢1" },
    { value: "option2", label: "選択肢2" }
  ];
  
  let selectedValue = "option1";
  
  function handleValueChange(value) {
    selectedValue = value;
  }
</script>

<RadioButtonGroup
  title="選択してください"
  {options}
  value={selectedValue}
  onValueChange={handleValueChange}
/>
\`\`\`
        `
      }
    }
  },
  tags: ['autodocs'],
  argTypes: {
    title: {
      control: 'text',
      description: 'ラジオボタングループのタイトル'
    },
    options: {
      control: 'object',
      description: '選択肢の配列（{ value: string, label: string }[]）'
    },
    initialValue: {
      control: 'text',
      description: '初期選択値'
    },
    disabled: {
      control: 'boolean',
      description: 'ラジオボタンを無効化するかどうか'
    },
    onValueChange: {
      action: 'value changed',
      description: '値が変更されたときのコールバック関数'
    }
  }
} satisfies Meta<RadioButtonGroupWrapper>;

export default meta;
type Story = StoryObj<typeof meta>;

// 基本的な使用例
export const Default: Story = {
  args: {
    title: "リネーム方法",
    options: [
      { value: "simple", label: "シンプルリネーム" },
      { value: "regex", label: "正規表現リネーム" }
    ],
    initialValue: "simple",
    disabled: false
  }
};

// メタデータロジック選択の例
export const MetadataLogic: Story = {
  args: {
    title: "検索条件の組み合わせ",
    options: [
      { value: "AND", label: "すべての条件を満たす" },
      { value: "OR", label: "いずれかの条件を満たす" }
    ],
    initialValue: "AND",
    disabled: false
  }
};

// 3つ以上の選択肢の例
export const MultipleOptions: Story = {
  args: {
    title: "表示モード",
    options: [
      { value: "list", label: "リスト表示" },
      { value: "grid", label: "グリッド表示" },
      { value: "compact", label: "コンパクト表示" }
    ],
    initialValue: "list",
    disabled: false
  }
};

// 無効化された状態
export const Disabled: Story = {
  args: {
    title: "選択不可能な状態",
    options: [
      { value: "option1", label: "選択肢1" },
      { value: "option2", label: "選択肢2" }
    ],
    initialValue: "option1",
    disabled: true
  }
};

// ファイル操作の例
export const FileOperation: Story = {
  args: {
    title: "操作種別",
    options: [
      { value: "copy", label: "コピー" },
      { value: "move", label: "移動" },
      { value: "link", label: "リンク作成" }
    ],
    initialValue: "copy",
    disabled: false
  }
};

// ソート順の例
export const SortOrder: Story = {
  args: {
    title: "並び順",
    options: [
      { value: "asc", label: "昇順" },
      { value: "desc", label: "降順" }
    ],
    initialValue: "asc",
    disabled: false
  }
};
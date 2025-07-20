import type { Meta, StoryObj } from '@storybook/svelte';
import Checkbox from '../lib/components/parts/Checkbox.svelte';

const meta: Meta<Checkbox> = {
  title: 'Components/Checkbox',
  component: Checkbox,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    checked: {
      control: 'boolean',
      description: 'チェックボックスの選択状態',
    },
    disabled: {
      control: 'boolean',
      description: 'チェックボックスの無効状態',
    },
    label: {
      control: 'text',
      description: 'チェックボックスのラベルテキスト',
    },
    id: {
      control: 'text',
      description: 'チェックボックスのID（省略時は自動生成）',
    },
    class: {
      control: 'text',
      description: '追加のCSSクラス',
    },
    size: {
      control: 'select',
      options: ['small', 'medium', 'large', 'xlarge'],
      description: 'チェックボックスのサイズ',
    },
  },
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    label: 'デフォルトのチェックボックス',
    checked: false,
    disabled: false,
  },
};

export const Checked: Story = {
  args: {
    label: 'チェック済みのチェックボックス',
    checked: true,
    disabled: false,
  },
};

export const Disabled: Story = {
  args: {
    label: '無効なチェックボックス',
    checked: false,
    disabled: true,
  },
};

export const CheckedDisabled: Story = {
  args: {
    label: 'チェック済み且つ無効なチェックボックス',
    checked: true,
    disabled: true,
  },
};

export const WithoutLabel: Story = {
  args: {
    checked: false,
    disabled: false,
  },
};

export const LongLabel: Story = {
  args: {
    label: '非常に長いラベルテキストを持つチェックボックスです。このようなケースでもレイアウトが適切に表示されることを確認します。',
    checked: false,
    disabled: false,
  },
};

export const Small: Story = {
  args: {
    label: '小さいチェックボックス',
    checked: false,
    disabled: false,
    size: 'small',
  },
};

export const Large: Story = {
  args: {
    label: '大きいチェックボックス',
    checked: false,
    disabled: false,
    size: 'large',
  },
};

export const MultipleCheckboxes: Story = {
  render: () => ({
    Component: Checkbox,
    props: {},
    template: `
      <div style="display: flex; flex-direction: column; gap: 1rem;">
        <Checkbox label="オプション1" checked={false} />
        <Checkbox label="オプション2" checked={true} />
        <Checkbox label="オプション3（無効）" checked={false} disabled={true} />
        <Checkbox label="オプション4（チェック済み・無効）" checked={true} disabled={true} />
      </div>
    `,
  }),
};

export const SizeComparison: Story = {
  render: () => ({
    Component: Checkbox,
    props: {},
    template: `
      <div style="display: flex; flex-direction: column; gap: 1rem;">
        <Checkbox label="Small size" size="small" checked={true} />
        <Checkbox label="Medium size" size="medium" checked={true} />
        <Checkbox label="Large size" size="large" checked={true} />
        <Checkbox label="XLarge size" size="xlarge" checked={true} />
      </div>
    `,
  }),
};
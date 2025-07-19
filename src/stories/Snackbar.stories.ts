import type { Meta, StoryObj } from '@storybook/svelte';
import SnackbarStorybook from './SnackbarStorybook.svelte';

const meta = {
  title: 'Components/Snackbar',
  component: SnackbarStorybook,
  parameters: {
    layout: 'fullscreen',
  },
  tags: ['autodocs'],
  argTypes: {}
} satisfies Meta<SnackbarStorybook>;

export default meta;
type Story = StoryObj<typeof meta>;

// デフォルトストーリー - 基本的な使用方法
export const Default: Story = {};

// 成功メッセージのストーリー
export const Success: Story = {};

// エラーメッセージのストーリー
export const Error: Story = {};

// 警告メッセージのストーリー
export const Warning: Story = {};

// 情報メッセージのストーリー
export const Info: Story = {};

// 長いメッセージのストーリー
export const LongMessage: Story = {};

// 複数メッセージのストーリー
export const MultipleMessages: Story = {};

// インタラクションテスト用のストーリー
export const InteractionTest: Story = {};
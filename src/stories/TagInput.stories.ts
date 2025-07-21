import type { Meta, StoryObj } from '@storybook/svelte';
import TagInput from '../lib/components/parts/TagInput.svelte';

const meta: Meta<any> = {
  title: 'Components/TagInput',
  component: TagInput as any,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: `
# TagInput Component

ファイルにタグを追加・削除するためのタグ入力コンポーネントです。オートコンプリート機能付きです。

## 特徴
- タグの追加・削除
- オートコンプリート機能
- タグの色表示
- キーボードナビゲーション対応

## 使用方法
\`\`\`svelte
<script>
  import TagInput from '$lib/components/parts/TagInput.svelte';
  
  let selectedTags = [];
  let availableTags = [
    { id: '1', name: 'Important', color: '#FF6B6B' },
    { id: '2', name: 'Work', color: '#4ECDC4' }
  ];
</script>

<TagInput
  bind:selectedTags
  {availableTags}
  placeholder="Add tags..."
/>
\`\`\`
        `
      }
    }
  },
  tags: ['autodocs'],
  argTypes: {
    tags: {
      control: 'object',
      description: 'Array of currently selected tags'
    },
    placeholder: {
      control: 'text',
      description: 'Placeholder text for the input'
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the input is disabled'
    },
    onchange: {
      action: 'tags changed',
      description: 'Callback when tags are changed'
    }
  } as any
} satisfies Meta<TagInput>;

export default meta;
type Story = StoryObj<typeof meta>;

const mockTags = [
  { id: '1', name: 'Important', color: '#FF6B6B', created_at: new Date().toISOString() },
  { id: '2', name: 'Work', color: '#4ECDC4', created_at: new Date().toISOString() },
  { id: '3', name: 'Personal', color: '#45B7D1', created_at: new Date().toISOString() }
];

export const Default: Story = {
  args: {
    tags: [],
    placeholder: 'Add tags...',
    disabled: false,
    onchange: (tags: any) => console.log('Tags changed:', tags)
  } as any
};

export const WithSelectedTags: Story = {
  args: {
    tags: mockTags.slice(0, 2),
    placeholder: 'Add more tags...',
    disabled: false,
    onchange: (tags: any) => console.log('Tags changed:', tags)
  } as any
};

export const Disabled: Story = {
  args: {
    tags: mockTags.slice(0, 1),
    placeholder: 'Add tags...',
    disabled: true,
    onchange: (tags: any) => console.log('Tags changed:', tags)
  } as any
};
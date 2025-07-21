import type { Meta, StoryObj } from '@storybook/svelte';
import SortControl from '../lib/components/parts/SortControl.svelte';

const meta: Meta<any> = {
  title: 'Components/SortControl',
  component: SortControl as any,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: `
# SortControl Component

ソート項目とソート順を選択できるコントロールコンポーネントです。

## 特徴
- カスタマイズ可能なソート項目
- 昇順/降順の切り替え
- アクセシビリティ対応
- 国際化対応

## 使用方法
\`\`\`svelte
<script>
  import SortControl from '$lib/components/parts/SortControl.svelte';
  
  const sortOptions = [
    { value: "name", label: "ファイル名" },
    { value: "size", label: "サイズ" }
  ];
  
  let sortField = "name";
  let sortOrder = "asc";
  
  async function handleSortChange(options) {
    sortField = options.field;
    sortOrder = options.order;
  }
</script>

<SortControl 
  {sortField} 
  {sortOrder} 
  {sortOptions}
  onSortChange={handleSortChange} 
/>
\`\`\`
        `
      }
    }
  },
  tags: ['autodocs'],
  argTypes: {
    sortField: {
      control: 'select',
      options: ['name', 'size', 'modified_at', 'created_at', 'file_type', 'last_accessed'],
      description: '現在選択されているソートフィールド'
    },
    sortOrder: {
      control: 'select',
      options: ['asc', 'desc'],
      description: '現在のソート順'
    },
    sortOptions: {
      control: 'object',
      description: 'ソート可能な項目のリスト'
    }
  } as any
} satisfies Meta<SortControl>;

export default meta;
type Story = StoryObj<typeof meta>;

// デフォルトのファイルソート
export const Default: Story = {
  args: {
    sortField: 'modified_at',
    sortOrder: 'desc',
    sortOptions: [
      { value: 'modified_at', label: '更新日時' },
      { value: 'name', label: 'ファイル名' },
      { value: 'size', label: 'サイズ' },
      { value: 'created_at', label: '作成日時' },
      { value: 'last_accessed', label: '最終アクセス' },
      { value: 'file_type', label: 'ファイル種別' }
    ],
    onSortChange: async (options: any) => {
      console.log('Sort changed:', options);
    }
  } as any
};

// 昇順ソート
export const Ascending: Story = {
  args: {
    sortField: 'name',
    sortOrder: 'asc',
    sortOptions: [
      { value: 'name', label: 'ファイル名' },
      { value: 'size', label: 'サイズ' },
      { value: 'modified_at', label: '更新日時' }
    ],
    onSortChange: async (options: any) => {
      console.log('Sort changed:', options);
    }
  } as any
};

// 降順ソート
export const Descending: Story = {
  args: {
    sortField: 'size',
    sortOrder: 'desc',
    sortOptions: [
      { value: 'name', label: 'ファイル名' },
      { value: 'size', label: 'サイズ' },
      { value: 'modified_at', label: '更新日時' }
    ],
    onSortChange: async (options: any) => {
      console.log('Sort changed:', options);
    }
  } as any
};

// 最小限のソート項目
export const MinimalOptions: Story = {
  args: {
    sortField: 'name',
    sortOrder: 'asc',
    sortOptions: [
      { value: 'name', label: 'ファイル名' },
      { value: 'size', label: 'サイズ' }
    ],
    onSortChange: async (options: any) => {
      console.log('Sort changed:', options);
    }
  } as any
};

// 写真用ソート項目
export const PhotoSort: Story = {
  args: {
    sortField: 'created_at',
    sortOrder: 'desc',
    sortOptions: [
      { value: 'created_at', label: '撮影日時' },
      { value: 'name', label: 'ファイル名' },
      { value: 'size', label: 'ファイルサイズ' },
      { value: 'file_type', label: 'ファイル形式' }
    ],
    onSortChange: async (options: any) => {
      console.log('Sort changed:', options);
    }
  } as any
};

// ドキュメント用ソート項目
export const DocumentSort: Story = {
  args: {
    sortField: 'modified_at',
    sortOrder: 'desc',
    sortOptions: [
      { value: 'modified_at', label: '更新日時' },
      { value: 'name', label: 'ドキュメント名' },
      { value: 'created_at', label: '作成日時' },
      { value: 'size', label: 'ファイルサイズ' },
      { value: 'file_type', label: 'ドキュメント種別' }
    ],
    onSortChange: async (options: any) => {
      console.log('Sort changed:', options);
    }
  } as any
};
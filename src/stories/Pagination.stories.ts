import type { Meta, StoryObj } from '@storybook/svelte';
import Pagination from '../lib/components/parts/Pagination.svelte';

const meta = {
  title: 'Components/Pagination',
  component: Pagination,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: `
# Pagination Component

大量のファイルリストを複数のページに分割して表示するためのページネーションコンポーネントです。

## 特徴
- 前へ・次へボタン
- ページ番号の直接指定
- 現在のページとアイテム情報の表示
- レスポンシブデザイン

## 使用方法
\`\`\`svelte
<script>
  import Pagination from '$lib/components/parts/Pagination.svelte';
  
  let currentPage = 1;
  let totalPages = 10;
  let totalItems = 100;
  let itemsPerPage = 10;
</script>

<Pagination
  bind:currentPage
  {totalPages}
  {totalItems}
  {itemsPerPage}
/>
\`\`\`
        `
      }
    }
  },
  tags: ['autodocs'],
  argTypes: {
    currentPage: {
      control: 'number',
      description: 'Current active page number'
    },
    totalPages: {
      control: 'number',
      description: 'Total number of pages'
    },
    maxVisiblePages: {
      control: 'number',
      description: 'Maximum number of page buttons to show'
    },
    disabled: {
      control: 'boolean',
      description: 'Whether pagination is disabled'
    }
  }
} satisfies Meta<typeof Pagination>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    currentPage: 1,
    totalPages: 10,
    maxVisiblePages: 5,
    disabled: false,
    onGoToPage: (page: number) => console.log(`Go to page ${page}`),
    onGoToPreviousPage: () => console.log('Go to previous page'),
    onGoToNextPage: () => console.log('Go to next page'),
    onGoToFirstPage: () => console.log('Go to first page'),
    onGoToLastPage: () => console.log('Go to last page')
  }
};

export const MiddlePage: Story = {
  args: {
    currentPage: 5,
    totalPages: 10,
    maxVisiblePages: 5,
    disabled: false,
    onGoToPage: (page: number) => console.log(`Go to page ${page}`),
    onGoToPreviousPage: () => console.log('Go to previous page'),
    onGoToNextPage: () => console.log('Go to next page'),
    onGoToFirstPage: () => console.log('Go to first page'),
    onGoToLastPage: () => console.log('Go to last page')
  }
};

export const LastPage: Story = {
  args: {
    currentPage: 10,
    totalPages: 10,
    maxVisiblePages: 5,
    disabled: false,
    onGoToPage: (page: number) => console.log(`Go to page ${page}`),
    onGoToPreviousPage: () => console.log('Go to previous page'),
    onGoToNextPage: () => console.log('Go to next page'),
    onGoToFirstPage: () => console.log('Go to first page'),
    onGoToLastPage: () => console.log('Go to last page')
  }
};

export const Disabled: Story = {
  args: {
    currentPage: 5,
    totalPages: 10,
    maxVisiblePages: 5,
    disabled: true,
    onGoToPage: (page: number) => console.log(`Go to page ${page}`),
    onGoToPreviousPage: () => console.log('Go to previous page'),
    onGoToNextPage: () => console.log('Go to next page'),
    onGoToFirstPage: () => console.log('Go to first page'),
    onGoToLastPage: () => console.log('Go to last page')
  }
};
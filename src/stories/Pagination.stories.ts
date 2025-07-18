import type { Meta, StoryObj } from '@storybook/svelte';
import Pagination from '../lib/components/Pagination.svelte';

const meta = {
  title: 'Components/Pagination',
  component: Pagination,
  parameters: {
    layout: 'centered',
  },
  argTypes: {
    currentPage: {
      control: { type: 'number', min: 1 },
      description: 'Current active page number'
    },
    totalPages: {
      control: { type: 'number', min: 1 },
      description: 'Total number of pages'
    },
    maxVisiblePages: {
      control: { type: 'number', min: 3 },
      description: 'Maximum number of page buttons to show'
    },
    showFirstLast: {
      control: 'boolean',
      description: 'Whether to show first and last page buttons'
    },
    disabled: {
      control: 'boolean',
      description: 'Whether pagination is disabled'
    }
  }
} satisfies Meta<Pagination>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    currentPage: 1,
    totalPages: 10,
    maxVisiblePages: 5,
    showFirstLast: true,
    disabled: false
  }
};

export const MiddlePage: Story = {
  args: {
    currentPage: 5,
    totalPages: 10,
    maxVisiblePages: 5,
    showFirstLast: true,
    disabled: false
  }
};

export const LastPage: Story = {
  args: {
    currentPage: 10,
    totalPages: 10,
    maxVisiblePages: 5,
    showFirstLast: true,
    disabled: false
  }
};

export const Disabled: Story = {
  args: {
    currentPage: 5,
    totalPages: 10,
    maxVisiblePages: 5,
    showFirstLast: true,
    disabled: true
  }
};
import type { Meta, StoryObj } from '@storybook/svelte';
import TagInput from '../lib/components/parts/TagInput.svelte';

const meta = {
  title: 'Components/TagInput',
  component: TagInput,
  parameters: {
    layout: 'centered',
  },
  argTypes: {
    selectedTags: {
      control: 'object',
      description: 'Array of currently selected tags'
    },
    availableTags: {
      control: 'object',
      description: 'Array of available tags for autocomplete'
    },
    placeholder: {
      control: 'text',
      description: 'Placeholder text for the input'
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the input is disabled'
    }
  }
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
    selectedTags: [],
    availableTags: mockTags,
    placeholder: 'Add tags...',
    disabled: false
  }
};

export const WithSelectedTags: Story = {
  args: {
    selectedTags: mockTags.slice(0, 2),
    availableTags: mockTags,
    placeholder: 'Add more tags...',
    disabled: false
  }
};

export const Disabled: Story = {
  args: {
    selectedTags: mockTags.slice(0, 1),
    availableTags: mockTags,
    placeholder: 'Add tags...',
    disabled: true
  }
};
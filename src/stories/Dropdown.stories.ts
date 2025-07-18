import type { Meta, StoryObj } from '@storybook/svelte';
import Dropdown from '../lib/components/Dropdown.svelte';

const meta = {
  title: 'Components/Dropdown',
  component: Dropdown,
  parameters: {
    layout: 'centered',
  },
  argTypes: {
    disabled: {
      control: 'boolean',
      description: 'Whether the dropdown is disabled'
    },
    position: {
      control: 'select',
      options: ['left', 'right'],
      description: 'Position of the dropdown menu relative to the trigger'
    }
  }
} satisfies Meta<Dropdown>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    disabled: false,
    position: "left",
    triggerClass: "",
    menuClass: ""
  }
};

export const Disabled: Story = {
  args: {
    disabled: true,
    position: 'right'
  }
};

export const LeftPosition: Story = {
  args: {
    disabled: false,
    position: 'left'
  }
};

export const RightPosition: Story = {
  args: {
    disabled: false,
    position: 'right'
  }
};
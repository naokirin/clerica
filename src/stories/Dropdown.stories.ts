import { createRawSnippet } from 'svelte';
import type { Meta, StoryObj } from '@storybook/svelte';
import Dropdown from '../lib/components/parts/Dropdown.svelte';

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
    },
    triggerClass: {
      control: 'text',
      description: 'Additional CSS classes for the trigger button'
    },
    menuClass: {
      control: 'text',
      description: 'Additional CSS classes for the dropdown menu'
    },
    menuItem1Text: {
      control: 'text',
      description: 'Text for the first menu item'
    },
    menuItem2Text: {
      control: 'text',
      description: 'Text for the second menu item'
    },
    menuItem3Text: {
      control: 'text',
      description: 'Text for the third menu item'
    }
  }
} satisfies Meta<any>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    disabled: false,
    position: "left",
    triggerClass: "",
    menuClass: "",
    menuItem1Text: "編集",
    menuItem2Text: "コピー",
    menuItem3Text: "削除"
  },
  render: (args: any) => ({
    Component: Dropdown,
    props: {
      disabled: args.disabled,
      position: args.position,
      triggerClass: args.triggerClass,
      menuClass: args.menuClass,
      children: createRawSnippet(() => {
        return {
          render: () => `
            <div>
              <button class="dropdown-menu-item">
                <svg class="icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
                  <path d="m18.5 2.5 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
                </svg>
                ${args.menuItem1Text}
              </button>
              <button class="dropdown-menu-item">
                <svg class="icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>
                  <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>
                </svg>
                ${args.menuItem2Text}
              </button>
              <button class="dropdown-menu-item danger">
                <svg class="icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M3 6h18"/>
                  <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/>
                  <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/>
                </svg>
                ${args.menuItem3Text}
              </button>
            </div>
          `
        }
      })
    }
  })
};

export const Disabled: Story = {
  args: {
    disabled: true,
    position: 'right',
    menuItem1Text: "編集",
    menuItem2Text: "コピー",
    menuItem3Text: "削除"
  },
  render: (args: any) => ({
    Component: Dropdown,
    props: {
      disabled: args.disabled,
      position: args.position,
      triggerClass: args.triggerClass,
      menuClass: args.menuClass,
      children: createRawSnippet(() => {
        return {
          render: () => `
            <div>
              <button class="dropdown-menu-item">
                <svg class="icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
                  <path d="m18.5 2.5 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
                </svg>
                ${args.menuItem1Text}
              </button>
              <button class="dropdown-menu-item">
                <svg class="icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>
                  <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>
                </svg>
                ${args.menuItem2Text}
              </button>
              <button class="dropdown-menu-item danger">
                <svg class="icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M3 6h18"/>
                  <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/>
                  <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/>
                </svg>
                ${args.menuItem3Text}
              </button>
            </div>
          `
        }
      })
    }
  })
};

export const LeftPosition: Story = {
  args: {
    disabled: false,
    position: 'left',
    menuItem1Text: "詳細表示",
    menuItem2Text: "エクスポート",
    menuItem3Text: "アーカイブ"
  },
  render: (args: any) => ({
    Component: Dropdown,
    props: {
      disabled: args.disabled,
      position: args.position,
      triggerClass: args.triggerClass,
      menuClass: args.menuClass,
      children: createRawSnippet(() => {
        return {
          render: () => `
            <div>
              <button class="dropdown-menu-item">
                <svg class="icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="11" cy="11" r="8"/>
                  <path d="m21 21-4.35-4.35"/>
                </svg>
                ${args.menuItem1Text}
              </button>
              <button class="dropdown-menu-item">
                <svg class="icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                  <polyline points="7,10 12,15 17,10"/>
                  <line x1="12" x2="12" y1="15" y2="3"/>
                </svg>
                ${args.menuItem2Text}
              </button>
              <button class="dropdown-menu-item">
                <svg class="icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M20 7h-9"/>
                  <path d="M14 17H5"/>
                  <path d="M17 21V3"/>
                  <path d="M7 21V3"/>
                </svg>
                ${args.menuItem3Text}
              </button>
            </div>
          `
        }
      })
    }
  })
};

export const RightPosition: Story = {
  args: {
    disabled: false,
    position: 'right',
    menuItem1Text: "共有",
    menuItem2Text: "印刷",
    menuItem3Text: "設定"
  },
  render: (args: any) => ({
    Component: Dropdown,
    props: {
      disabled: args.disabled,
      position: args.position,
      triggerClass: args.triggerClass,
      menuClass: args.menuClass,
      children: createRawSnippet(() => {
        return {
          render: () => `
            <div>
              <button class="dropdown-menu-item">
                <svg class="icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"/>
                  <polyline points="16,6 12,2 8,6"/>
                  <line x1="12" x2="12" y1="2" y2="15"/>
                </svg>
                ${args.menuItem1Text}
              </button>
              <button class="dropdown-menu-item">
                <svg class="icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="6,9 6,2 18,2 18,9"/>
                  <path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"/>
                  <rect width="8" height="8" x="8" y="14"/>
                </svg>
                ${args.menuItem2Text}
              </button>
              <button class="dropdown-menu-item">
                <svg class="icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/>
                  <circle cx="12" cy="12" r="3"/>
                </svg>
                ${args.menuItem3Text}
              </button>
            </div>
          `
        }
      })
    }
  })
};
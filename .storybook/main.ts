import type { StorybookConfig } from '@storybook/sveltekit';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const config: StorybookConfig = {
  "stories": [
    "../src/stories/**/*.mdx",
    "../src/stories/**/*.stories.@(js|ts)"
  ],
  "addons": [
    "@storybook/addon-essentials"
  ],
  "framework": {
    "name": "@storybook/sveltekit",
    "options": {}
  },
  "staticDirs": ["../static"],
  async viteFinal(config) {
    if (config.resolve) {
      config.resolve.alias = {
        ...config.resolve.alias,
        $lib: path.resolve(__dirname, '../src/lib'),
        $app: path.resolve(__dirname, '../src/app'),
      };
    }
    return config;
  },
};
export default config;
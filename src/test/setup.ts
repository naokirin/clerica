import { vi, expect } from 'vitest';
import '@testing-library/jest-dom';

// Mock Tauri API
Object.defineProperty(window, '__TAURI__', {
  value: {
    invoke: vi.fn(),
    event: {
      listen: vi.fn(),
      emit: vi.fn(),
    },
    tauri: {
      invoke: vi.fn(),
    },
  },
});

// Mock for Tauri core
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue({}),
}));

// Mock for Tauri plugins
vi.mock('@tauri-apps/plugin-sql', () => ({
  Database: {
    load: vi.fn().mockResolvedValue({
      select: vi.fn().mockResolvedValue([]),
      execute: vi.fn().mockResolvedValue(true),
    }),
  },
}));

vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn().mockResolvedValue('/mock/path'),
  save: vi.fn().mockResolvedValue('/mock/save/path'),
}));

// Mock for Fuse.js
vi.mock('fuse', () => ({
  default: vi.fn().mockImplementation(() => ({
    search: vi.fn().mockReturnValue([]),
    setCollection: vi.fn(),
  })),
}));

// Mock fetch API
global.fetch = vi.fn().mockResolvedValue({
  ok: true,
  json: vi.fn().mockResolvedValue({}),
  text: vi.fn().mockResolvedValue(''),
});

// Mock console methods to reduce test noise
const originalConsole = console;
global.console = {
  ...originalConsole,
  log: vi.fn(),
  warn: vi.fn(),
  error: vi.fn(),
  info: vi.fn(),
  debug: vi.fn(),
};
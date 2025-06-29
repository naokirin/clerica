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
  },
});

// Mock for Tauri plugins
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

vi.mock('@tauri-apps/plugin-sql', () => ({
  Database: {
    load: vi.fn(),
  },
}));
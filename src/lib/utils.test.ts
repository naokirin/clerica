import { describe, it, expect, vi } from 'vitest';
import { 
  formatFileSize, 
  formatDate, 
  getFileTypeIcon, 
  truncateText,
  debounce,
  validatePath,
  generateColorFromString,
  sortByProperty
} from './utils';

describe('formatFileSize', () => {
  it('should format bytes correctly', () => {
    expect(formatFileSize(0)).toBe('0 Bytes');
    expect(formatFileSize(1024)).toBe('1 KB');
    expect(formatFileSize(1048576)).toBe('1 MB');
    expect(formatFileSize(1073741824)).toBe('1 GB');
  });

  it('should handle decimal values', () => {
    expect(formatFileSize(1536)).toBe('1.5 KB');
    expect(formatFileSize(2097152)).toBe('2 MB');
  });

  it('should handle large numbers', () => {
    expect(formatFileSize(1073741824 * 1024)).toBe('1 TB');
  });
});

describe('formatDate', () => {
  it('should format valid date strings', () => {
    const date = '2023-12-25T10:30:00Z';
    const result = formatDate(date);
    expect(result).toContain('2023');
  });

  it('should handle null input', () => {
    expect(formatDate(null)).toBe('N/A');
  });

  it('should handle invalid date strings', () => {
    expect(formatDate('invalid-date')).toBe('Invalid Date');
  });

  it('should handle empty string', () => {
    expect(formatDate('')).toBe('N/A');
  });
});

describe('getFileTypeIcon', () => {
  it('should return correct icons for known file types', () => {
    expect(getFileTypeIcon('pdf')).toBe('📄');
    expect(getFileTypeIcon('jpg')).toBe('🖼️');
    expect(getFileTypeIcon('mp3')).toBe('🎵');
    expect(getFileTypeIcon('zip')).toBe('🗜️');
  });

  it('should handle case insensitive file types', () => {
    expect(getFileTypeIcon('PDF')).toBe('📄');
    expect(getFileTypeIcon('JPG')).toBe('🖼️');
  });

  it('should return default icon for unknown types', () => {
    expect(getFileTypeIcon('unknown')).toBe('📄');
    expect(getFileTypeIcon(null)).toBe('📄');
  });
});

describe('truncateText', () => {
  it('should truncate long text', () => {
    const longText = 'This is a very long text that needs to be truncated';
    expect(truncateText(longText, 10)).toBe('This is a ...');
  });

  it('should not truncate short text', () => {
    const shortText = 'Short';
    expect(truncateText(shortText, 10)).toBe('Short');
  });

  it('should handle edge cases', () => {
    expect(truncateText('', 10)).toBe('');
    expect(truncateText('test', 4)).toBe('test');
    expect(truncateText('test', 3)).toBe('tes...');
  });
});

describe('debounce', () => {
  it('should debounce function calls', async () => {
    const mockFn = vi.fn();
    const debouncedFn = debounce(mockFn, 100);

    debouncedFn();
    debouncedFn();
    debouncedFn();

    expect(mockFn).not.toHaveBeenCalled();

    await new Promise(resolve => setTimeout(resolve, 150));
    expect(mockFn).toHaveBeenCalledTimes(1);
  });

  it('should pass arguments correctly', async () => {
    const mockFn = vi.fn();
    const debouncedFn = debounce(mockFn, 50);

    debouncedFn('arg1', 'arg2');

    await new Promise(resolve => setTimeout(resolve, 100));
    expect(mockFn).toHaveBeenCalledWith('arg1', 'arg2');
  });
});

describe('validatePath', () => {
  it('should validate correct paths', () => {
    expect(validatePath('/home/user/documents')).toBe(true);
    expect(validatePath('~/Desktop/folder')).toBe(true);
    expect(validatePath('/usr/local/bin')).toBe(true);
  });

  it('should reject invalid paths', () => {
    expect(validatePath('')).toBe(false);
    expect(validatePath('   ')).toBe(false);
    expect(validatePath('<script>alert("xss")</script>')).toBe(false);
  });

  it('should handle edge cases', () => {
    expect(validatePath('/single')).toBe(true);
    expect(validatePath('relative/path')).toBe(true);
  });
});

describe('generateColorFromString', () => {
  it('should generate consistent colors for same input', () => {
    const color1 = generateColorFromString('test');
    const color2 = generateColorFromString('test');
    expect(color1).toBe(color2);
  });

  it('should generate different colors for different inputs', () => {
    const color1 = generateColorFromString('test1');
    const color2 = generateColorFromString('test2');
    expect(color1).not.toBe(color2);
  });

  it('should return valid HSL color format', () => {
    const color = generateColorFromString('test');
    expect(color).toMatch(/^hsl\(\d+, 70%, 50%\)$/);
  });
});

describe('sortByProperty', () => {
  const testArray = [
    { name: 'Charlie', age: 30, score: null },
    { name: 'Alice', age: 25, score: 95 },
    { name: 'Bob', age: 35, score: 85 }
  ];

  it('should sort by string property ascending', () => {
    const sorted = sortByProperty(testArray, 'name', 'asc');
    expect(sorted.map(item => item.name)).toEqual(['Alice', 'Bob', 'Charlie']);
  });

  it('should sort by string property descending', () => {
    const sorted = sortByProperty(testArray, 'name', 'desc');
    expect(sorted.map(item => item.name)).toEqual(['Charlie', 'Bob', 'Alice']);
  });

  it('should sort by number property ascending', () => {
    const sorted = sortByProperty(testArray, 'age', 'asc');
    expect(sorted.map(item => item.age)).toEqual([25, 30, 35]);
  });

  it('should sort by number property descending', () => {
    const sorted = sortByProperty(testArray, 'age', 'desc');
    expect(sorted.map(item => item.age)).toEqual([35, 30, 25]);
  });

  it('should handle null values', () => {
    const sorted = sortByProperty(testArray, 'score', 'asc');
    expect(sorted.map(item => item.score)).toEqual([85, 95, null]);
  });

  it('should not mutate original array', () => {
    const original = [...testArray];
    sortByProperty(testArray, 'name', 'asc');
    expect(testArray).toEqual(original);
  });
});
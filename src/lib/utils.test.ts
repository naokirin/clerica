import { describe, it, expect, vi } from 'vitest';
import { 
  formatFileSize, 
  formatDate, 
  getFileTypeIcon, 
  truncateText,
  debounce,
  validatePath,
  generateColorFromString,
  sortByProperty,
  getFileCategory
} from './utils';
import type { File } from './types';

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

  it('should handle undefined values', () => {
    const arrayWithUndefined = [
      { name: 'Alice', value: undefined },
      { name: 'Bob', value: 10 },
      { name: 'Charlie', value: 5 }
    ];
    const sorted = sortByProperty(arrayWithUndefined, 'value', 'asc');
    expect(sorted.map(item => item.value)).toEqual([5, 10, undefined]);
  });

  it('should handle mixed types gracefully', () => {
    const mixedArray = [
      { id: 1, value: 'string' },
      { id: 2, value: 42 },
      { id: 3, value: 'another' }
    ];
    const sorted = sortByProperty(mixedArray, 'value', 'asc');
    expect(sorted).toHaveLength(3);
  });
});

describe('getFileCategory', () => {
  it('should return "other" for directory files', () => {
    const dirFile: File = {
      id: '1',
      name: 'folder',
      path: '/test/folder',
      directory_id: 'dir1',
      size: 0,
      file_type: null,
      created_at: '2024-01-01',
      modified_at: '2024-01-01',
      birth_time: null,
      inode: null,
      is_directory: true,
      created_at_db: '2024-01-01T00:00:00Z',
      updated_at_db: '2024-01-01T00:00:00Z',
      file_size: null,
      mime_type: null,
      permissions: null,
      owner_uid: null,
      group_gid: null,
      hard_links: null,
      device_id: null,
      last_accessed: null,
      metadata: null
    };
    expect(getFileCategory(dirFile)).toBe('other');
  });

  it('should categorize image files by mime type', () => {
    const imageFile: File = {
      id: '1',
      name: 'photo.jpg',
      path: '/test/photo.jpg',
      directory_id: 'dir1',
      size: 1024,
      file_type: null,
      created_at: '2024-01-01',
      modified_at: '2024-01-01',
      birth_time: null,
      inode: null,
      is_directory: false,
      created_at_db: '2024-01-01T00:00:00Z',
      updated_at_db: '2024-01-01T00:00:00Z',
      file_size: 1024,
      mime_type: 'image/jpeg',
      permissions: null,
      owner_uid: null,
      group_gid: null,
      hard_links: null,
      device_id: null,
      last_accessed: null,
      metadata: null
    };
    expect(getFileCategory(imageFile)).toBe('image');
  });

  it('should categorize image files by extension', () => {
    const imageFile: File = {
      id: '1',
      name: 'photo.png',
      path: '/test/photo.png',
      directory_id: 'dir1',
      size: 1024,
      file_type: 'png',
      created_at: '2024-01-01',
      modified_at: '2024-01-01',
      birth_time: null,
      inode: null,
      is_directory: false,
      created_at_db: '2024-01-01T00:00:00Z',
      updated_at_db: '2024-01-01T00:00:00Z',
      file_size: 1024,
      mime_type: null,
      permissions: null,
      owner_uid: null,
      group_gid: null,
      hard_links: null,
      device_id: null,
      last_accessed: null,
      metadata: null
    };
    expect(getFileCategory(imageFile)).toBe('image');
  });

  it('should categorize audio files', () => {
    const audioFile: File = {
      id: '1',
      name: 'song.mp3',
      path: '/test/song.mp3',
      directory_id: 'dir1',
      size: 2048,
      file_type: 'mp3',
      created_at: '2024-01-01',
      modified_at: '2024-01-01',
      birth_time: null,
      inode: null,
      is_directory: false,
      created_at_db: '2024-01-01T00:00:00Z',
      updated_at_db: '2024-01-01T00:00:00Z',
      file_size: 2048,
      mime_type: 'audio/mpeg',
      permissions: null,
      owner_uid: null,
      group_gid: null,
      hard_links: null,
      device_id: null,
      last_accessed: null,
      metadata: null
    };
    expect(getFileCategory(audioFile)).toBe('audio');
  });

  it('should categorize video files', () => {
    const videoFile: File = {
      id: '1',
      name: 'movie.mp4',
      path: '/test/movie.mp4',
      directory_id: 'dir1',
      size: 10240,
      file_type: 'mp4',
      created_at: '2024-01-01',
      modified_at: '2024-01-01',
      birth_time: null,
      inode: null,
      is_directory: false,
      created_at_db: '2024-01-01T00:00:00Z',
      updated_at_db: '2024-01-01T00:00:00Z',
      file_size: 10240,
      mime_type: 'video/mp4',
      permissions: null,
      owner_uid: null,
      group_gid: null,
      hard_links: null,
      device_id: null,
      last_accessed: null,
      metadata: null
    };
    expect(getFileCategory(videoFile)).toBe('video');
  });

  it('should categorize document files', () => {
    const docFile: File = {
      id: '1',
      name: 'document.pdf',
      path: '/test/document.pdf',
      directory_id: 'dir1',
      size: 5120,
      file_type: 'pdf',
      created_at: '2024-01-01',
      modified_at: '2024-01-01',
      birth_time: null,
      inode: null,
      is_directory: false,
      created_at_db: '2024-01-01T00:00:00Z',
      updated_at_db: '2024-01-01T00:00:00Z',
      file_size: 5120,
      mime_type: 'application/pdf',
      permissions: null,
      owner_uid: null,
      group_gid: null,
      hard_links: null,
      device_id: null,
      last_accessed: null,
      metadata: null
    };
    expect(getFileCategory(docFile)).toBe('document');
  });

  it('should categorize archive files', () => {
    const archiveFile: File = {
      id: '1',
      name: 'archive.zip',
      path: '/test/archive.zip',
      directory_id: 'dir1',
      size: 15360,
      file_type: 'zip',
      created_at: '2024-01-01',
      modified_at: '2024-01-01',
      birth_time: null,
      inode: null,
      is_directory: false,
      created_at_db: '2024-01-01T00:00:00Z',
      updated_at_db: '2024-01-01T00:00:00Z',
      file_size: 15360,
      mime_type: 'application/zip',
      permissions: null,
      owner_uid: null,
      group_gid: null,
      hard_links: null,
      device_id: null,
      last_accessed: null,
      metadata: null
    };
    expect(getFileCategory(archiveFile)).toBe('archive');
  });

  it('should return "other" for unknown file types', () => {
    const unknownFile: File = {
      id: '1',
      name: 'unknown.xyz',
      path: '/test/unknown.xyz',
      directory_id: 'dir1',
      size: 1024,
      file_type: 'xyz',
      created_at: '2024-01-01',
      modified_at: '2024-01-01',
      birth_time: null,
      inode: null,
      is_directory: false,
      created_at_db: '2024-01-01T00:00:00Z',
      updated_at_db: '2024-01-01T00:00:00Z',
      file_size: 1024,
      mime_type: 'application/unknown',
      permissions: null,
      owner_uid: null,
      group_gid: null,
      hard_links: null,
      device_id: null,
      last_accessed: null,
      metadata: null
    };
    expect(getFileCategory(unknownFile)).toBe('other');
  });

  it('should handle files without mime type or extension', () => {
    const fileWithoutType: File = {
      id: '1',
      name: 'noextension',
      path: '/test/noextension',
      directory_id: 'dir1',
      size: 512,
      file_type: null,
      created_at: '2024-01-01',
      modified_at: '2024-01-01',
      birth_time: null,
      inode: null,
      is_directory: false,
      created_at_db: '2024-01-01T00:00:00Z',
      updated_at_db: '2024-01-01T00:00:00Z',
      file_size: 512,
      mime_type: null,
      permissions: null,
      owner_uid: null,
      group_gid: null,
      hard_links: null,
      device_id: null,
      last_accessed: null,
      metadata: null
    };
    expect(getFileCategory(fileWithoutType)).toBe('other');
  });

  it('should handle case insensitive matching', () => {
    const upperCaseFile: File = {
      id: '1',
      name: 'IMAGE.JPG',
      path: '/test/IMAGE.JPG',
      directory_id: 'dir1',
      size: 1024,
      file_type: 'JPG',
      created_at: '2024-01-01',
      modified_at: '2024-01-01',
      birth_time: null,
      inode: null,
      is_directory: false,
      created_at_db: '2024-01-01T00:00:00Z',
      updated_at_db: '2024-01-01T00:00:00Z',
      file_size: 1024,
      mime_type: 'IMAGE/JPEG',
      permissions: null,
      owner_uid: null,
      group_gid: null,
      hard_links: null,
      device_id: null,
      last_accessed: null,
      metadata: null
    };
    expect(getFileCategory(upperCaseFile)).toBe('image');
  });

  it('should prioritize mime type over extension', () => {
    const conflictFile: File = {
      id: '1',
      name: 'file.txt',
      path: '/test/file.txt',
      directory_id: 'dir1',
      size: 1024,
      file_type: 'txt',
      created_at: '2024-01-01',
      modified_at: '2024-01-01',
      birth_time: null,
      inode: null,
      is_directory: false,
      created_at_db: '2024-01-01T00:00:00Z',
      updated_at_db: '2024-01-01T00:00:00Z',
      file_size: 1024,
      mime_type: 'image/png',
      permissions: null,
      owner_uid: null,
      group_gid: null,
      hard_links: null,
      device_id: null,
      last_accessed: null,
      metadata: null
    };
    expect(getFileCategory(conflictFile)).toBe('image');
  });
});
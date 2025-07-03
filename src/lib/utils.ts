import { convertFileSrc } from '@tauri-apps/api/core';

export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes';

  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  // sizesの範囲を超えないように制限
  const sizeIndex = Math.min(i, sizes.length - 1);

  return parseFloat((bytes / Math.pow(k, sizeIndex)).toFixed(2)) + ' ' + sizes[sizeIndex];
}

export function formatDate(dateString: string | null): string {
  if (!dateString) return 'N/A';

  try {
    const date = new Date(dateString);
    return date.toLocaleString('ja-JP');
  } catch (error) {
    return 'Invalid Date';
  }
}

export function getFileTypeIcon(fileType: string | null): string {
  if (!fileType) return '📄';

  const iconMap: Record<string, string> = {
    'pdf': '📄',
    'txt': '📝',
    'md': '📝',
    'doc': '📄',
    'docx': '📄',
    'xls': '📊',
    'xlsx': '📊',
    'ppt': '📊',
    'pptx': '📊',
    'jpg': '🖼️',
    'jpeg': '🖼️',
    'png': '🖼️',
    'gif': '🖼️',
    'svg': '🖼️',
    'mp4': '🎬',
    'mov': '🎬',
    'avi': '🎬',
    'mp3': '🎵',
    'wav': '🎵',
    'm4a': '🎵',
    'zip': '🗜️',
    'rar': '🗜️',
    '7z': '🗜️',
    'tar': '🗜️',
    'gz': '🗜️',
    'js': '⚡',
    'ts': '⚡',
    'jsx': '⚡',
    'tsx': '⚡',
    'html': '🌐',
    'css': '🎨',
    'scss': '🎨',
    'json': '📋',
    'xml': '📋',
    'csv': '📊',
  };

  return iconMap[fileType.toLowerCase()] || '📄';
}

export function isImageFile(file: { mime_type?: string | null; name: string }): boolean {
  // MIMEタイプで判定
  if (file.mime_type?.startsWith('image/')) {
    return true;
  }

  // 拡張子で判定
  const imageExtensions = ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'svg', 'ico', 'tiff', 'raw'];
  const extension = file.name.split('.').pop()?.toLowerCase();

  return extension ? imageExtensions.includes(extension) : false;
}

export function isVideoFile(file: { mime_type?: string | null; name: string }): boolean {
  // MIMEタイプで判定
  if (file.mime_type?.startsWith('video/')) {
    return true;
  }

  // 拡張子で判定
  const videoExtensions = ['mp4', 'avi', 'mov', 'wmv', 'flv', 'webm', 'mkv', 'm4v', '3gp'];
  const extension = file.name.split('.').pop()?.toLowerCase();

  return extension ? videoExtensions.includes(extension) : false;
}

export function isAudioFile(file: { mime_type?: string | null; name: string }): boolean {
  // MIMEタイプで判定
  if (file.mime_type?.startsWith('audio/')) {
    return true;
  }

  // 拡張子で判定
  const audioExtensions = ['mp3', 'wav', 'ogg', 'flac', 'aac', 'm4a', 'wma', 'opus'];
  const extension = file.name.split('.').pop()?.toLowerCase();

  return extension ? audioExtensions.includes(extension) : false;
}

export function isArchiveFile(file: { mime_type?: string | null; name: string }): boolean {
  // MIMEタイプで判定
  if (file.mime_type?.includes('zip') || 
      file.mime_type?.includes('rar') || 
      file.mime_type?.includes('7z') || 
      file.mime_type?.includes('tar') || 
      file.mime_type?.includes('gzip')) {
    return true;
  }

  // 拡張子で判定
  const archiveExtensions = ['zip', 'rar', '7z', 'tar', 'gz', 'bz2', 'xz', 'lzma'];
  const extension = file.name.split('.').pop()?.toLowerCase();

  return extension ? archiveExtensions.includes(extension) : false;
}

export async function getImageUrl(filePath: string): Promise<string> {
  try {
    // Tauriの convertFileSrc を使用してファイルパスをURLに変換
    // v2では asset プロトコルが使用される
    const url = convertFileSrc(filePath);
    console.log('Converted file URL:', url);
    return url;
  } catch (error) {
    console.error('Failed to convert file src:', error);
    throw error;
  }
}

export function truncateText(text: string, maxLength: number): string {
  if (text.length <= maxLength) return text;
  return text.substring(0, maxLength) + '...';
}

export function debounce<T extends (...args: any[]) => any>(
  func: T,
  delay: number
): (...args: Parameters<T>) => void {
  let timeoutId: ReturnType<typeof setTimeout>;

  return (...args: Parameters<T>) => {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(() => func.apply(null, args), delay);
  };
}

export function validatePath(path: string): boolean {
  if (!path || path.trim().length === 0) return false;

  // 基本的なパス検証
  const pathRegex = /^[a-zA-Z0-9\/\-_.\s~]+$/;
  return pathRegex.test(path);
}

export function generateColorFromString(str: string): string {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    hash = str.charCodeAt(i) + ((hash << 5) - hash);
  }

  const hue = hash % 360;
  return `hsl(${hue}, 70%, 50%)`;
}

export function sortByProperty<T>(
  array: T[],
  property: keyof T,
  direction: 'asc' | 'desc' = 'asc'
): T[] {
  return [...array].sort((a, b) => {
    const aVal = a[property];
    const bVal = b[property];

    if (aVal === null || aVal === undefined) return 1;
    if (bVal === null || bVal === undefined) return -1;

    if (typeof aVal === 'string' && typeof bVal === 'string') {
      return direction === 'asc'
        ? aVal.localeCompare(bVal)
        : bVal.localeCompare(aVal);
    }

    if (typeof aVal === 'number' && typeof bVal === 'number') {
      return direction === 'asc' ? aVal - bVal : bVal - aVal;
    }

    return 0;
  });
}

import type { File, FileCategory, FileCategoryInfo } from './types.js';

export function getFileCategory(file: File): FileCategory {
  if (file.is_directory) return "other";

  const mimeType = file.mime_type?.toLowerCase() || "";
  const extension = file.file_type?.toLowerCase() || "";

  const fileCategories: FileCategoryInfo[] = [
    {
      key: "image",
      label: "画像",
      icon: "🖼️",
      mimeTypes: ["image/"],
      extensions: ["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg", "ico", "tiff", "raw"]
    },
    {
      key: "audio",
      label: "音声",
      icon: "🎵",
      mimeTypes: ["audio/"],
      extensions: ["mp3", "wav", "ogg", "flac", "aac", "m4a", "wma", "opus"]
    },
    {
      key: "video",
      label: "動画",
      icon: "🎬",
      mimeTypes: ["video/"],
      extensions: ["mp4", "avi", "mov", "wmv", "flv", "webm", "mkv", "m4v", "3gp"]
    },
    {
      key: "document",
      label: "ドキュメント",
      icon: "📄",
      mimeTypes: ["application/pdf", "application/msword", "application/vnd.", "text/"],
      extensions: ["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "md", "html", "htm", "css", "js", "json", "xml", "csv", "rtf"]
    },
    {
      key: "archive",
      label: "アーカイブ",
      icon: "📦",
      mimeTypes: ["application/zip", "application/x-rar", "application/x-7z", "application/x-tar", "application/gzip"],
      extensions: ["zip", "rar", "7z", "tar", "gz", "bz2", "xz", "lzma"]
    }
  ];

  for (const category of fileCategories) {
    // MIMEタイプでチェック
    for (const mime of category.mimeTypes) {
      if (mimeType.startsWith(mime.toLowerCase())) {
        return category.key;
      }
    }

    // 拡張子でチェック
    if (category.extensions.includes(extension)) {
      return category.key;
    }
  }

  return "other";
}
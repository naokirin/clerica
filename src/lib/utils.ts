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
    return date.toLocaleDateString('ja-JP', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
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
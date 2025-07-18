import type { File, FileWithTags, SortOptions, Tag, FileCategory } from '../../src/lib/types';

// `invoke`のモック
export const invoke = async (cmd: string, args?: any): Promise<any> => {
  console.log(`[Storybook] Tauri invoke mock: ${cmd}`, args);
  
  // サンプルデータを返す
  switch (cmd) {
    case 'get_files':
      return getMockFiles();
    case 'get_files_with_tags':
      return getMockFilesWithTags();
    case 'get_files_by_directory':
      return getMockFiles();
    case 'get_files_by_directory_with_tags':
      return getMockFilesWithTags();
    case 'get_files_paginated':
      return getMockFiles();
    case 'get_files_by_directory_paginated':
      return getMockFiles();
    case 'count_files':
      return 42;
    case 'count_files_by_directory':
      return 15;
    case 'get_directories':
      return getMockDirectories();
    case 'get_tags':
      return getMockTags();
    case 'search_files':
      return getMockFiles();
    case 'count_files_by_category':
      return {
        'image': 10,
        'video': 5,
        'audio': 3,
        'document': 8,
        'archive': 2,
        'other': 14
      };
    case 'get_files_paginated_with_category':
      return getMockFiles();
    case 'get_files_by_directory_paginated_with_category':
      return getMockFiles();
    case 'count_files_with_category':
      return 25;
    case 'count_files_by_directory_with_category':
      return 10;
    case 'open_file':
      return Promise.resolve();
    case 'reveal_in_finder':
      return Promise.resolve();
    case 'delete_file':
      return Promise.resolve();
    case 'delete_files':
      return {
        successful_files: ['file1.jpg', 'file2.png'],
        failed_files: []
      };
    case 'batch_rename_files':
      return {
        successful_files: ['renamed1.jpg', 'renamed2.png'],
        failed_files: []
      };
    case 'generate_video_thumbnail':
      return '/mock/thumbnail.jpg';
    case 'extract_audio_album_art':
      return '/mock/album-art.jpg';
    case 'generate_archive_thumbnail':
      return '/mock/archive-thumb.jpg';
    case 'update_file_tags':
      return Promise.resolve();
    case 'get_file_tags':
      return getMockTags();
    case 'preview_rename':
      return 'new_filename.jpg';
    case 'execute_rename':
      return 'new_filename.jpg';
    case 'preview_advanced_batch_rename':
      return [
        {
          file_id: '1',
          old_name: 'old_file.jpg',
          new_name: 'new_file.jpg'
        }
      ];
    case 'execute_advanced_batch_rename':
      return {
        successful_files: ['new_file.jpg'],
        failed_files: []
      };
    default:
      return Promise.resolve();
  }
};

// `listen`のモック
export const listen = async (event: string, handler: (event: any) => void): Promise<() => void> => {
  console.log(`[Storybook] Tauri listen mock: ${event}`);
  // 何もしないunlisten関数を返す
  return () => {
    console.log(`[Storybook] Tauri unlisten mock: ${event}`);
  };
};

// モックデータ生成関数
function getMockFiles(): File[] {
  return [
    {
      id: '1',
      name: 'sample_photo.jpg',
      path: '/mock/path/sample_photo.jpg',
      size: 1024000,
      modified_at: new Date().toISOString(),
      created_at: new Date().toISOString(),
      directory_id: '1',
      directory_name: 'Photos',
      file_type: 'image/jpeg',
      category: 'image' as FileCategory,
      thumbnail_path: '/mock/thumbnails/sample_photo.jpg'
    },
    {
      id: '2',
      name: 'document.pdf',
      path: '/mock/path/document.pdf',
      size: 512000,
      modified_at: new Date().toISOString(),
      created_at: new Date().toISOString(),
      directory_id: '2',
      directory_name: 'Documents',
      file_type: 'application/pdf',
      category: 'document' as FileCategory,
      thumbnail_path: null
    },
    {
      id: '3',
      name: 'video.mp4',
      path: '/mock/path/video.mp4',
      size: 10240000,
      modified_at: new Date().toISOString(),
      created_at: new Date().toISOString(),
      directory_id: '1',
      directory_name: 'Videos',
      file_type: 'video/mp4',
      category: 'video' as FileCategory,
      thumbnail_path: '/mock/thumbnails/video.jpg'
    }
  ];
}

function getMockFilesWithTags(): FileWithTags[] {
  const files = getMockFiles();
  const tags = getMockTags();
  
  return files.map(file => ({
    ...file,
    tags: tags.slice(0, 2) // 最初の2つのタグを付与
  }));
}

function getMockDirectories() {
  return [
    {
      id: '1',
      name: 'Photos',
      path: '/mock/path/photos',
      created_at: new Date().toISOString(),
      file_count: 15
    },
    {
      id: '2',
      name: 'Documents',
      path: '/mock/path/documents',
      created_at: new Date().toISOString(),
      file_count: 8
    },
    {
      id: '3',
      name: 'Videos',
      path: '/mock/path/videos',
      created_at: new Date().toISOString(),
      file_count: 5
    }
  ];
}

function getMockTags(): Tag[] {
  return [
    {
      id: '1',
      name: 'Important',
      color: '#FF0000',
      created_at: new Date().toISOString()
    },
    {
      id: '2',
      name: 'Work',
      color: '#00FF00',
      created_at: new Date().toISOString()
    },
    {
      id: '3',
      name: 'Personal',
      color: '#0000FF',
      created_at: new Date().toISOString()
    }
  ];
}

export const tauri = {
  invoke,
  listen,
};
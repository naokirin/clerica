<script lang="ts">
  import { X, Trash2, Loader2 } from "lucide-svelte";
  import type { File, CustomMetadataKey } from "../types.js";
  import { formatFileSize, formatDate } from "../utils.js";
  import CustomMetadataEditor from "./CustomMetadataEditor.svelte";
  import * as exifApi from "../api/exif.js";

  interface Props {
    file: File | null;
    isDeleting: boolean;
    customMetadataKeys: CustomMetadataKey[];
    onOpenFile: (filePath: string) => void;
    onRevealInFinder: (filePath: string) => void;
    onDeleteFile: (filePath: string, fileName: string) => void;
    onClose: () => void;
  }

  let {
    file,
    isDeleting,
    customMetadataKeys,
    onOpenFile,
    onRevealInFinder,
    onDeleteFile,
    onClose
  }: Props = $props();

  // EXIFメタデータの値を解釈する関数
  async function interpretExifValue(key: string, value: any): Promise<string> {
    if (value === null || value === undefined) return '不明';
    
    switch (key) {
      case 'Orientation':
        const orientationName = await exifApi.getOrientationName(value);
        return orientationName || `不明 (${value})`;
        
      case 'ColorSpace':
        const colorSpaceName = await exifApi.getColorSpaceName(value);
        return colorSpaceName || `不明 (${value})`;
        
      case 'MeteringMode':
        const meteringModeName = await exifApi.getMeteringModeName(value);
        return meteringModeName || `不明 (${value})`;
        
      case 'LightSource':
        const lightSourceName = await exifApi.getLightSourceName(value);
        return lightSourceName || `不明 (${value})`;
        
      case 'WhiteBalance':
        const whiteBalanceName = await exifApi.getWhiteBalanceName(value);
        return whiteBalanceName || `不明 (${value})`;
        
      case 'SceneCaptureType':
        const sceneCaptureTypeName = await exifApi.getSceneCaptureTypeName(value);
        return sceneCaptureTypeName || `不明 (${value})`;
        
      case 'Contrast':
      case 'Saturation':
      case 'Sharpness':
        const enhancementName = await exifApi.getEnhancementName(value);
        return enhancementName || `不明 (${value})`;
        
      case 'SubjectDistanceRange':
        const subjectDistanceRangeName = await exifApi.getSubjectDistanceRangeName(value);
        return subjectDistanceRangeName || `不明 (${value})`;
        
      case 'Flash':
        const flashModes: { [key: number]: string } = {
          0: 'フラッシュなし',
          1: 'フラッシュあり',
          5: 'フラッシュあり（ストロボリターンなし）',
          7: 'フラッシュあり（ストロボリターンあり）',
          9: 'フラッシュあり（強制発光）',
          13: 'フラッシュあり（強制発光、ストロボリターンなし）',
          15: 'フラッシュあり（強制発光、ストロボリターンあり）',
          16: 'フラッシュなし（強制非発光）',
          24: 'フラッシュなし（自動）',
          25: 'フラッシュあり（自動）',
          29: 'フラッシュあり（自動、ストロボリターンなし）',
          31: 'フラッシュあり（自動、ストロボリターンあり）',
          32: 'フラッシュなし（利用可能だが未使用）',
          65: 'フラッシュあり（赤目軽減）',
          69: 'フラッシュあり（赤目軽減、ストロボリターンなし）',
          71: 'フラッシュあり（赤目軽減、ストロボリターンあり）',
          73: 'フラッシュあり（強制発光、赤目軽減）',
          77: 'フラッシュあり（強制発光、赤目軽減、ストロボリターンなし）',
          79: 'フラッシュあり（強制発光、赤目軽減、ストロボリターンあり）',
          89: 'フラッシュあり（自動、赤目軽減）',
          93: 'フラッシュあり（自動、ストロボリターンなし、赤目軽減）',
          95: 'フラッシュあり（自動、ストロボリターンあり、赤目軽減）'
        };
        return flashModes[value] || `不明 (${value})`;
        
      case 'WhiteBalance':
        const whiteBalanceModes: { [key: number]: string } = {
          0: '自動',
          1: '手動'
        };
        return whiteBalanceModes[value] || `不明 (${value})`;
        
      case 'ColorSpace':
        const colorSpaces: { [key: number]: string } = {
          1: 'sRGB',
          65535: 'キャリブレーションなし'
        };
        return colorSpaces[value] || `不明 (${value})`;
        
      case 'MeteringMode':
        const meteringModes: { [key: number]: string } = {
          0: '不明',
          1: '平均',
          2: '中央重点',
          3: 'スポット',
          4: 'マルチスポット',
          5: 'パターン',
          6: '部分',
          255: 'その他'
        };
        return meteringModes[value] || `不明 (${value})`;
        
      case 'ExposureMode':
        const exposureModes: { [key: number]: string } = {
          0: '自動露出',
          1: 'マニュアル露出',
          2: '自動ブラケット'
        };
        return exposureModes[value] || `不明 (${value})`;
        
      case 'SceneCaptureType':
        const sceneTypes: { [key: number]: string } = {
          0: '標準',
          1: '風景',
          2: 'ポートレート',
          3: '夜景'
        };
        return sceneTypes[value] || `不明 (${value})`;
        
      case 'ExposureProgram':
        const exposurePrograms: { [key: number]: string } = {
          0: '未定義',
          1: 'マニュアル',
          2: 'プログラム自動',
          3: '絞り優先',
          4: 'シャッター優先',
          5: 'クリエイティブプログラム（被写界深度優先）',
          6: 'アクションプログラム（シャッター速度優先）',
          7: 'ポートレートモード',
          8: '風景モード'
        };
        return exposurePrograms[value] || `不明 (${value})`;
        
      case 'FocalLength':
        if (Array.isArray(value) && value.length > 0) {
          return `${value[0]} mm`;
        }
        return `${value} mm`;
        
      case 'ExposureTime':
        if (Array.isArray(value) && value.length > 0) {
          const seconds = value[0];
          if (seconds >= 1) {
            return `${seconds} 秒`;
          } else {
            return `1/${Math.round(1/seconds)} 秒`;
          }
        }
        return `${value}`;
        
      case 'FNumber':
        if (Array.isArray(value) && value.length > 0) {
          return `F/${value[0]}`;
        }
        return `F/${value}`;
        
      case 'ISOSpeedRatings':
        if (Array.isArray(value)) {
          return value.join(', ');
        }
        return `ISO ${value}`;
        
      case 'LightSource':
        const lightSources: { [key: number]: string } = {
          0: '不明',
          1: '昼光',
          2: '蛍光灯',
          3: 'タングステン',
          4: 'フラッシュ',
          9: '晴天',
          10: '曇天',
          11: '日陰',
          12: '昼光蛍光灯（D 5700 – 7100K）',
          13: '昼白色蛍光灯（N 4600 – 5400K）',
          14: '白色蛍光灯（W 3900 – 4500K）',
          15: '温白色蛍光灯（WW 3200 – 3700K）',
          17: '標準光A',
          18: '標準光B',
          19: '標準光C',
          20: 'D55',
          21: 'D65',
          22: 'D75',
          23: 'D50',
          24: 'ISO スタジオタングステン',
          255: 'その他の光源'
        };
        return lightSources[value] || `不明 (${value})`;
        
      case 'ResolutionUnit':
        const resolutionUnits: { [key: number]: string } = {
          1: '単位なし',
          2: 'インチ',
          3: 'センチメートル'
        };
        return resolutionUnits[value] || `不明 (${value})`;
        
      case 'YCbCrPositioning':
        const ycbcrPositions: { [key: number]: string } = {
          1: '中央',
          2: '共設置'
        };
        return ycbcrPositions[value] || `不明 (${value})`;
        
      case 'XResolution':
      case 'YResolution':
        if (Array.isArray(value) && value.length > 0) {
          return `${value[0]} dpi`;
        }
        return `${value} dpi`;
        
      case 'CompressedBitsPerPixel':
        if (Array.isArray(value) && value.length > 0) {
          return `${value[0]} ビット/ピクセル`;
        }
        return `${value} ビット/ピクセル`;
        
      case 'ExifImageWidth':
      case 'ExifImageHeight':
      case 'PixelXDimension':
      case 'PixelYDimension':
        return `${value} ピクセル`;
        
      case 'ComponentsConfiguration':
        if (Array.isArray(value)) {
          const components = value.map((comp: number) => {
            switch (comp) {
              case 0: return '-';
              case 1: return 'Y';
              case 2: return 'Cb';
              case 3: return 'Cr';
              case 4: return 'R';
              case 5: return 'G';
              case 6: return 'B';
              default: return '?';
            }
          });
          return components.join('');
        }
        return String(value);
        
      default:
        if (typeof value === 'object' && value !== null) {
          if (Array.isArray(value)) {
            return value.join(', ');
          } else {
            return JSON.stringify(value);
          }
        }
        return String(value);
    }
  }

  // EXIFタグ番号から名前を取得する関数
  async function getExifTagName(tagNumber: number): Promise<string> {
    const tagName = await exifApi.getTagName(tagNumber);
    return tagName || `タグ ${tagNumber}`;
  }
</script>

{#if file}
  <div class="modal-overlay" onclick={isDeleting ? undefined : onClose}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3>ファイル詳細</h3>
        <div class="modal-actions">
          <button 
            class="action-button open-button" 
            onclick={() => onOpenFile(file.path)}
            title="ファイルを開く"
            disabled={isDeleting}
          >
            📂 開く
          </button>
          <button 
            class="action-button finder-button" 
            onclick={() => onRevealInFinder(file.path)}
            title="Finderで表示"
            disabled={isDeleting}
          >
            🔍 Finder
          </button>
          <button 
            class="action-button delete-button" 
            onclick={() => onDeleteFile(file.path, file.name)}
            title={isDeleting ? "削除中..." : "ゴミ箱に移動"}
            disabled={isDeleting}
          >
            {#if isDeleting}
              <Loader2 size={16} class="animate-spin" />
              削除中...
            {:else}
              <Trash2 size={16} />
              削除
            {/if}
          </button>
          <button 
            class="close-button" 
            onclick={onClose}
            disabled={isDeleting}
          >
            <X size={20} />
          </button>
        </div>
      </div>
      <div class="modal-body">
        <div class="file-detail-section">
          <h4>基本情報</h4>
          <div class="detail-grid">
            <div class="detail-item">
              <span class="detail-label">ファイル名:</span>
              <span class="detail-value">{file.name}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">パス:</span>
              <span class="detail-value">{file.path}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">サイズ:</span>
              <span class="detail-value">{formatFileSize(file.file_size || file.size)}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">種類:</span>
              <span class="detail-value">{file.is_directory ? 'ディレクトリ' : (file.mime_type || file.file_type || 'Unknown')}</span>
            </div>
          </div>
        </div>

        <div class="file-detail-section">
          <h4>日時情報</h4>
          <div class="detail-grid">
            <div class="detail-item">
              <span class="detail-label">作成日時:</span>
              <span class="detail-value">{formatDate(file.created_at)}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">更新日時:</span>
              <span class="detail-value">{formatDate(file.modified_at)}</span>
            </div>
            {#if file.birth_time}
              <div class="detail-item">
                <span class="detail-label">作成日時 (birth_time):</span>
                <span class="detail-value">{formatDate(file.birth_time)}</span>
              </div>
            {/if}
            {#if file.last_accessed}
              <div class="detail-item">
                <span class="detail-label">最終アクセス日時:</span>
                <span class="detail-value">{formatDate(file.last_accessed)}</span>
              </div>
            {/if}
          </div>
        </div>

        <div class="file-detail-section">
          <h4>システム情報</h4>
          <div class="detail-grid">
            {#if file.permissions}
              <div class="detail-item">
                <span class="detail-label">権限:</span>
                <span class="detail-value">{file.permissions}</span>
              </div>
            {/if}
            {#if file.owner_uid !== null}
              <div class="detail-item">
                <span class="detail-label">オーナー UID:</span>
                <span class="detail-value">{file.owner_uid}</span>
              </div>
            {/if}
            {#if file.group_gid !== null}
              <div class="detail-item">
                <span class="detail-label">グループ GID:</span>
                <span class="detail-value">{file.group_gid}</span>
              </div>
            {/if}
            {#if file.inode !== null}
              <div class="detail-item">
                <span class="detail-label">inode:</span>
                <span class="detail-value">{file.inode}</span>
              </div>
            {/if}
            {#if file.hard_links !== null}
              <div class="detail-item">
                <span class="detail-label">ハードリンク数:</span>
                <span class="detail-value">{file.hard_links}</span>
              </div>
            {/if}
            {#if file.device_id !== null}
              <div class="detail-item">
                <span class="detail-label">デバイス ID:</span>
                <span class="detail-value">{file.device_id}</span>
              </div>
            {/if}
          </div>
        </div>

        <!-- ファイルメタデータ (EXIF, 音声情報等) -->
        {#if file.metadata}
          <div class="file-detail-section">
            <h4>ファイルメタデータ</h4>
            <div class="metadata-section">
              {#if (() => {
                try {
                  const parsed = JSON.parse(file.metadata);
                  return typeof parsed === 'object' && parsed !== null;
                } catch {
                  return false;
                }
              })()}
                {#each Object.entries(JSON.parse(file.metadata)) as [category, data]}
                  <div class="metadata-category">
                    <h5>{(() => {
                      switch(category) {
                        case 'exif': return 'EXIF情報';
                        case 'audio': return '音声情報';
                        default: return category.toUpperCase() + '情報';
                      }
                    })()} </h5>
                    
                    {#if category === 'audio'}
                      <!-- 音声ファイル専用の表示 -->
                      <div class="detail-grid">
                        <!-- 基本情報 -->
                        {#if data.duration !== undefined}
                          <div class="detail-item">
                            <span class="detail-label">再生時間:</span>
                            <span class="detail-value">{Math.floor(data.duration / 60)}:{(data.duration % 60).toString().padStart(2, '0')}</span>
                          </div>
                        {/if}
                        {#if data.bitrate !== undefined && data.bitrate > 0}
                          <div class="detail-item">
                            <span class="detail-label">ビットレート:</span>
                            <span class="detail-value">{data.bitrate} kbps</span>
                          </div>
                        {/if}
                        {#if data.sample_rate !== undefined && data.sample_rate > 0}
                          <div class="detail-item">
                            <span class="detail-label">サンプルレート:</span>
                            <span class="detail-value">{data.sample_rate} Hz</span>
                          </div>
                        {/if}
                        {#if data.channels !== undefined && data.channels > 0}
                          <div class="detail-item">
                            <span class="detail-label">チャンネル数:</span>
                            <span class="detail-value">{data.channels === 1 ? 'モノラル' : data.channels === 2 ? 'ステレオ' : data.channels + 'ch'}</span>
                          </div>
                        {/if}
                        
                        <!-- タグ情報 -->
                        {#if data.tags && typeof data.tags === 'object'}
                          {#if data.tags.title}
                            <div class="detail-item">
                              <span class="detail-label">タイトル:</span>
                              <span class="detail-value">{data.tags.title}</span>
                            </div>
                          {/if}
                          {#if data.tags.artist}
                            <div class="detail-item">
                              <span class="detail-label">アーティスト:</span>
                              <span class="detail-value">{data.tags.artist}</span>
                            </div>
                          {/if}
                          {#if data.tags.album}
                            <div class="detail-item">
                              <span class="detail-label">アルバム:</span>
                              <span class="detail-value">{data.tags.album}</span>
                            </div>
                          {/if}
                          {#if data.tags.year}
                            <div class="detail-item">
                              <span class="detail-label">年:</span>
                              <span class="detail-value">{data.tags.year}</span>
                            </div>
                          {/if}
                          {#if data.tags.genre}
                            <div class="detail-item">
                              <span class="detail-label">ジャンル:</span>
                              <span class="detail-value">{data.tags.genre}</span>
                            </div>
                          {/if}
                          {#if data.tags.track}
                            <div class="detail-item">
                              <span class="detail-label">トラック番号:</span>
                              <span class="detail-value">{data.tags.track}</span>
                            </div>
                          {/if}
                        {/if}
                      </div>
                    {:else}
                      <!-- その他のメタデータ（EXIF等）の表示 -->
                      <div class="detail-grid">
                        {#each Object.entries(data) as [key, value]}
                          {#snippet metadataItem()}
                            {#await Promise.all([ 
                              (async () => {
                                // Tag(Exif, 数値)形式のキーを処理
                                if (key.startsWith('Tag(') && key.includes(',')) {
                                  const match = key.match(/Tag\((\w+),\s*(\d+)\)/);
                                  if (match) {
                                    const tagNumber = parseInt(match[2]);
                                    return await getExifTagName(tagNumber);
                                  }
                                }
                                
                                // キー名を日本語化
                                switch(key) {
                                  case 'Make': return 'メーカー';
                                  case 'Model': return 'モデル';
                                  case 'DateTime': return '撮影日時';
                                  case 'ExposureTime': return '露出時間';
                                  case 'FNumber': return 'F値';
                                  case 'ISOSpeedRatings': return 'ISO感度';
                                  case 'FocalLength': return '焦点距離';
                                  case 'Flash': return 'フラッシュ';
                                  case 'WhiteBalance': return 'ホワイトバランス';
                                  case 'ColorSpace': return 'カラースペース';
                                  case 'ExifImageWidth': return '画像幅';
                                  case 'ExifImageHeight': return '画像高';
                                  case 'Orientation': return '向き';
                                  case 'MeteringMode': return '測光モード';
                                  case 'ExposureMode': return '露出モード';
                                  case 'ExposureProgram': return '露出プログラム';
                                  case 'SceneCaptureType': return 'シーンタイプ';
                                  case 'LightSource': return '光源';
                                  case 'FlashPixVersion': return 'FlashPixバージョン';
                                  case 'ExifVersion': return 'Exifバージョン';
                                  case 'ComponentsConfiguration': return 'コンポーネント構成';
                                  case 'CompressedBitsPerPixel': return '圧縮ビット/ピクセル';
                                  case 'PixelXDimension': return 'ピクセルX次元';
                                  case 'PixelYDimension': return 'ピクセルY次元';
                                  case 'UserComment': return 'ユーザーコメント';
                                  case 'RelatedSoundFile': return '関連音声ファイル';
                                  case 'DateTimeOriginal': return '撮影日時（オリジナル）';
                                  case 'DateTimeDigitized': return 'デジタル化日時';
                                  case 'SubSecTime': return 'サブ秒時間';
                                  case 'SubSecTimeOriginal': return 'サブ秒時間（オリジナル）';
                                  case 'SubSecTimeDigitized': return 'サブ秒時間（デジタル化）';
                                  case 'ImageDescription': return '画像説明';
                                  case 'Software': return 'ソフトウェア';
                                  case 'Artist': return 'アーティスト';
                                  case 'Copyright': return '著作権';
                                  case 'XResolution': return 'X解像度';
                                  case 'YResolution': return 'Y解像度';
                                  case 'ResolutionUnit': return '解像度単位';
                                  case 'ImageWidth': return '画像幅';
                                  case 'ImageLength': return '画像高';
                                  case 'BitsPerSample': return 'ビット/サンプル';
                                  case 'Compression': return '圧縮';
                                  case 'PhotometricInterpretation': return '測光解釈';
                                  case 'SamplesPerPixel': return 'サンプル/ピクセル';
                                  case 'PlanarConfiguration': return 'プレーナー構成';
                                  case 'TransferFunction': return '転送関数';
                                  case 'WhitePoint': return 'ホワイトポイント';
                                  case 'PrimaryChromaticities': return '原色度';
                                  case 'YCbCrCoefficients': return 'YCbCr係数';
                                  case 'YCbCrSubSampling': return 'YCbCrサブサンプリング';
                                  case 'YCbCrPositioning': return 'YCbCr位置';
                                  case 'ReferenceBlackWhite': return '基準黒白';
                                  default: return key;
                                }
                              })(),
                              interpretExifValue(key, value)
                            ])}
                              <div class="detail-item">
                                <span class="detail-label">読み込み中...</span>
                                <span class="detail-value">読み込み中...</span>
                              </div>
                            {:then [labelName, interpretedValue]}
                              <div class="detail-item">
                                <span class="detail-label">{labelName}:</span>
                                <span class="detail-value">{interpretedValue}</span>
                              </div>
                            {:catch error}
                              <div class="detail-item">
                                <span class="detail-label">{key}:</span>
                                <span class="detail-value">エラー: {error.message}</span>
                              </div>
                            {/await}
                          {/snippet}
                          {@render metadataItem()}
                        {/each}
                      </div>
                    {/if}
                  </div>
                {/each}
              {:else}
                <div class="error-message">
                  メタデータの解析に失敗しました
                </div>
              {/if}
            </div>
          </div>
        {/if}

        <!-- カスタムメタデータ -->
        {#if customMetadataKeys.length > 0}
          <CustomMetadataEditor 
            fileId={file.id}
            {customMetadataKeys}
          />
        {/if}
      </div>
    </div>
  </div>
{/if}
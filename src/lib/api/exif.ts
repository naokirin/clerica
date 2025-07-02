import { invoke } from '@tauri-apps/api/core';

export interface ExifConfig {
  exif_tags: Record<number, string>;
  orientation_values: Record<number, string>;
  color_space_values: Record<number, string>;
  metering_mode_values: Record<number, string>;
  light_source_values: Record<number, string>;
  white_balance_values: Record<number, string>;
  scene_capture_type_values: Record<number, string>;
  enhancement_values: Record<number, string>;
  subject_distance_range_values: Record<number, string>;
}

let exifConfig: ExifConfig | null = null;

export async function getExifConfig(): Promise<ExifConfig> {
  if (!exifConfig) {
    exifConfig = await invoke<ExifConfig>('get_exif_config_data');
  }
  return exifConfig;
}

export async function getTagName(tagId: number): Promise<string | null> {
  const config = await getExifConfig();
  return config.exif_tags[tagId] || null;
}

export async function getOrientationName(value: number): Promise<string | null> {
  const config = await getExifConfig();
  return config.orientation_values[value] || null;
}

export async function getColorSpaceName(value: number): Promise<string | null> {
  const config = await getExifConfig();
  return config.color_space_values[value] || null;
}

export async function getMeteringModeName(value: number): Promise<string | null> {
  const config = await getExifConfig();
  return config.metering_mode_values[value] || null;
}

export async function getLightSourceName(value: number): Promise<string | null> {
  const config = await getExifConfig();
  return config.light_source_values[value] || null;
}

export async function getWhiteBalanceName(value: number): Promise<string | null> {
  const config = await getExifConfig();
  return config.white_balance_values[value] || null;
}

export async function getSceneCaptureTypeName(value: number): Promise<string | null> {
  const config = await getExifConfig();
  return config.scene_capture_type_values[value] || null;
}

export async function getEnhancementName(value: number): Promise<string | null> {
  const config = await getExifConfig();
  return config.enhancement_values[value] || null;
}

export async function getSubjectDistanceRangeName(value: number): Promise<string | null> {
  const config = await getExifConfig();
  return config.subject_distance_range_values[value] || null;
}
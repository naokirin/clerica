use std::collections::HashMap;

/// シーンキャプチャタイプの値マッピング
pub fn get_scene_capture_type_values() -> HashMap<u8, &'static str> {
    let mut map = HashMap::new();
    
    map.insert(0, "標準");
    map.insert(1, "風景");
    map.insert(2, "ポートレート");
    map.insert(3, "夜景");
    
    map
}
use std::collections::HashMap;

/// 被写体距離範囲の値マッピング
pub fn get_subject_distance_range_values() -> HashMap<u8, &'static str> {
    let mut map = HashMap::new();
    
    map.insert(0, "不明");
    map.insert(1, "マクロ");
    map.insert(2, "近景");
    map.insert(3, "遠景");
    
    map
}
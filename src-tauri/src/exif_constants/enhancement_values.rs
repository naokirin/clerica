use std::collections::HashMap;

/// コントラスト/彩度/シャープネスの値マッピング
pub fn get_enhancement_values() -> HashMap<u8, &'static str> {
    let mut map = HashMap::new();
    
    map.insert(0, "標準");
    map.insert(1, "ソフト");
    map.insert(2, "ハード");
    
    map
}
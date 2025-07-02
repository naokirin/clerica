use std::collections::HashMap;

/// 色空間の値マッピング
pub fn get_color_space_values() -> HashMap<u16, &'static str> {
    let mut map = HashMap::new();
    
    map.insert(1, "sRGB");
    map.insert(2, "Adobe RGB");
    map.insert(65535, "未較正");
    
    map
}
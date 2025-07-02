use std::collections::HashMap;

/// ホワイトバランスの値マッピング
pub fn get_white_balance_values() -> HashMap<u8, &'static str> {
    let mut map = HashMap::new();
    
    map.insert(0, "オート");
    map.insert(1, "マニュアル");
    
    map
}
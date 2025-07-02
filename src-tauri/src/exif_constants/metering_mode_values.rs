use std::collections::HashMap;

/// 測光方式の値マッピング
pub fn get_metering_mode_values() -> HashMap<u8, &'static str> {
    let mut map = HashMap::new();
    
    map.insert(0, "不明");
    map.insert(1, "平均");
    map.insert(2, "中央重点");
    map.insert(3, "スポット");
    map.insert(4, "マルチスポット");
    map.insert(5, "パターン");
    map.insert(6, "部分");
    map.insert(255, "その他");
    
    map
}
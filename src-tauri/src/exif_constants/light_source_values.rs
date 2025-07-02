use std::collections::HashMap;

/// 光源の値マッピング
pub fn get_light_source_values() -> HashMap<u8, &'static str> {
    let mut map = HashMap::new();
    
    map.insert(0, "不明");
    map.insert(1, "昼光");
    map.insert(2, "蛍光灯");
    map.insert(3, "タングステン");
    map.insert(4, "フラッシュ");
    map.insert(9, "晴天");
    map.insert(10, "曇天");
    map.insert(11, "日陰");
    map.insert(12, "昼光色蛍光灯");
    map.insert(13, "昼白色蛍光灯");
    map.insert(14, "白色蛍光灯");
    map.insert(15, "温白色蛍光灯");
    map.insert(17, "標準光A");
    map.insert(18, "標準光B");
    map.insert(19, "標準光C");
    map.insert(20, "D55");
    map.insert(21, "D65");
    map.insert(22, "D75");
    map.insert(23, "D50");
    map.insert(24, "ISO スタジオタングステン");
    map.insert(255, "その他");
    
    map
}
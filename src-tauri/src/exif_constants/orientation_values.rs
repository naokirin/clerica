use std::collections::HashMap;

/// 画像の向きの値マッピング
pub fn get_orientation_values() -> HashMap<u8, &'static str> {
    let mut map = HashMap::new();
    
    map.insert(1, "正常");
    map.insert(2, "水平反転");
    map.insert(3, "180度回転");
    map.insert(4, "垂直反転");
    map.insert(5, "90度反時計回りに回転後水平反転");
    map.insert(6, "90度時計回りに回転");
    map.insert(7, "90度時計回りに回転後水平反転");
    map.insert(8, "90度反時計回りに回転");
    
    map
}
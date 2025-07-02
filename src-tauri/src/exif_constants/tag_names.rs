use std::collections::HashMap;

/// EXIFタグIDから日本語名へのマッピング
pub fn get_exif_tag_names() -> HashMap<u32, &'static str> {
    let mut map = HashMap::new();
    
    // 基本的なTIFFタグ
    map.insert(271, "メーカー");
    map.insert(272, "モデル");
    map.insert(274, "画像の向き");
    map.insert(282, "X解像度");
    map.insert(283, "Y解像度");
    map.insert(296, "解像度単位");
    map.insert(306, "撮影日時");
    map.insert(315, "作成者");
    
    // EXIFタグ
    map.insert(33434, "露出時間");
    map.insert(33437, "F値");
    map.insert(34850, "露出プログラム");
    map.insert(34855, "ISO感度");
    map.insert(36864, "Exifバージョン");
    map.insert(36867, "撮影日時");
    map.insert(36868, "デジタル化日時");
    map.insert(37121, "色空間の構成要素");
    map.insert(37377, "シャッタースピード値");
    map.insert(37378, "絞り値");
    map.insert(37379, "明度値");
    map.insert(37380, "露出補正値");
    map.insert(37381, "最大絞り値");
    map.insert(37382, "被写体距離");
    map.insert(37383, "測光方式");
    map.insert(37384, "光源");
    map.insert(37385, "フラッシュ");
    map.insert(37386, "焦点距離");
    map.insert(37396, "被写体エリア");
    map.insert(40960, "FlashPixバージョン");
    map.insert(40961, "色空間");
    map.insert(40962, "ピクセルX寸法");
    map.insert(40963, "ピクセルY寸法");
    map.insert(41486, "ホワイトバランス");
    map.insert(41487, "デジタルズーム倍率");
    map.insert(41488, "35mm換算焦点距離");
    map.insert(41489, "シーンキャプチャタイプ");
    map.insert(41490, "ゲインコントロール");
    map.insert(41491, "コントラスト");
    map.insert(41492, "彩度");
    map.insert(41493, "シャープネス");
    map.insert(41495, "被写体距離範囲");
    map.insert(42016, "画像ユニークID");
    
    map
}
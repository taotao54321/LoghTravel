use crate::geometry::Vec3;

pub const PLANET_COUNT: usize = 32;

pub const fn planet_name(id: usize) -> &'static str {
    const TABLE: [&str; PLANET_COUNT] = [
        "ミニュアス",
        "キュクレウス",
        "ヒュプノイア",
        "ニュクテーナ",
        "コリューバスト",
        "マル・ペッサ",
        "ドリュアント",
        "ペルセポネ",
        "ラグプール",
        "ライガール",
        "トリプラ",
        "ダゴン",
        "ドーリア",
        "バンフリート",
        "マル・アデッタ",
        "レグニツァ",
        "バーラト",
        "アスターテ",
        "メルカルト",
        "ネプティス",
        "カッファー",
        "バルメレンド",
        "シャンプール",
        "ポレビト",
        "ランテマリオ",
        "エリューセラ",
        "リオベルデ",
        "ガンダルバ",
        "エル・ファシル",
        "バーミリオン",
        "リューカス",
        "ハイネセン",
    ];

    TABLE[id]
}

pub const fn planet_position(id: usize) -> Vec3 {
    const TABLE: [Vec3; PLANET_COUNT] = [
        Vec3::new(8, 8, 8),
        Vec3::new(8, 16, 24),
        Vec3::new(16, 8, 16),
        Vec3::new(8, 32, 32),
        Vec3::new(24, 24, 32),
        Vec3::new(32, 8, 40),
        Vec3::new(16, 56, 48),
        Vec3::new(40, 40, 40),
        Vec3::new(64, 16, 24),
        Vec3::new(32, 64, 80),
        Vec3::new(56, 56, 32),
        Vec3::new(80, 24, 48),
        Vec3::new(8, 120, 56),
        Vec3::new(8, 104, 56),
        Vec3::new(120, 16, 8),
        Vec3::new(120, 8, 72),
        Vec3::new(32, 104, 24),
        Vec3::new(48, 88, 112),
        Vec3::new(88, 48, 64),
        Vec3::new(104, 32, 88),
        Vec3::new(48, 112, 88),
        Vec3::new(80, 80, 72),
        Vec3::new(112, 48, 56),
        Vec3::new(72, 112, 80),
        Vec3::new(96, 96, 24),
        Vec3::new(112, 72, 96),
        Vec3::new(96, 120, 104),
        Vec3::new(120, 96, 32),
        Vec3::new(112, 120, 48),
        Vec3::new(112, 112, 88),
        Vec3::new(120, 112, 104),
        Vec3::new(120, 120, 120),
    ];

    TABLE[id]
}

/// 2 つの惑星間の距離を返す。
pub fn planet_distance(id1: usize, id2: usize) -> u32 {
    let pos1 = planet_position(id1);
    let pos2 = planet_position(id2);

    pos1.distance(pos2)
}

pub const fn planet_neighbors(id: usize) -> &'static [usize] {
    const TABLE: [&[usize]; PLANET_COUNT] = [
        &[1, 2],
        &[0, 2, 3],
        &[0, 1, 4, 16],
        &[1, 4, 12],
        &[2, 3, 6, 10],
        &[6, 7, 10, 11],
        &[4, 5, 9, 12],
        &[5, 9, 20, 21],
        &[10, 16, 22, 24],
        &[6, 7, 13],
        &[4, 5, 8],
        &[5, 18, 21],
        &[3, 6, 13],
        &[9, 12],
        &[27, 28],
        &[22, 27, 29],
        &[2, 8, 24],
        &[20, 26],
        &[11, 19, 22, 29],
        &[18, 21, 25, 30],
        &[7, 17, 23],
        &[7, 11, 19, 23],
        &[8, 15, 18, 28],
        &[20, 21, 25],
        &[8, 16, 28],
        &[19, 23, 26],
        &[17, 25, 31],
        &[14, 15],
        &[14, 22, 24],
        &[15, 18, 30],
        &[19, 29, 31],
        &[26, 30],
    ];

    TABLE[id]
}

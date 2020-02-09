fn main() {
    let n1 = 200u8;
    let n2 = 3u8;

    // n1 x n2 = 600の計算
    // std::u8::MAXは255なので桁あふれ

    // 検査付き乗算の結果
    assert_eq!(n1.checked_mul(n2), None);

    // 飽和乗算の結果
    assert_eq!(n1.saturating_mul(n2), std::u8::MAX);

    // ラッピング乗算: 600を256で割ったあまり
    assert_eq!(n1.wrapping_mul(n2), 88);

    // 桁あふれ乗算: 88と桁あふれを示すtrue
    assert_eq!(n1.overflowing_mul(n2), (88, true));
}

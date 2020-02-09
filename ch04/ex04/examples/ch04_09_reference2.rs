fn main() {
    let c1 = 'A'; // char
    let c1_ptr = &c1; // &char イミュータブル参照
    assert_eq!(*c1_ptr, 'A');

    let mut n1 = 0; // i32
    let n1_ptr = &mut n1; // &mut i32 ミュータブル参照
    assert_eq!(*n1_ptr, 0);

    *n1_ptr = 1_000; // mutなので値の変更化
    assert_eq!(*n1_ptr, 1_000);
}

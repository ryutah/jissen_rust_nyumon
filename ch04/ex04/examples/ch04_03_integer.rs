fn main() {
    let n1 = 10_000; // i32
    let n2 = 0u8; // u8
    let n3 = 100_isize; // isize
    let n4 = 10; // 型推論によりisizeになる
    let n5 = n3 + n4; // isize

    print_typename("n1", n1);
    print_typename("n2", n2);
    print_typename("n3", n3);
    print_typename("n4", n4);
    print_typename("n5", n5);

    let h1 = 0xff; // 16進数 i32
    let o1 = 0o744; // 8進数 i32
    let b1 = 0b1010_0110_1110_1001; // 2進数 i32
    println!("h1: {}", h1);
    println!("o1: {}", o1);
    println!("b1: {}", b1);

    let n6 = b'A'; // ASCII文字'A'の文字コード
    println!("n6: {}", n6);
    print_typename("n6", n6);
    assert_eq!(n6, 65u8);
}

fn print_typename<T>(val_name: &str, _: T) {
    println!("{}: {}", val_name, std::any::type_name::<T>());
}

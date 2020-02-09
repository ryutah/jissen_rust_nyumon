// オーバーフロー
//
//   デバッグモードではpanicになる
//   $ cargo run --example ch04_04_overflowed1
//        Running `/Users/ryutah/projects/github.com/ryutah/jissen_rust_nyumon/ch04/ex04/target/debug/examples/ch04_04_overflowed1`
//     thread 'main' panicked at 'attempt to add with overflow', examples/ch04_04_overflowed1.rs:4:14
//     note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
//
//   リリースモードでは誤った値が表示される
//   $ cargo run --release --example ch04_04_overflowed1
//        Running `/Users/ryutah/projects/github.com/ryutah/jissen_rust_nyumon/ch04/ex04/target/release/examples/ch04_04_overflowed1`
//     0
fn main() {
    let n1 = std::u8::MAX; // 255
    let n2 = 1u8;

    let n3 = n1 + n2;
    println!("{}", n3);
}

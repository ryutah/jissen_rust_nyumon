fn main() {
    let mut n = 0;
    println!("main:    n = {}", n);

    f1(n);
    println!("main:    n = {}", n);

    f2(&mut n);
    println!("main:    n = {}", n);
}

/// 呼ぼ出し元の値のコピーを引数nに束縛し、1に変更する
fn f1(mut n: u32) {
    n = 1;
    println!("f1:      n = {}", n);
}

/// 呼び出し元の値を指すポインタを受け取りポインタが指す場所に2を格納する
fn f2(n_ptr: &mut u32) {
    println!("f2:  n_ptr = {:p}", n_ptr);
    *n_ptr = 2; // デリファレンス
    println!("f2: *n_ptr = {}", *n_ptr);
}

pub fn main() {
    smart_pointer();
}

fn smart_pointer() {
    // スマートポインタ: データの所有をするポインタ
    // box: サイズが未確定でもコンパイル→ヒープに確保
    let x = Box::new(10);
    println!("x_box: {:p}", x);
    println!("*x + 2 = {}", *x + 2);
}

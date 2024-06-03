pub fn main() {
    smart_pointer();
}

fn smart_pointer() {
    // スマートポインタ: データの所有をするポインタ
    // box
    let x = Box::new(10);
    println!("x_box: {:p}", x);
}

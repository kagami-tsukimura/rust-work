use std::rc::Rc;

pub fn main() {
    smart_pointer();
}

fn smart_pointer() {
    // スマートポインタ: データの所有をするポインタ
    // box: サイズが未確定でもコンパイル→ヒープに確保
    let x = Box::new(10);
    println!("x_box: {:p}", x);
    println!("*x + 2 = {}", *x + 2);

    // rc: 値の参照数をカウント(references count)
    let y = Rc::new("hello".to_string());
    println!("y_rc count1: {}", Rc::strong_count(&y));
    {
        let z = Rc::clone(&y);
        println!("y: {:p}", y);
        println!("z: {:p}", z);
        println!("y_ptr == z_ptr: {}", y.as_ptr() == z.as_ptr());
        println!("y_rc count2: {}", Rc::strong_count(&y));
        println!("z_rc count: {}", Rc::strong_count(&z));
    }
    println!("y_rc count2: {}", Rc::strong_count(&y));
}

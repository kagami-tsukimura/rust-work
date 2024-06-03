pub fn main() {
    references();
}

fn references() {
    let x = 1;
    // 共有参照: 読み出し可、変更不可
    let y = &x;
    let z = &x;
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);

    println!("-----");

    // 可変参照: 読み出し可、変更可
    let mut a = 1;
    let b = &mut a;
    *b += 1;
    // println!("{:?}", a);
    println!("{:?}", b);
}

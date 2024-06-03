pub fn main() {
    references();
}

fn references() {
    let x = 1;
    // 共有参照
    let y = &x;
    let z = &x;
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}

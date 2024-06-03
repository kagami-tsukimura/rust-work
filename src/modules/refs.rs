pub fn main() {
    references();
}

fn references() {
    let x = 1;
    // 共有参照
    let y = &x;
    println!("{:?}", x);
    println!("{:?}", y);
}

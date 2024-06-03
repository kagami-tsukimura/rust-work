pub fn main() {
    references();
}

fn references() {
    let x = 1;
    let y = &x;
    println!("{:?}", x);
    println!("{:?}", y);
}

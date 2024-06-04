pub fn main() {
    options();
}

fn options() {
    let x = Some(5);
    let y: Option<i32> = None;
    println!("{:?}", x);
    println!("{:?}", y);
}

pub fn main() {
    options();
}

fn options() {
    let a = Some(5);
    let b = Some("str");
    let y: Option<i32> = None;
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", y);
}

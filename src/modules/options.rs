pub fn main() {
    options();
}

fn options() {
    let a = Some(5);
    let b = Some("str");
    let c: Option<i32> = None;
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}

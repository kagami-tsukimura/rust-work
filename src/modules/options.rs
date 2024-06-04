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

    println!("-----");

    let v: Vec<i32> = vec![1, 2, 3];
    let val: Option<&i32> = v.get(2);

    match val {
        Some(x) => println!("value exists: {}", x),
        None => println!("None"),
    }
}

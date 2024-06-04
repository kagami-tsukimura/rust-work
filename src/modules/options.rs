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
    let val2: Option<&i32> = v.get(4);

    print_match(val);
    print_match(val2);

    println!("-----");

    print_if(val);
    print_if(val2);
}

fn print_match(x: Option<&i32>) {
    match x {
        Some(x) => println!("value exists: {}", x),
        None => println!("None"),
    }
}

fn print_if(x: Option<&i32>) {
    if let Some(x) = x {
        println!("value exists: {}", x);
    }
}

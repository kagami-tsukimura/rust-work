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
    let val3: Option<&i32> = v.get(0);

    print_match(val);
    print_match(val2);
    print_match(val3);

    println!("-----");

    print_if(val);
    print_if(val2);
    print_if(val3);
}

fn print_match(x: Option<&i32>) {
    match x {
        // Some(1) => println!("value is 1"),
        // Some(2 | 3) => println!("value is 2 or 3"),
        // *x: 参照外し→実体にアクセス
        Some(x) if *x == 1 => println!("value exists: {}", x),
        Some(x) => println!("value exists: {}", x),
        None => println!("None"),
    }
}

fn print_if(x: Option<&i32>) {
    if let Some(1) = x {
        println!("value is 1");
    } else if let Some(x) = x {
        println!("value exists: {}", x);
    }
}

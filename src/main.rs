fn main() {
    print_msg();

    // 書き換えng
    let a: i32 = 1;
    println!("{}", a);
    // a = 2;

    // mut: 書き換えok
    let mut b: i32 = 1;
    println!("before: {}", b);
    b = 2;
    println!("after: {}", b);

    let c: i32;
    c = 1;
    println!("{}", c);

    // 変数のシャドーイング（型書き換え）: rustではok
    let c: &str = "hello";
    println!("{}", c);
}

fn print_msg() {
    println!("Hello, world!");
    println!("Hello, {}!", "world");
    /*
    println!("Hello1, {}!", "world");
    println!("Hello2, {}!", "world");
    println!("Hello3, {}!", "world");
     */
    // 改行なし
    print!("Hello, ");
    print!("Rust");
}

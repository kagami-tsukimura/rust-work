fn main() {
    print_msg();
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

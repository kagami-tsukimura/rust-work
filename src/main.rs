// 定数: グローバルに記述可能
const B: i32 = 10;

fn main() {
    print_msg();
    print_variable();
    print_constant();
    println!("-----");
}

fn print_msg() {
    println!("-----");
    println!("Hello, world!");
    println!("Hello, {}!", "world");
    /*
    println!("Hello1, {}!", "world");
    println!("Hello2, {}!", "world");
    println!("Hello3, {}!", "world");
     */
    // 改行なし
    print!("Hello, ");
    println!("Rust");
}

fn print_variable() {
    println!("-----");
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

fn print_constant() {
    println!("-----");
    const A: i32 = 1;
    println!("Local: {}", A);
    println!("Global: {}", B);
}

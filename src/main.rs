// 定数: グローバルに記述可能
const B: i32 = 10;

fn main() {
    print_msg();
    print_variable();
    print_constant();
    print_type();
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

fn print_type() {
    println!("-----");
    // 数値型
    let a = 1;
    let b = 2.0;

    let c: u16 = 3;
    let d = 4.0f32;

    let e = 1 + 2;
    // let e = 1 - 2;
    // let e = 1 * 2;
    // let e = 1 / 2;
    // let e = 1 % 2;

    let f = 1 as f64 + 2.0;

    println!("a: {} b: {} c: {} d: {} e: {} f: {}", a, b, c, d, e, f);

    // 論理型
    let g = true;
    let h = false;
    println!("g: {} h: {}", g, h);
    let i = !g;
    println!("i(!g): {}", i);
    let j = g && h;
    println!("j(g && h): {}", j);
    let k = g || h;
    println!("k(g || h): {}", k);
    let x = 1 < 2;
    println!("x(1 < 2): {}", x);
}

// 定数: グローバルに記述可能
const B: i32 = 10;

fn main() {
    print_msg();
    print_variable();
    print_constant();
    print_type();
    print_array();
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

    println!("-----");
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

    println!("-----");
    // タプル型
    let t1 = (1, true, 2.0);
    let t2 = (2.0, 1, true);
    // {:?}: デバッグフォーマット
    println!("{:?}", t1);
    println!("{:?}", t2);

    let i = t1.0;
    println!("i: {}", i);

    let (x, _, z) = t2;
    println!("x: {} z: {}", x, z);

    // ユニット型
    let u: () = ();
    println!("{:?}", u);
}

fn print_array() {
    println!("-----");
    // 配列: 要素数固定（要素数を可変にする場合はベクタ型）
    let a = [1, 2, 3];
    println!("{:?}", a);

    let b = [0; 1000];
    // println!("{:?}", b);

    let c = &b[990..];
    println!("{:?}", c);

    println!("-----");
    // ベクタ型: 要素数可変
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(3);
    println!("before_v: {:?}", v);
    let x = v.pop();
    println!("pop_v: {:?}", x);
    println!("after_v: {:?}", v);

    let v2 = vec![4, 5, 6];
    println!("{:?}", v2);

    let y = v[1];
    println!("y: {}", y);
    let z = v.get(0);
    println!("z: {:?}", z);
    let s = &v[0..2];
    println!("s: {:?}", s);
}

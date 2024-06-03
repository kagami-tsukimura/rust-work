pub fn main() {
    references();
}

fn references() {
    let x = 1;
    // 共有参照: 読み出し可、変更不可
    let y = &x;
    let z = &x;
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);

    println!("-----");

    // 可変参照: 読み出し可、変更可
    let mut a = 1;
    let b = &mut a;
    *b += 1;
    // println!("{:?}", a);
    println!("{:?}", b);

    println!("-----");

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    // 共有参照
    let s = concat(&s1, &s2);
    println!("{}", s);
    println!("{}", s1);
    println!("{}", s2);
}

fn concat(a: &String, b: &String) -> String {
    let c: String = format!("{}, {}", a, b);
    c
}

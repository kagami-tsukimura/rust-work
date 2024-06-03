pub fn main() {
    // stacks();

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let (s, s1, s2) = concat(s1, s2);
    println!("s: {}", s);
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    // 所有権エラー: 関数の引数に所有権が渡されているため
    // let s = concat(s1, s2);
    // println!("s1_err: {}", s1);
    // println!("s2_err: {}", s2);
}

fn stacks() {
    let mut v1 = vec![1, 2, 3];
    println!("v1 ptr: {:?}", v1.as_ptr());
    println!("v1[0] : {:p}", &v1[0]);

    println!("v1 len: {}", v1.len());
    println!("v1 cap: {}", v1.capacity());

    v1.push(4);
    println!("v1 ptr: {:?}", v1.as_ptr());
    println!("v1 len: {}", v1.len());
    println!("v1 cap: {}", v1.capacity());

    println!("-----");
    let v2 = v1;
    // // 所有権エラー: v1 → v2に所有権が移っているため
    // println!("v1 ptr: {:?}", v1.as_ptr());
    println!("v2 ptr: {:?}", v2.as_ptr());
    println!("v2 len: {}", v2.len());
    println!("v2 cap: {}", v2.capacity());

    let v3 = v2.clone();
    println!("v2 ptr: {:?}", v2.as_ptr());
    println!("v3 ptr: {:?}", v3.as_ptr());
    println!("v2_ptr == v3_ptr? : {}", v2.as_ptr() == v3.as_ptr());
    println!("v3 len: {}", v3.len());
    println!("v3 cap: {}", v3.capacity());
}

fn concat(a: String, b: String) -> (String, String, String) {
    let c: String = format!("{}, {}", a, b);
    (c, a, b)
}

// fn concat(a: String, b: String) -> String {
//     let c: String = format!("{}, {}", a, b);
//     c
// }

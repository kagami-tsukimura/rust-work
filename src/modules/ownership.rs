pub fn main() {
    stacks();
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
}

pub fn main() {
    stacks();
}

fn stacks() {
    let mut v1 = vec![1, 2, 3];
    println!("v1 ptr: {:?}", v1.as_ptr());
    println!("v1[0]: {:p}", &v1[0]);

    println!("v1 len: {}", v1.len());
    println!("v1 cap: {}", v1.capacity());

    v1.push(4);
    println!("v1 len: {}", v1.len());
    println!("v1 cap: {}", v1.capacity());
}

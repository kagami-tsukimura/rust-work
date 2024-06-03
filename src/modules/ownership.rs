pub fn main() {
    stacks();
}

fn stacks() {
    let mut v1 = vec![1, 2, 3];
    println!("v1 ptr: {:?}", v1.as_ptr());
    println!("v1[0]: {:p}", &v1[0]);
}

pub fn main() {
    derive();
}

fn derive() {
    // #[属性名]
    println!("{:?}", (1, 2, 3));

    // debug trait
    #[derive(Debug)]
    struct S {
        val1: i32,
        val2: i32,
    }
    println!("{:?}", S { val1: 1, val2: 2 });
}

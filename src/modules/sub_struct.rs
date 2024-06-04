pub fn main() {
    sub_struct();
}

fn sub_struct() {
    pub struct Test {
        pub val1: i32,
        pub val2: i32,
    }

    let test = Test { val1: 1, val2: 2 };
    println!("{:?}", test.val1);
    println!("{:?}", test.val2);
}

pub fn main() {
    genericses();
}

fn genericses() {
    println!("{}", max(1, 2));
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

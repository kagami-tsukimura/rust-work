pub fn main() {
    genericses();
}

fn genericses() {
    println!("{}", max(1, 2));
}

fn max<T>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

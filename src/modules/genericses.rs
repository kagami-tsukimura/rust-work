pub fn main() {
    genericses();
}

fn genericses() {
    println!("{}", max(1, 2));
    println!("{}", max(1.0, 2.3));
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

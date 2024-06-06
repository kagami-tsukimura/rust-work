pub fn main() {
    closures();
}

fn closures() {
    let c1 = |x: i32| x + 1;
    println!("{}", c1(10));
}

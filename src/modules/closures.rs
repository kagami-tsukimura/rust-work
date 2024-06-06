pub fn main() {
    closures();
}

fn closures() {
    let c1 = |x: i32| x + 1;
    println!("{}", c1(10));

    let m = 10;
    let c2 = |x: i32| x + m;
    println!("{}", c2(10));

    let m = 20;
    println!("{}", c2(10));
}

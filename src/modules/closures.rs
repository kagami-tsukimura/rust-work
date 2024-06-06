pub fn main() {
    closures();
}

fn closures() {
    let add = |x, y| x + y;
    println!("{}", add(1, 2));
}

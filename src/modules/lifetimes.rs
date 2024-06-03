pub fn main() {
    lifetimes();
}

fn lifetimes() {
    let s = String::from("Hello");
    let s2 = String::from("Rust");
    println!("{}", s);
    println!("{}", s2);
}

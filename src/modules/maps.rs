pub fn main() {
    maps();
}

fn maps() {
    let mut m = std::collections::HashMap::new();
    m.insert("Rust", "Programming");
    m.insert("Python", "Programming");
    m.insert("Java", "Programming");
    println!("{:?}", m);
}

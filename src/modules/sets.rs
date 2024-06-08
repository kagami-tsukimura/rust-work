pub fn main() {
    sets();
}

fn sets() {
    let mut s = std::collections::HashSet::new();
    s.insert(1);
    s.insert(2);
    s.insert(3);
    println!("{:?}", s);
}

use std::collections::HashSet;

pub fn main() {
    sets();
}

fn sets() {
    let mut s = HashSet::new();
    s.insert(1);
    s.insert(2);
    s.insert(3);
    println!("{:?}", s);
}

use std::collections::HashMap;

pub fn main() {
    maps();
}

fn maps() {
    let mut m = HashMap::new();
    m.insert("Japan", 12);
    m.insert("USA", 3);
    m.insert("China", 2);
    m.insert("India", 1);
    println!("{:?}", m);

    println!("-----");

    m.insert("Japan", 10);
    println!("{:?}", m);

    println!("{:?}", m.get("Japan"));
    println!("{:?}", m.get("Japan").unwrap());
    println!("{:?}", m.remove("India"));
    println!("{:?}", m);

    println!("-----");

    for (k, v) in &m {
        println!("{:?} {:?}", k, v);
    }
}

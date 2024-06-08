use std::collections::HashSet;

pub fn main() {
    sets();
}

fn sets() {
    let mut s1 = HashSet::new();
    // 重複はしない
    s1.insert(1);
    s1.insert(1);
    s1.insert(1);
    s1.insert(2);
    s1.insert(3);
    // HashSet: 順序はランダム
    println!("{:?}", s1);

    println!("{:?}", s1.contains(&2));
    println!("{:?}", s1.remove(&2));
    println!("{:?}", s1);

    println!("-----");

    let mut s2 = HashSet::new();
    s2.insert(1);
    s2.insert(2);
    s2.insert(3);
    s2.insert(5);

    let s3 = &s1 | &s2;
    println!("set1: {:?}", s1);
    println!("set2: {:?}", s2);
    println!("set3(set1 & set2): {:?}", s3);

    println!("-----");

    let s4 = &s1 & &s2;
    println!("set1: {:?}", s1);
    println!("set2: {:?}", s2);
    println!("set4(set1 & set2): {:?}", s4);

    println!("-----");

    let s5 = &s1 - &s2;
    println!("set1: {:?}", s1);
    println!("set2: {:?}", s2);
    println!("set5(set1 - set2): {:?}", s5);

    println!("-----");

    let s6 = &s1 ^ &s2;
    println!("set1: {:?}", s1);
    println!("set2: {:?}", s2);
    println!("set6(set1 ^ set2): {:?}", s6);
}

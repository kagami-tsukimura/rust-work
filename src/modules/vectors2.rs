pub fn main() {
    vectors2();
}

fn vectors2() {
    let v1 = vec!["Rust", "Python", "Java"];
    let v2 = vec!["C#", "Go"];
    println!("{:?}", v1);
    println!("{:?}", v2);

    // vector結合
    let v3 = [v1, v2].concat();

    println!("{:?}", v3);

    // vector分割
    let (v4, v5) = v3.split_at(3);
    println!("{:?}", v4);
    println!("{:?}", v5);
    println!("{:?}, {:?}", v4.len(), v5.len());

    println!("-----");

    let mut v6 = vec![3, 6, 1, 7, 2];
    println!("beore sort: {:?}", v6);
    // vector昇順
    v6.sort();
    println!("after sort: {:?}", v6);
    // vector降順
    v6.reverse();
    println!("after reverse: {:?}", v6);
}

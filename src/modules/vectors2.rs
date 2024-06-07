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

    println!("-----");

    #[derive(Debug)]
    struct S {
        val1: i32,
        val2: i32,
    }
    let mut v7 = vec![
        S { val1: 3, val2: 1 },
        S { val1: 2, val2: 2 },
        S { val1: 1, val2: 3 },
    ];
    v7.sort_by_key(|s| s.val1);
    println!("{:?}", v7);

    let v8 = vec![3, 6, 1, 7, 2];
    println!("{:?}", v8.contains(&6));
    println!("{:?}", v8.contains(&5));

    let x = v8.iter().position(|x| *x == 6);
    let y = v8.iter().position(|x| *x == 5);
    println!("{:?}", x);
    println!("{:?}", y);
}

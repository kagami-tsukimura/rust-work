pub fn main() {
    vectors();
}

fn vectors() {
    let v1 = vec!["Rust", "Python", "Java"];
    println!("{:?}", v1);
    println!("{:?}", v1.as_ptr());
    println!("{:?}", v1.len());
    println!("{:?}", v1.capacity());

    println!("-----");

    let v2 = vec!["Rust", "Python", "Java"];
    // vectorの要素取得
    // &v2[0]: 値をそのまま取得→存在しないインデックスでpanic
    println!("{:?}", &v2[0]);
    // v2.get(0): 値の中身を取り出す必要がある→存在しないインデックスでNone
    println!("{:?}", v2.get(0));
    println!("{:?}", v2.get(0).unwrap());

    println!("-----");

    let mut v3 = vec!["Rust", "Python", "Java"];
    v3.push("PHP");
    println!("{:?}", v3);
    println!("{:?}", v3.get(0).unwrap());
    let val = v3.pop();
    println!("{:?}", val);
    println!("{:?}", val.unwrap());
    println!("{:?}", v3);

    println!("-----");

    let mut v4 = vec!["Rust", "Python", "Java"];
    v4.insert(1, "C#");
    println!("{:?}", v4);
    v4.remove(2);
    println!("{:?}", v4);
}

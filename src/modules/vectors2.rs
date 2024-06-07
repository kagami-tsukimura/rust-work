pub fn main() {
    vectors2();
}

fn vectors2() {
    let v1 = vec!["Rust", "Python", "Java"];
    let v2 = vec!["C#", "Go"];
    let v3 = [v1, v2].concat();

    println!("{:?}", v3);
}

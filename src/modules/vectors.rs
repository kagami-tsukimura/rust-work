pub fn main() {
    vectors();
}

fn vectors() {
    let v1 = vec!["Rust", "Python", "Java"];
    println!("{:?}", v1);
    println!("{:?}", v1.as_ptr());
    println!("{:?}", v1.len());
    println!("{:?}", v1.capacity());

    let v2 = vec!["Rust", "Python", "Java"];
    println!("{:?}", &v2[0]);
}

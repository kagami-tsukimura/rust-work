pub fn main() {
    iterator_method1();
}

fn iterator_method1() {
    let v = vec![1, 2, 3, 4, 5];
    let m = v.iter().map(|x| x * 2);
    for val in m {
        println!("{}", val);
    }
}

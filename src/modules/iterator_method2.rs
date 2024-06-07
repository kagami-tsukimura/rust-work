pub fn main() {
    iterator_method2();
}

fn iterator_method2() {
    let v = vec![1, 2, 3, 4, 5];
    // イテレータを消費するメソッド: count
    // count
    let c = v.iter().count();
    println!("{:?}", c);
}

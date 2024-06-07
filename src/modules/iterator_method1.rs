pub fn main() {
    iterator_method1();
}

fn iterator_method1() {
    let v = vec![1, 2, 3, 4, 5];
    let m = v.iter().map(|x| x * 2);
    // イテレータの消費
    for val in m {
        println!("{}", val);
    }

    let c: Vec<_> = v.iter().map(|x| x * 2).collect();
    println!("{:?}", c);
}

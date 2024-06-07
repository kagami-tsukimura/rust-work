pub fn main() {
    iterator_method2();
}

fn iterator_method2() {
    let v = vec![1, 2, 3, 4, 5];
    // map
    let m = v.iter().map(|x| x * 2);
    // イテレータの消費
    for val in m {
        println!("{}", val);
    }

    let c: Vec<_> = v.iter().map(|x| x * 2).collect();
    println!("{:?}", c);

    // filter
    let f: Vec<_> = v.iter().filter(|x| *x % 2 != 0).collect();
    println!("{:?}", f);
}

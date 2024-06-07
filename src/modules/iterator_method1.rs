pub fn main() {
    iterator_method1();
}

fn iterator_method1() {
    let v = vec![1, 2, 3, 4, 5];
    // イテレータを消費しないメソッド: map, filter
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

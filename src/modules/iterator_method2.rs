pub fn main() {
    iterator_method2();
}

fn iterator_method2() {
    let v = vec![1, 2, 3, 4, 5];
    // イテレータを消費するメソッド: count, sum, product
    // count
    let c = v.iter().count();
    println!("{:?}", c);

    // sum, product
    let s: i32 = v.iter().sum();
    let p: i32 = v.iter().product();
    println!("{:?}", s);
    println!("{:?}", p);

    // min, max
    let min = v.iter().min();
    let max = v.iter().max();
    println!("{:?}", min);
    println!("{:?}", max);
}

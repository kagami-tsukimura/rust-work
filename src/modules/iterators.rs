pub fn main() {
    iterators();
}

fn iterators() {
    let v = vec![1, 2, 3, 4, 5];
    let v_iter = v.iter();
    for i in v_iter {
        println!("{:?}", i);
    }

    println!("-----");
    let mut v2_iter = v.iter();
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
}

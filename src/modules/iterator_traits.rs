pub fn main() {
    iterator_traits();
}

fn iterator_traits() {
    let v = vec![1, 2, 3, 4, 5];
    let mut v_iter = v.iter();
    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
}

pub fn main() {
    iterators();
}

fn iterators() {
    let v = vec![1, 2, 3, 4, 5];
    let v_iter = v.iter();
    for i in v_iter {
        println!("{}", i);
    }
}

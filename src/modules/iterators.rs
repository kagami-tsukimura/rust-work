pub fn main() {
    iterators();
}

fn iterators() {
    let v = vec![1, 2, 3];
    for i in v.iter() {
        println!("{}", i);
    }
}

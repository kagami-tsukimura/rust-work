pub fn main() {
    counter_iterate();
}

fn counter_iterate() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut v_iter = v.iter();
    let mut cnt = 0;
    while let Some(i) = v_iter.next() {
        println!("{}: {}", cnt, i);
        cnt += 1;
    }
}

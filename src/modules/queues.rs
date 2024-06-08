use std::collections::VecDeque;

pub fn main() {
    queues();
}

fn queues() {
    let mut q = VecDeque::new();
    // let mut q = VecDeque::from(vec![1, 2, 3]);

    q.push_back(1);
    q.push_back(2);
    q.push_back(3);
    println!("{:?}", q);
}

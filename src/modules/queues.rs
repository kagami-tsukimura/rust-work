use std::collections::VecDeque;

pub fn main() {
    queues();
}

fn queues() {
    let mut q = VecDeque::new();
    q.push_back(1);
    q.push_back(2);
    q.push_back(3);
    println!("{:?}", q);
    let x = q.pop_front();
    println!("{:?}", x);
    println!("{:?}", q);
}

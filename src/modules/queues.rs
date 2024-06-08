use std::collections::{BinaryHeap, VecDeque};

pub fn main() {
    queues();
}

fn queues() {
    let mut q = VecDeque::new();
    // let mut q = VecDeque::from(vec![1, 2, 3]);

    q.push_back(1);
    q.push_back(2);
    q.push_back(3);
    q.push_back(4);
    println!("{:?}", q);

    println!("{:?}", q.pop_front());
    println!("{:?}", q.pop_front());
    println!("{:?}", q);

    let mut bh = BinaryHeap::new();
    bh.push(1);
    bh.push(10);
    bh.push(20);
    bh.push(2);
    println!("{:?}", bh);
    println!("{:?}", bh.pop());
    println!("{:?}", bh);
}

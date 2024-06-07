struct Counter {
    start: u32,
    end: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.start >= self.end {
            None
        } else {
            let result = Some(self.start);
            self.start += 1;
            result
        }
    }
}

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

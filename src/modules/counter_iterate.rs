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
    let mut c = Counter { start: 1, end: 5 };

    for i in c {
        println!("{}", i);
    }
}

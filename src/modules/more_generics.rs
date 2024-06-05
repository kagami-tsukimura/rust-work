use std::fmt::{Debug, Display};

struct Point<T> {
    x: T,
    y: T,
}

impl<T: PartialOrd + Debug> Point<T> {
    fn max(&self) -> &T {
        if self.x >= self.y {
            &self.x
        } else {
            &self.y
        }
    }

    fn print_arg<U: Display>(&self, val: U) {
        println!("self.x: {:?}", self.x);
        println!("val: {}", val);
    }
}

pub fn main() {
    more_generics();
}

fn more_generics() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = Point { x: "a", y: "d" };

    println!("p1.max: {:?}", p1.max());
    println!("p2.max: {:?}", p2.max());
    println!("p3.max: {:?}", p3.max());

    p1.print_arg("hoge");
    p1.print_arg(1);
    p1.print_arg(true);
}

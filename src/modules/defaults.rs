use rust_work::sample_trait::{Circle, Rectangle, Shape};
pub fn main() {
    defaults();
}

fn defaults() {
    let rect = Rectangle {
        width: 4.0,
        height: 5.0,
    };

    let circle = Circle { radius: 3.0 };

    println!("Rectangle default: {}", rect.default_something());
    println!("Circle default: {}", circle.default_something());
}

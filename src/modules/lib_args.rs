use rust_work::sample_trait::{double_area, Circle, Rectangle, Shape};
pub fn main() {
    lib_args();
}

fn lib_args() {
    let rect = Rectangle {
        width: 4.0,
        height: 5.0,
    };

    let circle = Circle { radius: 3.0 };

    println!("Rectangle double_area: {}", double_area(&rect));
    println!("Circle double_area: {}", double_area(&circle));
}

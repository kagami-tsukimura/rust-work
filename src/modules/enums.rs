enum Shape {
    Circle,
    Square(u32),
    Triangle { base: u32, heght: u32 },
}

impl Shape {
    fn sample_method(&self) {
        println!("call sample_method");
    }
}

pub fn main() {
    enums()
}

fn enums() {
    let c = Shape::Circle;
    let s: Shape = Shape::Square(1);
    let t = Shape::Triangle { base: 10, heght: 5 };

    c.sample_method();
    s.sample_method();
    t.sample_method();
}

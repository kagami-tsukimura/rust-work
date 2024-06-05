use rust_work::sample_trait::{Circle, Rectangle, Shape};

mod modules {
    // pub mod basic;
    // pub mod flow;
    // pub mod fizz_buzz;
    // pub mod ownership;
    // pub mod refs;
    // pub mod smart_pointer;
    // pub mod structs;
    // pub mod enums;
    // pub mod options;
    // pub mod crates;
    // pub mod mods;
    // pub mod sub_struct;
    // pub mod binaries;
}

fn main() {
    // modules::basic::main();
    // modules::flow::main();
    // modules::fizz_buzz::main(20);
    // modules::ownership::main();
    // modules::refs::main();
    // modules::smart_pointer::main();
    // modules::structs::main();
    // modules::enums::main();
    // modules::options::main();
    // modules::crates::main();
    // modules::mods::main();
    // modules::sub_struct::main();
    // modules::binaries::main();

    let rect = Rectangle {
        width: 4.0,
        height: 5.0,
    };

    let circle = Circle { radius: 3.0 };

    println!("Rectangle area is {}", rect.calc_area());
    println!("Rectangle perimeter is {}", rect.calc_perimeter());
    Rectangle::do_something();
    println!("Circle area is {}", circle.calc_area());
    println!("Circle perimeter is {}", circle.calc_perimeter());
    Circle::do_something();
}

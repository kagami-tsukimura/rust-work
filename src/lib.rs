pub mod sample_trait {
    pub trait Shape {
        fn calc_area(&self) -> f64;
        fn calc_perimeter(&self) -> f64;
        fn do_something();
    }
}

// ! lib.rs documentation
pub fn say_hello() {
    println!("Hello!");
}

/// Say goodbye
/// ### Example
pub fn say_goodbye() {
    say_bye();
}

fn say_bye() {
    println!("Bye!");
}

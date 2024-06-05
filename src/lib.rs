pub mod sample_trait {
    pub trait Shape {
        fn calc_area(&self) -> f64;
        fn calc_perimeter(&self) -> f64;
        fn default_something(&self) -> &str {
            "This is default method!"
        }
        fn do_something();
    }

    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }

    impl Shape for Rectangle {
        fn calc_area(&self) -> f64 {
            self.width * self.height
        }
        fn calc_perimeter(&self) -> f64 {
            2.0 * (self.width + self.height)
        }
        fn default_something(&self) -> &str {
            "This is Rectangle default!"
        }
        fn do_something() {
            println!("do something Rectangle");
        }
    }

    pub struct Circle {
        pub radius: f64,
    }

    impl Shape for Circle {
        fn calc_area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }

        fn calc_perimeter(&self) -> f64 {
            self.radius * 2.0 * std::f64::consts::PI
        }

        fn do_something() {
            println!("do something Circle");
        }
    }

    pub fn double_area(shape: &impl Shape) {
        println!("{}", shape.calc_area() * 2.0);
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

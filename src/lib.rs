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

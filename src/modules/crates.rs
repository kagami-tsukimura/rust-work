use rand::Rng;

pub fn main() {
    rands();
}

fn rands() {
    let random_number = rand::thread_rng().gen_range(1..101);
    println!("{}", random_number);
}

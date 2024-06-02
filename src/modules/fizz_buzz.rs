pub fn main(count: i32) {
    fizzbuzz(count);
}

fn fizzbuzz(count: i32) {
    for i in 1..=count {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz")
        } else if i % 3 == 0 {
            println!("Fizz")
        } else if i % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", i)
        }
    }
}

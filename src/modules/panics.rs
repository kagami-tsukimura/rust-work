pub fn main() {
    panics();
}

fn panics() {
    // panic: 異常終了
    println!("{}", [1, 2, 3][100]);
    panic!("Hello, panic!");
}

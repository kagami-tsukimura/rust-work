struct Rectangle {
    width: u32,
    height: u32,
}

pub fn main() {
    structs();
}

fn structs() {
    let height = 20;
    let mut rectangle = Rectangle { width: 10, height };

    println!("width; {}", rectangle.width);
    println!("height; {}", rectangle.height);
    println!("rectangle: {}", rectangle.width * rectangle.height);

    rectangle.height = 30;

    println!("rectangle: {}", rectangle.width * rectangle.height);
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // selfでも所有権が移動するため、共有参照とすること
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 型関連関数: selfなし
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
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

    println!("-----");

    println!("area: {}", rectangle.area());

    println!("-----");
    type_relation();
}

fn type_relation() {
    let rectangle = Rectangle::new(10, 50);

    println!("rectanfle_area: {}", rectangle.area());
}

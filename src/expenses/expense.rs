use std::io;

pub fn main() {
    expense();
}

fn expense() {
    let mut service_type = String::new();
    println!("実行したい内容を入力してください（0: 登録, 1: 集計）");
    io::stdin().read_line(&mut service_type).unwrap();
    // String → u8にparse
    let service_type: u8 = service_type.trim().parse().expect("数値を入力してください");
}

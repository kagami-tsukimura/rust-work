use crate::services;
use std::io;

fn input_register_type() -> u8 {
    println!("登録種別を入力してください(0: 収入, 1: 支出)");
    let mut register_type = String::new();
    io::stdin()
        .read_line(&mut register_type)
        .expect("登録種別の入力に失敗しました");
    let register_type: u8 = register_type
        .trim()
        .parse()
        .expect("登録種別は数値を入力してください");

    services::validate::InputValidator::validate_register_type(register_type);
    register_type
}

fn input_name() -> String {
    println!("品目名を入力してください");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("品目名の入力に失敗しました");
    name.trim().to_string()
}

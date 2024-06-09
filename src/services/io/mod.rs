use crate::models;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_data_or_create_new_data(file_path: &str) -> Vec<models::Item> {
    let file = File::open(file_path);

    match file {
        Ok(f) => {
            let buf_reader = BufReader::new(f);
            serde_json::from_reader(buf_reader).expect("デシリアライズに失敗しました")
        }
        Err(_) => {
            println!("新規ファイルを作成します");
            Vec::new()
        }
    }
}

use std::fs;

pub fn main() {
    file_systems();
}

fn file_systems() {
    fs::create_dir("src/test1").unwrap();
    fs::remove_dir_all("src/test2/test2-1/test2-2").unwrap();
}

use std::fs;

pub fn main() {
    file_systems();
}

fn file_systems() {
    let dir1 = "src/test1";
    let dir2 = "src/test2/test2-1/test2-2";

    // fs::create_dir(dir1).unwrap();
    // fs::create_dir_all(dir2).unwrap();

    fs::remove_dir(dir1).unwrap();
    fs::remove_dir_all(dir2).unwrap();
}

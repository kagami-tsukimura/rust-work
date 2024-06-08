use std::path::{Path, PathBuf};

pub fn main() {
    paths();
}

fn paths() {
    let path = Path::new("src");
    println!("path_exist: {:?}", path.exists());
    println!("is_dir: {:?}", path.is_dir());
    println!("is_file: {:?}", path.is_file());
    println!("file_name: {:?}", path.file_name());

    let mut path_buf = PathBuf::from("src");
    // path_buf.push("sample1.txt");
    path_buf.push("main.rs");
    println!("path_buf: {:?}", path_buf);
    path_buf.set_file_name("path.txt");
    println!("path_buf: {:?}", path_buf);
    path_buf.pop();
    println!("path_buf: {:?}", path_buf);
}

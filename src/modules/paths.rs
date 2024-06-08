use std::path::{Path, PathBuf};

pub fn main() {
    paths();
}

fn paths() {
    let path = Path::new("src");
    println!("path_exist: {:?}", path.exists());
    println!("is_dir: {:?}", path.is_dir());
    println!("is_file: {:?}", path.is_file());
}

use std::fs;

pub fn main() {
    file_systems();
}

fn file_systems() {
    let dir1 = "src/test1";
    let dir2 = "src/test2/test2-1/test2-2";
    let dir3 = "src/test2/";

    let file1 = "src/sample1.txt";
    let file2 = "src/sample2.txt";
    let file3 = "src/sample3.txt";
    let mvfile = "src/test1/sample3.txt";

    fs::create_dir(dir1).unwrap();
    // fs::create_dir_all(dir2).unwrap();

    // fs::remove_dir(dir1).unwrap();
    // fs::remove_dir_all(dir3).unwrap();

    // // fs::remove_file(file1).unwrap();
    // fs::copy(file2, file3).unwrap();

    fs::rename(file3, mvfile).unwrap();
}

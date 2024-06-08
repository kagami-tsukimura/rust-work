pub fn main() {
    outfiles();
}

fn outfiles() {
    let file_path = "./sample1.txt";
    let mut f = File::create(file_path).expect("Failed to create file");
    f.write_all(b"Hello, Rust!\n")
        .expect("Failed to write to file");
}

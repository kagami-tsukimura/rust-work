pub fn main() {
    sub_struct();
}

fn sub_struct() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
}

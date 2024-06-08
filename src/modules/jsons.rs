pub fn main() {
    jsons();
}

fn jsons() {
    let json = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;
    println!("{}", json);
}

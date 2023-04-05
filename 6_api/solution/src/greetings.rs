pub fn greets(name: &str) -> String {
    format!("Hello, {} 🦀 !", name)
}

#[test]
fn test_greets() {
    assert_eq!(greets("You"), String::from("Hello, You 🦀 !"));
}

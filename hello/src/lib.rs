pub fn print() {
    println!("Hello, world!");
}

pub fn world() -> String {
    String::from("Hello, world!")
}

#[test]
fn world_returns_hello_world() {
    let result = world();
    assert_eq!(
        result, "Hello, world!",
        "world(): want 'Hello, world!', got '{result}'"
    );
}

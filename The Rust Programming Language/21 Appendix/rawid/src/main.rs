fn r#match(_needle: &str, _haystick: &str) -> bool {
    // haystick.contains(needle)
    true
}

fn main() {
    assert!(r#match("foo", "foobar"));

    // const PI: f64 = 3.1415;
    const PI: f64 = std::f64::consts::PI;
    println!("circle area: {}", PI * 3.0 * 3.0);
}

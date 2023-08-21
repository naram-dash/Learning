use ch11_test_organization;

mod common;

#[test]
fn it_add_twos() {
    common::setup();
    assert_eq!(4, ch11_test_organization::add_two(2)); 
}
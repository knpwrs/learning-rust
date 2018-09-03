extern crate c11_testing;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(c11_testing::add_two(2), 4);
}

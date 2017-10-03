extern crate testing;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(testing::add_two(2), 4);
}
use n_tests;
mod common;

#[test]
fn it_adds() {
    common::setup();
    assert_eq!(4, n_tests::add(2, 2));
}

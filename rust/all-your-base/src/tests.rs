use super::*;

#[test]
fn test_from() {
    assert_eq!(conv_from_base(&[1, 0, 1], 2), 5);
}

#[test]
fn test_to() {
    assert_eq!(conv_to_base(5, 2), [1, 0, 1]);
}

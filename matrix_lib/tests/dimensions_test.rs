extern crate matrix_lib;

use matrix_lib::dimensions::Dimensions;
use matrix_lib::errors::*;

#[test]
fn dimensions_init_from_vector() {
    let v = vec![vec![1, 2, 3], vec![2, 3, 4]];
    assert_eq!(Dimensions::from_vector(&v), Ok(Dimensions::new(2, 3)));

    let v: Vec<Vec<()>> = vec![];
    assert_eq!(Dimensions::from_vector(&v), Ok(Dimensions::new(0, 0)));

    let v = vec![vec![1, 2, 3], vec![2, 3]];
    assert_eq!(
        Dimensions::from_vector(&v),
        Err(MathError::IncorrectVectorDimensions)
    );

    let v: Vec<Vec<()>> = vec![vec![()]];
    assert_eq!(Dimensions::from_vector(&v), Ok(Dimensions::new(1, 1)));
}

#[test]
fn dimensions_is_square() {
    let d = Dimensions::new(3, 5);
    assert!(!d.is_square(), "Dims are not square");

    let d = Dimensions::new(5, 5);
    assert!(d.is_square(), "Dims expected to be square");
}

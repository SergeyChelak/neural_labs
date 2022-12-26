extern crate matrix_lib;

use matrix_lib::{
    matrix::Matrix,
    convolution::*, errors::MathResult,
};

#[test]
fn convolution_cross_correlation1() -> MathResult<()> {
    let input = Matrix::from_vector(&vec![
        vec![1.0, 6.0, 2.0],
        vec![5.0, 3.0, 1.0],
        vec![7.0, 0.0, 4.0],
    ])?;

    let kernel = Matrix::from_vector(&vec![
        vec![ 1.0, 2.0],
        vec![-1.0, 0.0],
    ])?;

    let expected = Matrix::from_vector(&vec![
        vec![8.0, 7.0],
        vec![4.0, 5.0],
    ])?;

    let result = cross_correlation(&input, &kernel)?;
    assert!(expected == result, "Cross-corrleation calculated incorrectly");
    Ok(())
}

#[test]
fn convolution_cross_correlation2() -> MathResult<()> {
    let input = Matrix::from_vector(&vec![
        vec![3.0, 3.0, 2.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0, 3.0, 1.0],
        vec![3.0, 1.0, 2.0, 2.0, 3.0],
        vec![2.0, 0.0, 0.0, 2.0, 2.0],
        vec![2.0, 0.0, 0.0, 0.0, 1.0],
    ])?;

    let kernel = Matrix::from_vector(&vec![
        vec![0.0, 1.0, 2.0],
        vec![2.0, 2.0, 0.0],
        vec![0.0, 1.0, 2.0],
    ])?;

    let expected = Matrix::from_vector(&vec![
        vec![12.0, 12.0, 17.0],
        vec![10.0, 17.0, 19.0],
        vec![ 9.0,  6.0, 14.0],
    ])?;

    let result = cross_correlation(&input, &kernel)?;
    assert!(expected == result, "Cross-corrleation calculated incorrectly");
    Ok(())
}

#[test]
fn convolution_full_conv() -> MathResult<()> {
    let input = Matrix::from_vector(&vec![
        vec![1.0, 6.0, 2.0],
        vec![5.0, 3.0, 1.0],
        vec![7.0, 0.0, 4.0],
    ])?;

    let kernel = Matrix::from_vector(&vec![
        vec![ 1.0, 2.0],
        vec![-1.0, 0.0],
    ])?;

    let expected = Matrix::from_vector(&vec![
        vec![ 0.0, -1.0, -6.0, -2.0],
        vec![ 2.0,  8.0,  7.0,  1.0],
        vec![10.0,  4.0,  5.0, -3.0],
        vec![14.0,  7.0,  8.0,  4.0],
    ])?;

    let result = full_convolution(&input, &kernel)?;
    assert!(expected == result, "Full convolution calculated incorrectly");
    Ok(())
}
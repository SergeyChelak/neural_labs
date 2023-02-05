extern crate tensor_lib;

// use std::env;

use tensor_lib::{
    tensor::*,
    common::TensorResult,
};

#[test]
fn tensor_product2d_1_test() -> TensorResult<()> {
    let a = Tensor::matrix(&vec![
        vec![1.0, 0.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 2.0]

    ])?;

    let b = Tensor::matrix(&vec![
        vec![1.0, 2.0, 1.0],
        vec![2.0, 3.0, 1.0],
        vec![4.0, 2.0, 2.0]
    ])?;

    let expected = Tensor::matrix(&vec![
        vec![ 5.0, 4.0, 3.0],
        vec![ 8.0, 9.0, 5.0],
        vec![ 6.0, 5.0, 3.0],
        vec![11.0, 9.0, 6.0]

    ])?;

    let ab = a.product2d(&b)?;
    // assert!(ab == expected, "Matrix product implemented incorrectly");
    // Ok(())
    todo!()
}

#[test]
fn tensor_product2d_2_test() -> TensorResult<()> {
    let a = Tensor::matrix(&vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],

    ])?;

    let b = Tensor::matrix(&vec![
        vec![ 7.0,  8.0],
        vec![ 9.0, 10.0],
        vec![11.0, 12.0]
    ])?;

    let expected = Tensor::matrix(&vec![
        vec![ 58.0,  64.0],
        vec![139.0, 154.0]

    ])?;

    let ab = a.product2d(&b)?;
    // assert!(ab == expected, "Matrix product implemented incorrectly");
    // Ok(())
    todo!()
}

#[test]
fn tensor_product2d_testfailure() -> TensorResult<()> {
    let a = Tensor::matrix(&vec![
        vec![1.0, 0.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 2.0]

    ])?;

    let b = Tensor::matrix(&vec![
        vec![1.0, 0.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 2.0],
        vec![1.0, 1.0, 2.0]
    ])?;

    assert!(a.product2d(&b).is_err());
    Ok(())
}
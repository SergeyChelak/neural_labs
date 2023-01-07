extern crate tensor_lib;

// use std::env;

use tensor_lib::{
    tensor::*,
    common::TensorResult,
};

#[test]
fn tensor_mul_assign() -> TensorResult<()> {
    let matrix = vec![
        vec![1.0, 0.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 2.0],
    ];
    let mut tensor = Tensor::matrix(&matrix)?;
    tensor.mul_assign(2.0);

    let expected = vec![
        vec![2.0, 0.0, 2.0],
        vec![4.0, 2.0, 2.0],
        vec![0.0, 2.0, 2.0],
        vec![2.0, 2.0, 4.0],
    ];

    for i in 0..expected.len() {
        for j in 0..expected[i].len() {
            assert_eq!(tensor.get(&vec![i, j])?, expected[i][j], "Tensor mul_assign implemented incorrectly");
        }
    }
    Ok(())
}

#[test]
fn tensor_div_assign() -> TensorResult<()> {
    let matrix = vec![
        vec![2.0, 0.0, 2.0],
        vec![4.0, 2.0, 2.0],
        vec![0.0, 2.0, 2.0],
        vec![2.0, 2.0, 4.0],
    ];

    let mut tensor = Tensor::matrix(&matrix)?;
    tensor.div_assign(2.0);

    let expected = vec![
        vec![1.0, 0.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 2.0],
    ];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            assert_eq!(tensor.get(&vec![i, j])?, expected[i][j], "Tensor div_assign implemented incorrectly");
        }
    }
    Ok(())
}
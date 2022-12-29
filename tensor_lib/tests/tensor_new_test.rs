extern crate tensor_lib;

// use std::env;

use tensor_lib::{
    tensor::*,
    common::TensorResult,
};

#[test]
fn tensor_create_empty() {
    let tensor = Tensor::<i32>::new(vec![], i32::default());
    todo!()
}

#[test]
fn tensor_create_1d() -> TensorResult<()> {
    let arr = vec![1i32, 2, 3, 4, 5];
    let tensor = Tensor::vector(&arr)?;
    for i in 0..arr.len() {
        assert_eq!(tensor.get(&vec![i])?, arr[i], "Incorrect tensor value in vector representation");
    }
    Ok(())
}

#[test]
fn tensor_create_2d() -> TensorResult<()> {
 //   env::set_var("RUST_BACKTRACE", "1");
    let matrix = vec![
        vec![1.0, 0.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 2.0],
    ];
    let tensor = Tensor::matrix(&matrix)?;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            assert_eq!(tensor.get(&vec![i, j])?, matrix[i][j], "Tensor for matrix filled incorrectly");
        }
    }
    Ok(())
}

#[test]
fn tensor_create_3d() -> TensorResult<()> {
    let arr = vec![
        vec![
            vec![1.0, 0.0, 1.0],
            vec![2.0, 1.0, 1.0],
            vec![0.0, 1.0, 1.0],
            vec![1.0, 1.0, 2.0],
        ],
        vec![
            vec![5.0, 1.0, 3.0],
            vec![6.0, 2.0, 4.0],
            vec![7.0, 3.0, 5.0],
            vec![8.0, 4.0, 6.0],
        ],
        vec![
            vec![1.9, 0.0, 1.9],
            vec![2.8, 1.9, 1.9],
            vec![0.0, 1.9, 1.9],
            vec![1.9, 1.9, 2.8],
        ],
    ];
    let tensor = Tensor::tensor3d(&arr)?;
    for d in 0..arr.len() {
        for i in 0..arr[d].len() {
            for j in 0..arr[d][i].len() {
                assert_eq!(tensor.get(&vec![d, i, j])?, arr[d][i][j], "Tensor for matrix filled incorrectly");
            }
        }
    }
    Ok(())
}
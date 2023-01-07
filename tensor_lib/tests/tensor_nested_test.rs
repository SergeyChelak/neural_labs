extern crate tensor_lib;

use std::env;

use tensor_lib::{
    tensor::*,
    common::TensorResult,
};

#[test]
fn tensor_nested_get() -> TensorResult<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let expected = vec![
        vec![5.0, 1.0, 3.0],
        vec![6.0, 2.0, 4.0],
        vec![7.0, 3.0, 5.0],
        vec![8.0, 4.0, 6.0],
    ];

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
    let nested = tensor.nested_tensor(&vec![1])?;
    
    for i in 0..expected.len() {
        for j in 0..expected[i].len() {
            let v = nested.get(&vec![i, j])?;
            assert_eq!(v, expected[i][j], "Tensor for matrix filled incorrectly");
        }
    }
    Ok(())
}
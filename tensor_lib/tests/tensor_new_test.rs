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

// #[test]
// fn tensor_create_3d() {
//     let matrix3d = vec![
//         vec![
//             vec![1.0, 0.0, 1.0],
//             vec![2.0, 1.0, 1.0],
//             vec![0.0, 1.0, 1.0],
//             vec![1.0, 1.0, 2.0],
//         ],
//         vec![
//             vec![5.0, 1.0, 3.0],
//             vec![6.0, 2.0, 4.0],
//             vec![7.0, 3.0, 5.0],
//             vec![8.0, 4.0, 6.0],
//         ],
//         vec![
//             vec![1.9, 0.0, 1.9],
//             vec![2.8, 1.9, 1.9],
//             vec![0.0, 1.9, 1.9],
//             vec![1.9, 1.9, 2.8],
//         ],
//     ];
//     let tensor = Tensor::new(vec![3, 4, 3], |indicies| {
//         let depth = indicies[0];
//         let row = indicies[1];
//         let col = indicies[2];
//         matrix3d[depth][row][col]
//     });
//     assert_eq!(tensor.rank(), 3, "Tensor 3D dimensions are wrong");
//     for d in 0..matrix3d.len() {
//         for i in 0..matrix3d[d].len() {
//             for j in 0..matrix3d[d][i].len() {
//                 assert_eq!(tensor[d][i][j].value().unwrap(), matrix3d[d][i][j], "Tensor for matrix filled incorrectly");
//             }
//         }
//     }
// }
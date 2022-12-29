// extern crate tensor_lib;

// use std::env;

// use tensor_lib::{
//     tensor::*,
// };

// #[test]
// fn tensor_create_0d() {
//     let tensor = Tensor::new(vec![], |_| { 123.0 });
//     assert_eq!(tensor.rank(), 0, "Tensor for scalar is wrong");
//     assert_eq!(tensor.value().unwrap(), 123.0, "Invalid scalar value");
// }

// #[test]
// fn tensor_create_1d() {
//     let tensor = Tensor::new(vec![10], |indices| {
//         let v = indices.first().unwrap();
//         *v as TensorFloat
//     });
//     assert_eq!(tensor.rank(), 1, "Tensor dimension for vector is wrong");
//     for i in 0..10 {
//         assert_eq!(tensor[i].value().unwrap(), i as TensorFloat, "Incorrect tensor value in vector representation");
//     }
// }

// #[test]
// fn tensor_create_2d() {
//  //   env::set_var("RUST_BACKTRACE", "1");

//     let matrix = vec![
//         vec![1.0, 0.0, 1.0],
//         vec![2.0, 1.0, 1.0],
//         vec![0.0, 1.0, 1.0],
//         vec![1.0, 1.0, 2.0],
//     ];
//     let tensor = Tensor::new(vec![4, 3], |indicies| {
//         let row = indicies[0];
//         let col = indicies[1];
//         matrix[row][col]
//     });
//     assert_eq!(tensor.rank(), 2, "Tensor dimensions for matrix are wrong");
//     for i in 0..matrix.len() {
//         for j in 0..matrix[i].len() {
//             assert_eq!(tensor[i][j].value().unwrap(), matrix[i][j], "Tensor for matrix filled incorrectly");
//         }
//     }
// }

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
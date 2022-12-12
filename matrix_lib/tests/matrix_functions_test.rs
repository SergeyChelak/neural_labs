extern crate matrix_lib;

use matrix_lib::{
    matrix::Matrix,
};

#[test]
fn matrix_eq() {
    let a = vec![
        vec![1.0, 2.0, 3.0], 
        vec![2.0, 3.0, 4.0]
    ];
    let b = a.clone();
    let m1 = Matrix::from_vector(&a);
    let m2 = Matrix::from_vector(&b);
    assert!(m1 == m2, "Matrices should be equal");

    let b = vec![
        vec![5.0, 2.0, 3.0], 
        vec![2.0, 3.0, 4.0]
    ];
    let m2 = Matrix::from_vector(&b);
    assert!(m1 != m2, "Matrices shouldn't be equal");


    let b = vec![
        vec![5.0, 2.0], 
        vec![2.0, 3.0]
    ];
    let m2 = Matrix::from_vector(&b);
    assert!(m1 != m2, "Matrices shouldn't be equal");
}
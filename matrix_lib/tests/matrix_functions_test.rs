extern crate matrix_lib;

use matrix_lib::{
    errors::*,
    matrix::Matrix,
    matrix_functions::*,
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

#[test]
fn matrix_function_add() -> MathResult<()> {
    let m1 = Matrix::from_vector(&vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0]
    ])?;

    let m2 = Matrix::from_vector(&vec![
        vec![9.0, 8.0, 7.0],
        vec![6.0, 5.0, 4.0],
        vec![3.0, 2.0, 1.0]
    ])?;

    let expected = Matrix::from_vector(&vec![
        vec![10.0, 10.0, 10.0],
        vec![10.0, 10.0, 10.0],
        vec![10.0, 10.0, 10.0]
    ])?;

    let sum = add(&m1, &m2)?;
    assert!(sum == expected, "Matrix sum implemented incorrectly");
    Ok(())
}

#[test]
fn matrix_function_sub() -> MathResult<()> {
    let m1 = Matrix::from_vector(&vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0]
    ])?;

    let m2 = Matrix::from_vector(&vec![
        vec![9.0, 8.0, 7.0],
        vec![6.0, 5.0, 4.0],
        vec![3.0, 2.0, 1.0]
    ])?;

    let expected = Matrix::from_vector(&vec![
        vec![-8.0, -6.0, -4.0],
        vec![-2.0,  0.0,  2.0],
        vec![ 4.0,  6.0,  8.0]
    ])?;

    let sum = sub(&m1, &m2)?;
    assert!(sum == expected, "Matrix subtraction implemented incorrectly");
    Ok(())
}

#[test]
#[ignore = "not implemented"]
fn matrix_function_mul() -> MathResult<()> {
    todo!()
}

#[test]
#[ignore = "not implemented"]
fn matrix_function_div() -> MathResult<()> {
    todo!()
}

#[test]
fn matrix_operation_product() -> MathResult<()> {
    let a = Matrix::from_vector(&vec![
        vec![1.0, 0.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 2.0]

    ])?;

    let b = Matrix::from_vector(&vec![
        vec![1.0, 2.0, 1.0],
        vec![2.0, 3.0, 1.0],
        vec![4.0, 2.0, 2.0]
    ])?;

    let expected = Matrix::from_vector(&vec![
        vec![ 5.0, 4.0, 3.0],
        vec![ 8.0, 9.0, 5.0],
        vec![ 6.0, 5.0, 3.0],
        vec![11.0, 9.0, 6.0]

    ])?;

    let ab = product(&a, &b)?;
    assert!(ab == expected, "Matrix product implemented incorrectly");

    let a = Matrix::from_vector(&vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],

    ])?;

    let b = Matrix::from_vector(&vec![
        vec![ 7.0,  8.0],
        vec![ 9.0, 10.0],
        vec![11.0, 12.0]
    ])?;

    let expected = Matrix::from_vector(&vec![
        vec![ 58.0,  64.0],
        vec![139.0, 154.0]

    ])?;

    let ab = product(&a, &b)?;
    assert!(ab == expected, "Matrix product implemented incorrectly");

    let identity = Matrix::identity(a.cols());
    assert!(product(&a, &identity)? == a, "Matrix product identity should return the same matrix");
    Ok(())
}

#[test]
fn matrix_operation_product_failure() -> MathResult<()> {
    let a = Matrix::from_vector(&vec![
        vec![1.0, 0.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 2.0]

    ])?;

    let b = Matrix::from_vector(&vec![
        vec![1.0, 0.0, 1.0],
        vec![2.0, 1.0, 1.0],
        vec![0.0, 1.0, 1.0],
        vec![1.0, 1.0, 2.0],
        vec![1.0, 1.0, 2.0]
    ])?;

    assert!(product(&a, &b).is_err());
    Ok(())
}

#[test]
fn matrix_operation_mul_scalar() -> MathResult<()> {
    let m = Matrix::from_vector(&vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0]
    ])?;

    let expected = Matrix::from_vector(&vec![
        vec![ 2.0,  4.0,  6.0],
        vec![ 8.0, 10.0, 12.0],
        vec![14.0, 16.0, 18.0]
    ])?;

    assert!(m.mul(2.0) == expected, "Matrix scalar multiplication implemented incorrectly");
    Ok(())
}

#[test]
fn matrix_operation_transpose() -> MathResult<()> {
    let a = Matrix::from_vector(&vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0]
    ])?;

    let expected = Matrix::from_vector(&vec![
        vec![1.0, 4.0],
        vec![2.0, 5.0],
        vec![3.0, 6.0]

    ])?;
    assert!(a.transpose() == expected, "Matrix transpose implemented incorrectly");
    Ok(())
}

#[test]
fn matrix_operation_power_integer() -> MathResult<()> {
    let mut m = Matrix::from_vector(&vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0]
    ])?;

    let expected = Matrix::from_vector(&vec![
        vec![ 1.0,  4.0,  9.0],
        vec![16.0, 25.0, 36.0],
        vec![49.0, 64.0, 81.0]
    ])?;
    m = m.powi(2);
    assert!(m == expected, "Matrix power by scalar implemented incorrectly");
    Ok(())
}

#[test]
fn matrix_operation_add_assign() -> MathResult<()> {
    let mut m1 = Matrix::from_vector(&vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0]
    ])?;

    let m2 = Matrix::from_vector(&vec![
        vec![9.0, 8.0, 7.0],
        vec![6.0, 5.0, 4.0],
        vec![3.0, 2.0, 1.0]
    ])?;

    let expected = Matrix::from_vector(&vec![
        vec![10.0, 10.0, 10.0],
        vec![10.0, 10.0, 10.0],
        vec![10.0, 10.0, 10.0]
    ])?;

    m1.add_assign(&m2)?;
    assert!(m1 == expected, "Matrix add & assign implemented incorrectly");
    Ok(())
}

#[test]
fn matrix_operation_sub_assign() -> MathResult<()> {
    let mut m1 = Matrix::from_vector(&vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0]
    ])?;

    let m2 = Matrix::from_vector(&vec![
        vec![9.0, 8.0, 7.0],
        vec![6.0, 5.0, 4.0],
        vec![3.0, 2.0, 1.0]
    ])?;

    let expected = Matrix::from_vector(&vec![
        vec![-8.0, -6.0, -4.0],
        vec![-2.0,  0.0,  2.0],
        vec![ 4.0,  6.0,  8.0]
    ])?;

    m1.sub_assign(&m2)?;
    assert!(m1 == expected, "Matrix sub & assign implemented incorrectly");
    Ok(())
}

#[test]
fn matrix_operation_scalar_multiplication() -> MathResult<()> {
    let mut m = Matrix::from_vector(&vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0]
    ])?;

    let expected = Matrix::from_vector(&vec![
        vec![ 2.0,  4.0,  6.0],
        vec![ 8.0, 10.0, 12.0],
        vec![14.0, 16.0, 18.0]
    ])?;
    m.mul_assign(2.0);
    assert!(m == expected, "Matrix scalar multiplication & assign implemented incorrectly");
    Ok(())
}

#[test]
fn matrix_operation_scalar_divide() -> MathResult<()> {
    let expected = Matrix::from_vector(&vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0]
    ])?;

    let mut m = Matrix::from_vector(&vec![
        vec![ 2.0,  4.0,  6.0],
        vec![ 8.0, 10.0, 12.0],
        vec![14.0, 16.0, 18.0]
    ])?;
    m.div_assign(2.0);
    assert!(m == expected, "Matrix divide by scalar & assign implemented incorrectly");
    Ok(())
}
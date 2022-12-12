extern crate matrix_lib;

use matrix_lib::{
    matrix::Matrix,
    errors::*,
};

#[test]
fn matrix_init_square() {
    let dim: usize = 4;
    let m = Matrix::new_square(dim, |_, _| 0.0);
    assert_eq!(m.rows(), m.cols(), "Matrix is not square")
}

#[test]
fn matrix_init_identity() {
    let matrix = Matrix::identity(5);
    assert_eq!(matrix.rows(), matrix.cols(), "Matrix isn't square");
    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            let value = matrix[i][j];
            if i == j {
                assert!(
                    f64::abs(value - 1.0) < f64::EPSILON,
                    "Diagonal element is not 1"
                );
            } else {
                assert!(
                    f64::abs(value) < f64::EPSILON,
                    "Non-diagonal element is not 0"
                );
            }
        }
    }
}

#[test]
fn matrix_init_diagonal() {
    let diagonal_vector = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let clone = diagonal_vector.clone();
    let matrix = Matrix::diagonal(&diagonal_vector);
    assert_eq!(
        matrix.rows(),
        clone.len(),
        "Incorrect rows count in diagonal matrix"
    );
    assert_eq!(
        matrix.cols(),
        clone.len(),
        "Incorrect cols count in diagonal matrix"
    );
    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            let value = matrix[i][j];
            if i == j {
                assert!(f64::abs(value - clone[i]) < f64::EPSILON, "Non-diagonal element is not equals to correspoing item of initializer vector");
            } else {
                assert!(
                    f64::abs(value) < f64::EPSILON,
                    "Non-diagonal element is not 0"
                );
            }
        }
    }
}

#[test]
fn matrix_utils_random() {
    let m = Matrix::random(50, 80);
    for i in 0..m.rows() {
        for j in 0..m.cols() {
            let v = m[i][j];
            assert!(v >= 0.0 && v < 1.0, "Incorrect initial random value")
        }
    }
}

#[test]
fn matrix_init_zero_matrix() {
    let matrix = Matrix::zero(2, 2);
    let zero_bits = 0.0f64.to_bits();
    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            assert_eq!(
                matrix[i][j].to_bits(),
                zero_bits,
                "Zero matrix contains non-zero elements"
            );
        }
    }
}

#[test]
fn matrix_init_from_vector() -> MathResult<()> {
    let v = vec![
        vec![1.0, 2.0, 3.0], 
        vec![2.0, 3.0, 4.0]
    ];
    let matrix = Matrix::from_vector(&v)?;
    assert_eq!(matrix.rows(), 2, "Incorrect rows count");
    assert_eq!(matrix.cols(), 3, "Incorrect cols count");
    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            assert_eq!(matrix[i][j], v[i][j], "Value of matrix doesn't correspond to original value");
        }
    }
    Ok(())
}

#[test]
fn matrix_init_from_incorrect_vector() -> MathResult<()> {
    let v = vec![
        vec![1.0, 2.0, 3.0], 
        vec![2.0, 3.0]
    ];
    assert_eq!(Matrix::from_vector(&v), Err(MathError::IncorrectVectorDimensions));
    Ok(())
}
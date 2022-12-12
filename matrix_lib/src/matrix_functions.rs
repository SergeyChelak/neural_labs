use super::{
    matrix::Matrix,
    matrix::map,
    errors::*,
};

impl Matrix {
    pub fn add(&self, other: &Matrix) -> MathResult<Self> {
        add(self, other)
    }

    pub fn sub(&self, other: &Matrix) -> MathResult<Self> {
        sub(self, other)
    }

    pub fn div(&self, other: &Matrix) -> MathResult<Self> {
        div(self, other)
    }

    pub fn product(&self, other: &Matrix) -> MathResult<Self> {
        product(self, other)
    }

    pub fn transpose(&self) -> Self {
        let (rows, cols) = (self.rows(), self.cols());
        Self::new(cols, rows, |i, j| {
            self.get_unchecked(j, i)
        })
    }

    pub fn powi(&self, power: i32) -> Self {
        Self::map(self, |x| x.powi(power))
    }
}

pub trait MatrixMultiplication<T> {
    type ResultType;
    fn mul(&self, value: T) -> Self::ResultType;
}

impl MatrixMultiplication<f64> for Matrix {
    type ResultType = Matrix;
    fn mul(&self, value: f64) -> Self::ResultType {
        self.map(|x| x * value)
    }
}

impl MatrixMultiplication<&Matrix> for Matrix {
    type ResultType = MathResult<Matrix>;
    fn mul(&self, value: &Matrix) -> Self::ResultType {
        mul(self, value)
    }
}

/// Elementwise sum
#[inline]
pub fn add(first: &Matrix, second: &Matrix) -> MathResult<Matrix> {
    map(first, second, |x, y| x + y)
}

/// Elementwise subtraction
#[inline]
pub fn sub(first: &Matrix, second: &Matrix) -> MathResult<Matrix> {
    map(first, second, |x, y| x - y)
}

/// Elementwise multiplication
#[inline]
pub fn mul(first: &Matrix, second: &Matrix) -> MathResult<Matrix> {
    map(first, second, |x, y| x * y)
}

/// Elementwise division
#[inline]
pub fn div(first: &Matrix, second: &Matrix) -> MathResult<Matrix> {
    map(first, second, |x, y| x / y)
}

/// Matrix product
pub fn product(first: &Matrix, second: &Matrix) -> MathResult<Matrix> {
    let (rows, fc) = (first.rows(), first.cols());
    let (sr, cols) = (second.rows(), second.cols());
    if fc == sr {
        let mut matrix = Matrix::zero(rows, cols);
        for i in 0..rows {            
            for j in 0..cols {
                let mut sum = 0.0f64;
                for k in 0..fc {
                    sum += first.get_unchecked(i, k) * second.get_unchecked(k, j);
                }
                matrix.set_unchecked(i, j, sum);
            }
        }
        Ok(matrix)
    } else {
        Err(MathError::IncorrectMatricesDimensions("product".to_string(), first.dimensions(), second.dimensions()))
    }        
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
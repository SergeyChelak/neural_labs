use super::matrix::Matrix;
use super::errors::*;

impl Matrix {

    pub fn add(first: &Matrix, second: &Matrix) -> MathResult<Self> {
        Self::element_wise(first, second, |x, y| x + y)
    }

    pub fn sub(first: &Matrix, second: &Matrix) -> MathResult<Self> {
        Self::element_wise(first, second, |x, y| x - y)
    }

    pub fn product(first: &Matrix, second: &Matrix) -> MathResult<Self> {
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

    pub fn transposed(&self) -> Matrix {
        let (rows, cols) = (self.rows(), self.cols());
        Matrix::new(cols, rows, |i, j| {
            self.get_unchecked(j, i)
        })
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_operation_plus() -> MathResult<()> {
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

        let sum = Matrix::add(&m1, &m2)?;
        assert!(sum == expected, "Matrix sum implemented incorrectly");
        Ok(())
    }

    #[test]
    fn matrix_operation_minus() -> MathResult<()> {
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

        let sum = Matrix::sub(&m1, &m2)?;
        assert!(sum == expected, "Matrix subtraction implemented incorrectly");
        Ok(())
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

        let ab = Matrix::product(&a, &b)?;
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

        let ab = Matrix::product(&a, &b)?;
        assert!(ab == expected, "Matrix product implemented incorrectly");

        let identity = Matrix::identity(a.cols());
        assert!(Matrix::product(&a, &identity)? == a, "Matrix product identity should return the same matrix");
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

        assert!(Matrix::product(&a, &b).is_err());
        Ok(())
    }

    #[test]
    fn matrix_operation_transposed() -> MathResult<()> {
        let a = Matrix::from_vector(&vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0]
        ])?;

        let expected = Matrix::from_vector(&vec![
            vec![1.0, 4.0],
            vec![2.0, 5.0],
            vec![3.0, 6.0]

        ])?;
        assert!(a.transposed() == expected, "Matrix transpose implemented incorrectly");
        Ok(())
    }

}
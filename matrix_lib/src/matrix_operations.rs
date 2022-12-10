use super::matrix::Matrix;

impl Matrix {
    pub fn mul_scalar(&self, scalar: f64) -> Self {
        self.map(|x| x * scalar)
    }

    pub fn transpose(&self) -> Matrix {
        let (rows, cols) = (self.rows(), self.cols());
        Matrix::new(cols, rows, |i, j| {
            self.get_unchecked(j, i)
        })
    }

    pub fn powi(&self, power: i32) -> Matrix {
        Self::map(self, |x| x.powi(power))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        *,
        super::errors::*,
    };

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

        assert!(m.mul_scalar(2.0) == expected, "Matrix scalar multiplication implemented incorrectly");
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
use super::matrix::Matrix;
use super::errors::*;

impl Matrix {
    // TODO: refactor this method and move in matrix.rs
    pub fn map<Func: Fn(f64) -> f64>(matrix: &Matrix, func: Func) -> Self {
        Self::new(matrix.rows(), matrix.cols(), |i, j| func(matrix.get_unchecked(i, j)))   
    }

    // TODO: refactor
    pub fn mul_scalar(matrix: &Matrix, scalar: f64) -> Self {
        Self::map(matrix, |x| x * scalar)
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

    pub fn plus_assign(&mut self, other: &Matrix) -> MathResult<()> {
        self.modify_other(other, |a, b| a + b)        
    }

    pub fn minus_assign(&mut self, other: &Matrix) -> MathResult<()> {
        self.modify_other(other, |a, b| a - b)
    }

    pub fn multiplicate_assign(&mut self, scalar: f64) {
        self.modify(|x| x * scalar)
    }

    pub fn divide_assign(&mut self, scalar: f64) {
        self.modify(|x| x / scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert!(Matrix::mul_scalar(&m, 2.0) == expected, "Matrix scalar multiplication implemented incorrectly");
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

        m1.plus_assign(&m2)?;
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

        m1.minus_assign(&m2)?;
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
        m.multiplicate_assign(2.0);
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
        m.divide_assign(2.0);
        assert!(m == expected, "Matrix divide by scalar & assign implemented incorrectly");
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
use super::matrix::Matrix;
use super::errors::*;

impl Matrix {
    pub fn add_mut(&mut self, other: &Matrix) -> MathResult<&mut Self> {
        self.element_wise_other(other, |a, b| a + b)        
    }

    pub fn sub_mut(&mut self, other: &Matrix) -> MathResult<&mut Self> {
        self.element_wise_other(other, |a, b| a - b)
    }

    pub fn mul_mut(&mut self, scalar: f64) -> &mut Self {
        self.element_wise_mut(|x| x * scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_operation_add() -> MathResult<()> {
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

        m1.add_mut(&m2)?;
        assert!(m1 == expected, "Matrix add implemeted incorrectly");
        Ok(())
    }

    #[test]
    fn matrix_operation_sub() -> MathResult<()> {
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

        m1.sub_mut(&m2)?;
        assert!(m1 == expected, "Matrix sub implemeted incorrectly");
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

        assert!(m.mul_mut(2.0) == &expected, "Matrix scalar multiplication implemented incorrectly");
        Ok(())
    }
}
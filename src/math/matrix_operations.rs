use super::matrix::Matrix;
use super::errors::*;

impl Matrix {
    pub fn add(first: &Matrix, second: &Matrix) -> MathResult<Self> {
        Self::element_wise(first, second, |x, y| x + y)
    }

    pub fn sub(first: &Matrix, second: &Matrix) -> MathResult<Self> {
        Self::element_wise(first, second, |x, y| x - y)
    }

    pub fn mul(first: &Matrix, second: &Matrix) -> MathResult<Self> {
        Self::element_wise(first, second, |x, y| x * y)
    }

    pub fn div(first: &Matrix, second: &Matrix) -> MathResult<Self> {
        Self::element_wise(first, second, |x, y| x / y)
    }

    pub fn map<Func: Fn(f64) -> f64>(matrix: &Matrix, func: Func) -> Self {
        Self::new(matrix.rows(), matrix.cols(), |i, j| func(matrix.get_unchecked(i, j)))   
    }

    pub fn mul_scalar(matrix: &Matrix, scalar: f64) -> Self {
        Self::map(matrix, |x| x * scalar)
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

    pub fn transpose(&self) -> Matrix {
        let (rows, cols) = (self.rows(), self.cols());
        Matrix::new(cols, rows, |i, j| {
            self.get_unchecked(j, i)
        })
    }

    pub fn plus_assign(&mut self, other: &Matrix) -> MathResult<&mut Self> {
        self.element_wise_other(other, |a, b| a + b)        
    }

    pub fn minus_assign(&mut self, other: &Matrix) -> MathResult<&mut Self> {
        self.element_wise_other(other, |a, b| a - b)
    }

    pub fn multiplicate_assign(&mut self, scalar: f64) -> &mut Self {
        self.map_assign(|x| x * scalar)
    }

    pub fn divide_assign(&mut self, scalar: f64) -> &mut Self {
        self.map_assign(|x| x / scalar)
    }

    pub fn powi(matrix: &Matrix, power: i32) -> Self {
        Self::new(matrix.rows(), matrix.cols(), |i, j| {
            f64::powi(matrix.get_unchecked(i, j), power)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_operation_add() -> MathResult<()> {
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
    fn matrix_operation_sub() -> MathResult<()> {
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

        assert!(m.multiplicate_assign(2.0) == &expected, "Matrix scalar multiplication & assign implemented incorrectly");
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

        assert!(m.divide_assign(2.0) == &expected, "Matrix divide by scalar & assign implemented incorrectly");
        Ok(())
    }

    #[test]
    fn matrix_operation_power_integer() -> MathResult<()> {
        let m = Matrix::from_vector(&vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ])?;

        let expected = Matrix::from_vector(&vec![
            vec![ 1.0,  4.0,  9.0],
            vec![16.0, 25.0, 36.0],
            vec![49.0, 64.0, 81.0]
        ])?;

        assert!(Matrix::powi(&m, 2) == expected, "Matrix power by scalar implemented incorrectly");
        Ok(())
    }

}
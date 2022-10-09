use super::matrix::Matrix;

impl Matrix {
    pub fn map<Func>(&mut self, f: Func) -> &mut Matrix where Func: Fn(usize, usize, f64) -> f64 {
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                let value = f(i, j, self.get(i, j));
                self.set(i, j, value);
            }
        }
        self
    }

    pub fn plus<'a>(first: &'a Matrix, second: &'a Matrix) -> Matrix {
        Self::new_with_binary_operation(first, second, |x, y| x + y)        
    }

    pub fn minus<'a>(first: &'a Matrix, second: &'a Matrix) -> Matrix {
        Self::new_with_binary_operation(first, second, |x, y| x - y)        
    }

    fn new_with_binary_operation<'a, Operation>(first: &'a Matrix, second: &'a Matrix, operation: Operation) -> Self where Operation : Fn(f64, f64) -> f64 {
        if !first.is_same_size(&second) {
            panic!("Matrices should be the same size")
        }
        let (rows, cols) = first.dimensions();
        Matrix::new(rows, cols, |i: usize, j: usize| {
            operation(first.get(i, j), second.get(i, j))
        })
    }

    pub fn add(&mut self, other: &Matrix) -> &mut Matrix {
        if !self.is_same_size(&other) {
            panic!("Can't sum matrices with different size")
        }
        self.map(|i, j, value| {
            value + other.get(i, j)
        })
    }

    pub fn sub(&mut self, other: &Matrix) -> &mut Matrix {
        if !self.is_same_size(&other) {
            panic!("Can't subtract matrices with different size")
        }
        self.map(|i, j, value| {
            value - other.get(i, j)
        })
    }

    pub fn product(first: &Matrix, second: &Matrix) -> Matrix {
        let (rows, fc) = first.dimensions();
        let (sr, cols) = second.dimensions();
        if fc != sr {
            panic!("Product operation is't applicable for matrices with dimensions {}x{} and {}x{}", rows, fc, sr, cols)
        }
        let mut matrix = Matrix::zero(rows, cols);
        for i in 0..rows {            
            for j in 0..cols {
                let mut sum = 0.0f64;
                for k in 0..fc {
                    sum += first.get(i, k) * second.get(k, j);
                }
                matrix.set(i, j, sum);
            }
        }
        matrix
    }

    pub fn transposed(&self) -> Matrix {
        let (rows, cols) = self.dimensions();
        Matrix::new(cols, rows, |i, j| {
            self.get(j, i)
        })
    }

    pub fn mul(&mut self, scalar: f64) -> &mut Matrix {
        self.map(|_, _, v| v * scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_operation_plus() {
        let m1 = Matrix::from_vector(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ]);

        let m2 = Matrix::from_vector(vec![
            vec![9.0, 8.0, 7.0],
            vec![6.0, 5.0, 4.0],
            vec![3.0, 2.0, 1.0]
        ]);

        let expected = Matrix::from_vector(vec![
            vec![10.0, 10.0, 10.0],
            vec![10.0, 10.0, 10.0],
            vec![10.0, 10.0, 10.0]
        ]);

        let sum = Matrix::plus(&m1, &m2);
        assert!(sum == expected, "Matrix plus implemeted incorrectly");
    }

    #[test]
    fn matrix_operation_add() {
        let mut m1 = Matrix::from_vector(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ]);

        let m2 = Matrix::from_vector(vec![
            vec![9.0, 8.0, 7.0],
            vec![6.0, 5.0, 4.0],
            vec![3.0, 2.0, 1.0]
        ]);

        let expected = Matrix::from_vector(vec![
            vec![10.0, 10.0, 10.0],
            vec![10.0, 10.0, 10.0],
            vec![10.0, 10.0, 10.0]
        ]);

        m1.add(&m2);
        assert!(m1 == expected, "Matrix add implemeted incorrectly")
    }

    #[test]
    fn matrix_operation_minus() {
        let m1 = Matrix::from_vector(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ]);

        let m2 = Matrix::from_vector(vec![
            vec![9.0, 8.0, 7.0],
            vec![6.0, 5.0, 4.0],
            vec![3.0, 2.0, 1.0]
        ]);

        let expected = Matrix::from_vector(vec![
            vec![-8.0, -6.0, -4.0],
            vec![-2.0,  0.0,  2.0],
            vec![ 4.0,  6.0,  8.0]
        ]);

        let sum = Matrix::minus(&m1, &m2);
        assert!(sum == expected, "Matrix minus implemeted incorrectly");
    }

    #[test]
    fn matrix_operation_sub() {
        let mut m1 = Matrix::from_vector(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ]);

        let m2 = Matrix::from_vector(vec![
            vec![9.0, 8.0, 7.0],
            vec![6.0, 5.0, 4.0],
            vec![3.0, 2.0, 1.0]
        ]);

        let expected = Matrix::from_vector(vec![
            vec![-8.0, -6.0, -4.0],
            vec![-2.0,  0.0,  2.0],
            vec![ 4.0,  6.0,  8.0]
        ]);

        m1.sub(&m2);
        assert!(m1 == expected, "Matrix sub implemeted incorrectly")
    }

    #[test]
    fn matrix_operation_product() {
        let a = Matrix::from_vector(vec![
            vec![1.0, 0.0, 1.0],
            vec![2.0, 1.0, 1.0],
            vec![0.0, 1.0, 1.0],
            vec![1.0, 1.0, 2.0]

        ]);

        let b = Matrix::from_vector(vec![
            vec![1.0, 2.0, 1.0],
            vec![2.0, 3.0, 1.0],
            vec![4.0, 2.0, 2.0]
        ]);

        let expected = Matrix::from_vector(vec![
            vec![ 5.0, 4.0, 3.0],
            vec![ 8.0, 9.0, 5.0],
            vec![ 6.0, 5.0, 3.0],
            vec![11.0, 9.0, 6.0]

        ]);

        let ab = Matrix::product(&a, &b);
        assert!(ab == expected, "Matrix product implemented incorrectly");

        let a = Matrix::from_vector(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],

        ]);

        let b = Matrix::from_vector(vec![
            vec![ 7.0,  8.0],
            vec![ 9.0, 10.0],
            vec![11.0, 12.0]
        ]);

        let expected = Matrix::from_vector(vec![
            vec![ 58.0,  64.0],
            vec![139.0, 154.0]

        ]);

        let ab = Matrix::product(&a, &b);
        assert!(ab == expected, "Matrix product implemented incorrectly");

        let identity = Matrix::identity(a.cols());
        assert!(Matrix::product(&a, &identity) == a, "Matrix product identity should return the same matrix");
    }

    #[test]
    #[should_panic]
    fn matrix_operation_product_failure() {
        let a = Matrix::from_vector(vec![
            vec![1.0, 0.0, 1.0],
            vec![2.0, 1.0, 1.0],
            vec![0.0, 1.0, 1.0],
            vec![1.0, 1.0, 2.0]

        ]);

        let b = Matrix::from_vector(vec![
            vec![1.0, 0.0, 1.0],
            vec![2.0, 1.0, 1.0],
            vec![0.0, 1.0, 1.0],
            vec![1.0, 1.0, 2.0],
            vec![1.0, 1.0, 2.0]
        ]);
        _ = Matrix::product(&a, &b);
    }

    #[test]
    fn matrix_operation_transposed() {
        let a = Matrix::from_vector(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0]
        ]);

        let expected = Matrix::from_vector(vec![
            vec![1.0, 4.0],
            vec![2.0, 5.0],
            vec![3.0, 6.0]

        ]);
        assert!(a.transposed() == expected, "Matrix transpose implemented incorrectly");
    }

    #[test]
    fn matrix_operation_scalar_multiplication() {
        let mut m = Matrix::from_vector(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ]);

        let expected = Matrix::from_vector(vec![
            vec![ 2.0,  4.0,  6.0],
            vec![ 8.0, 10.0, 12.0],
            vec![14.0, 16.0, 18.0]
        ]);

        assert!(m.mul(2.0) == &expected, "Matrix scalar multiplication implemented incorrectly");
    }
}
use super::matrix::Matrix;

impl Matrix {
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

    pub fn add(&mut self, other: &Matrix) {
        self.modify_with_binary_operation(other, |x, y| x + y)
    }

    pub fn sub(&mut self, other: &Matrix) {
        self.modify_with_binary_operation(other, |x, y| x - y)
    }

    fn modify_with_binary_operation<Operation>(&mut self, other: &Matrix, operation: Operation) where Operation : Fn(f64, f64) -> f64 {
        if !self.is_same_size(&other) {
            panic!("Matrices should be the same size")
        }
        let (rows, cols) = self.dimensions();
        for i in 0..rows {
            for j in 0..cols {
                let value = operation(self.get(i, j), other.get(i, j));
                self.set(i, j, value);
            }
        }
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
}
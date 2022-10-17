use super::matrix::Matrix;

impl Matrix {
    pub fn map_mut<Func>(&mut self, f: Func) -> &mut Matrix where Func: Fn(usize, usize, f64) -> f64 {
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                let value = f(i, j, self.get(i, j));
                self.set(i, j, value);
            }
        }
        self
    }

    pub fn add_mut(&mut self, other: &Matrix) -> &mut Matrix {
        if !self.is_same_size(&other) {
            panic!("Can't sum matrices with different size")
        }
        self.map_mut(|i, j, value| {
            value + other.get(i, j)
        })
    }

    pub fn sub_mut(&mut self, other: &Matrix) -> &mut Matrix {
        if !self.is_same_size(&other) {
            panic!("Can't subtract matrices with different size")
        }
        self.map_mut(|i, j, value| {
            value - other.get(i, j)
        })
    }

    pub fn mul_mut(&mut self, scalar: f64) -> &mut Matrix {
        self.map_mut(|_, _, v| v * scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_mutations_map() {
        panic!()
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

        m1.add_mut(&m2);
        assert!(m1 == expected, "Matrix add implemeted incorrectly")
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

        m1.sub_mut(&m2);
        assert!(m1 == expected, "Matrix sub implemeted incorrectly")
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

        assert!(m.mul_mut(2.0) == &expected, "Matrix scalar multiplication implemented incorrectly");
    }
}
use super::matrix::Matrix;

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.is_same_size(other) {
            let rows = self.rows();
            let cols = self.cols();
            for i in 0..rows {
                for j in 0..cols {
                    if f64::abs(self[i][j] - other[i][j]) > f64::EPSILON {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }
}

impl Eq for Matrix {
    //
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_eq() {
        let a = vec![
            vec![1.0, 2.0, 3.0], 
            vec![2.0, 3.0, 4.0]
        ];
        let b = a.clone();
        let m1 = Matrix::from_vector(&a);
        let m2 = Matrix::from_vector(&b);
        assert!(m1 == m2, "Matrices should be equal");

        let b = vec![
            vec![5.0, 2.0, 3.0], 
            vec![2.0, 3.0, 4.0]
        ];
        let m2 = Matrix::from_vector(&b);
        assert!(m1 != m2, "Matrices shouldn't be equal");


        let b = vec![
            vec![5.0, 2.0], 
            vec![2.0, 3.0]
        ];
        let m2 = Matrix::from_vector(&b);
        assert!(m1 != m2, "Matrices shouldn't be equal");
    }
}

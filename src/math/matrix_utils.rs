use super::matrix::Matrix;

impl Matrix {
    pub fn is_square(&self) -> bool {
        self.rows() == self.cols()
    }
    
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_utils_is_square() {
        let m = Matrix::zero(3, 5);
        assert!(!m.is_square(), "Matrix is not square");

        let m = Matrix::zero(5, 5);
        assert!(m.is_square(), "Matrix is not square");
    }

}
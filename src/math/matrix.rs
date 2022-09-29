pub struct Matrix {
    rows: usize,
    cols: usize,
    content: Vec<Vec<f64>>
}

type MatrixInitializer = fn(usize, usize) -> f64;

impl Matrix {
    pub fn new(rows: usize, cols: usize, initializer: MatrixInitializer) -> Self {
        let mut content = vec![vec![0.0f64; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                content[i][j] = initializer(i, j);
            }
        }
        Matrix {
            rows,
            cols,
            content
        }
    }

    pub fn new_square(dimension: usize, initializer: MatrixInitializer) -> Self {
        Self::new(dimension, dimension, initializer)
    }

    pub fn identity(dimension: usize) -> Self {
        Self::new_square(dimension, |row, col| {
            if row == col { 1.0 } else { 0.0 }
        })
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols, |_, _| { 0.0 })
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.content[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.content[row][col] = value;
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_zero_matrix() {
        let matrix = Matrix::zero(2, 2);
        let zero_bits = 0.0f64.to_bits();
        for i in 0..matrix.rows {
            for j in 0..matrix.cols {
                assert_eq!(matrix.get(i, j).to_bits(), zero_bits, "Zero matrix contains non-zero elements");
            }
        }
    }

    #[test]
    fn create_identity() {
        let matrix = Matrix::identity(5);
        assert_eq!(matrix.rows(), matrix.cols(), "Matrix isn't square");
        for i in 0..matrix.rows() {
            for j in 0..matrix.cols() {
                let value = matrix.get(i, j);
                if i == j {
                    assert!(f64::abs(value - 1.0) < f64::EPSILON, "Diagonal element is not 1");
                } else {
                    assert!(f64::abs(value) < f64::EPSILON, "Non-diagonal element is not 0");
                }
            }
        }
    }
}
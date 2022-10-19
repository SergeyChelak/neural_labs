use super::dimensions::Dimensions;

pub struct Matrix {
    dimensions: Dimensions,
    content: Vec<Vec<f64>>,
}

impl Matrix {
    // initializers
    pub fn new<P>(rows: usize, cols: usize, mut producer: P) -> Self where P: FnMut(usize, usize) -> f64 {
        let mut content = vec![vec![0.0f64; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                content[i][j] = producer(i, j);
            }
        }
        Matrix {
            dimensions: Dimensions::new(rows, cols),
            content: content,
        }
    }

    pub fn new_square<P>(dimension: usize, producer: P) -> Self where P: FnMut(usize, usize) -> f64 {
        Self::new(dimension, dimension, producer)
    }

    pub fn identity(dimension: usize) -> Self {
        Self::new_square(
            dimension,
            |row, col| if row == col { 1.0 } else { 0.0 }
        )
    }

    pub fn diagonal(vector: &Vec<f64>) -> Self {
        let dimension = vector.len();
        Self::new_square(
            dimension,
            |row, col| if row == col { vector[row] } else { 0.0 }
        )
    }

    pub fn from_vector(vector: Vec<Vec<f64>>) -> Self {
        if let Ok(dims) = Dimensions::from_vector(&vector) {
            return Matrix {
                dimensions: dims,
                content: vector 
            }
        } else {
            panic!("Initializer vector must be rectangular")
        }
    }

    pub fn from_scalar(scalar: f64) -> Self {
        Self::from_vector(vec![vec![scalar]])
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols, Box::new(|_, _| 0.0))
    }

    // properties and accessors
    pub fn rows(&self) -> usize {
        self.dimensions.rows()
    }

    pub fn cols(&self) -> usize {
        self.dimensions.cols()
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.content[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.content[row][col] = value;
    }
    
    pub fn is_same_size(&self, other: &Matrix) -> bool {
        self.dimensions == other.dimensions
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_init_new() {
        let (rows, cols) = (4usize, 8usize);
        let m = Matrix::new(rows, cols, |i, j| (i * rows + j) as f64);
        assert!(m.rows() == rows, "Matrix initialized with incorrect row value");
        assert!(m.cols() == cols, "Matrix initialized with incorrect column value");
        for i in 0..rows {
            for j in 0..cols {
                let value = (i * rows + j) as f64;
                assert_eq!(m.get(i, j), value, "Unexpected matrix value");
            }
        }
    }

    #[test]
    fn matrix_init_square() {
        let dim: usize = 4;
        let m = Matrix::new_square(dim, |_, _| 0.0);
        assert_eq!(m.rows(), m.cols(), "Matrix is not square")
    }

    #[test]
    fn matrix_init_zero_matrix() {
        let matrix = Matrix::zero(2, 2);
        let zero_bits = 0.0f64.to_bits();
        for i in 0..matrix.rows() {
            for j in 0..matrix.cols() {
                assert_eq!(
                    matrix.get(i, j).to_bits(),
                    zero_bits,
                    "Zero matrix contains non-zero elements"
                );
            }
        }
    }

    #[test]
    fn matrix_init_identity() {
        let matrix = Matrix::identity(5);
        assert_eq!(matrix.rows(), matrix.cols(), "Matrix isn't square");
        for i in 0..matrix.rows() {
            for j in 0..matrix.cols() {
                let value = matrix.get(i, j);
                if i == j {
                    assert!(
                        f64::abs(value - 1.0) < f64::EPSILON,
                        "Diagonal element is not 1"
                    );
                } else {
                    assert!(
                        f64::abs(value) < f64::EPSILON,
                        "Non-diagonal element is not 0"
                    );
                }
            }
        }
    }

    #[test]
    fn matrix_init_diagonal() {
        let diagonal_vector = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let clone = diagonal_vector.clone();
        let matrix = Matrix::diagonal(&diagonal_vector);
        assert_eq!(
            matrix.rows(),
            clone.len(),
            "Incorrect rows count in diagonal matrix"
        );
        assert_eq!(
            matrix.cols(),
            clone.len(),
            "Incorrect cols count in diagonal matrix"
        );
        for i in 0..matrix.rows() {
            for j in 0..matrix.cols() {
                let value = matrix.get(i, j);
                if i == j {
                    assert!(f64::abs(value - clone[i]) < f64::EPSILON, "Non-diagonal element is not equals to correspoing item of initializer vector");
                } else {
                    assert!(
                        f64::abs(value) < f64::EPSILON,
                        "Non-diagonal element is not 0"
                    );
                }
            }
        }
    }

    #[test]
    fn matrix_init_from_vector() {
        let v = vec![
            vec![1.0, 2.0, 3.0], 
            vec![2.0, 3.0, 4.0]
        ];
        let clone = v.clone();
        let matrix = Matrix::from_vector(v);
        assert_eq!(matrix.rows(), 2, "Incorrect rows count");
        assert_eq!(matrix.cols(), 3, "Incorrect cols count");
        for i in 0..matrix.rows() {
            for j in 0..matrix.cols() {
                assert_eq!(matrix.get(i, j), clone[i][j], "Value of matrix doesn't respond to original value");
            }
        }
    }

    #[test]
    #[should_panic]
    fn matrix_init_from_incorrect_vector() {
        let v = vec![
            vec![1.0, 2.0, 3.0], 
            vec![2.0, 3.0]
        ];
        _ = Matrix::from_vector(v);
    }
}

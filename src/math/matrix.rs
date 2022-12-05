use std::vec;

use super::dimensions::Dimensions;
use super::errors::*;

#[derive(Clone)]
pub struct Matrix {
    dimensions: Dimensions,
    content: Vec<f64>,
}

impl Matrix {
    // initializers
    pub fn new<P>(rows: usize, cols: usize, mut producer: P) -> Self where P: FnMut(usize, usize) -> f64 {
        let mut content = vec![0.0f64; rows * cols];
        for i in 0..rows {
            for j in 0..cols {
                content[i * cols + j] = producer(i, j);
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

    pub fn from_vector(vector: &Vec<Vec<f64>>) -> MathResult<Self> {
        let dims = Dimensions::from_vector(vector)?;
        Ok(Matrix::new(dims.rows(), dims.cols(), |i, j| vector[i][j]))
    }

    pub fn create_vector(vector: &Vec<f64>) -> MathResult<Self> {
        let dims = Dimensions::new(vector.len(), 1);
        Ok(Matrix::new(dims.rows(), dims.cols(), |i, _| vector[i]))
    }

    pub fn from_scalar(scalar: f64) -> MathResult<Self> {
        Self::from_vector(&vec![vec![scalar]])
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols, |_, _| 0.0)
    }

    // properties and accessors
    #[inline(always)]
    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    #[inline(always)]
    pub fn rows(&self) -> usize {
        self.dimensions.rows
    }

    #[inline(always)]
    pub fn cols(&self) -> usize {
        self.dimensions.cols
    }

    pub fn get(&self, row: usize, col: usize) -> MathResult<f64> {
        if self.dimensions.is_valid_position(row, col) {
            Ok(self.get_unchecked(row, col))
        } else {
            Err(MathError::IncorrectPosition(row, col))
        }
    }

    #[inline(always)]
    pub fn get_unchecked(&self, row: usize, col: usize) -> f64 {        
        let pos = self.position(row, col);
        self.content[pos]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) -> MathResult<()> {
        if self.dimensions.is_valid_position(row, col) {
            self.set_unchecked(row, col, value);
            Ok(())
        } else {
            Err(MathError::IncorrectPosition(row, col))
        }
    }

    #[inline(always)]
    pub fn set_unchecked(&mut self, row: usize, col: usize, value: f64) {
        let pos = self.position(row, col);
        self.content[pos] = value;
    }
    
    #[inline(always)]
    pub fn is_same_size(&self, other: &Matrix) -> bool {
        self.dimensions == other.dimensions
    }

    #[inline(always)]
    fn position(&self, row: usize, col: usize) -> usize {
        row * self.dimensions.cols + col
    }

    pub fn element_wise<Op: Fn(f64, f64) -> f64>(a: &Matrix, b: &Matrix, operation: Op) -> MathResult<Matrix> {
        if a.is_same_size(&b) {
            let size = a.content.len();
            let mut vector = vec![0.0; size];
            for i in 0..size {
                vector[i] = operation(a.content[i], b.content[i]);
            }
            Ok(
                Matrix { 
                    dimensions: a.dimensions, 
                    content: vector 
                }
            )
        } else {
            Err(MathError::IncorrectMatricesDimensions("element wise".to_string(), a.dimensions, b.dimensions))
        }
    }

    pub fn element_wise_other<Op: Fn(f64, f64) -> f64>(&mut self, other: &Matrix, operation: Op) -> MathResult<&mut Self> {
        if self.is_same_size(other) {
            for i in 0..self.content.len() {
                self.content[i] = operation(self.content[i], other.content[i]);
            }
            Ok(self)
        } else {
            Err(MathError::IncorrectMatricesDimensions("element wise mut".to_string(), self.dimensions, other.dimensions))
        }        
    }

    pub fn map_assign<Op: Fn(f64) -> f64>(&mut self, operation: Op) -> &mut Self {
        for i in 0..self.content.len() {
            self.content[i] = operation(self.content[i]);
        }
        self 
    }

    pub fn mean(&self) -> f64 {
        self.content.iter().fold(0.0, |acc, v| acc + v) / self.content.len() as f64
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        let cols = self.cols();
        let pos = index * cols;
        &self.content[pos..pos + cols]
    }
}

impl std::ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let cols = self.cols();
        let pos = index * cols;
        self.content[pos..pos + cols].as_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::errors::MathError;

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
                assert_eq!(m[i][j], value, "Unexpected matrix value");
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
                    matrix[i][j].to_bits(),
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
                let value = matrix[i][j];
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
                let value = matrix[i][j];
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
    fn matrix_init_from_vector() -> MathResult<()> {
        let v = vec![
            vec![1.0, 2.0, 3.0], 
            vec![2.0, 3.0, 4.0]
        ];
        let matrix = Matrix::from_vector(&v)?;
        assert_eq!(matrix.rows(), 2, "Incorrect rows count");
        assert_eq!(matrix.cols(), 3, "Incorrect cols count");
        for i in 0..matrix.rows() {
            for j in 0..matrix.cols() {
                assert_eq!(matrix[i][j], v[i][j], "Value of matrix doesn't correspond to original value");
            }
        }
        Ok(())
    }

    #[test]
    fn matrix_init_from_incorrect_vector() -> MathResult<()> {
        let v = vec![
            vec![1.0, 2.0, 3.0], 
            vec![2.0, 3.0]
        ];
        assert_eq!(Matrix::from_vector(&v), Err(MathError::IncorrectVectorDimensions));
        Ok(())
    }

    #[test]
    fn matrix_access_get_set() -> Result<(), MathError> {
        let vector = vec![
            vec![1.2, 2.3, 3.4, 6.1],
            vec![4.5, 5.6, 6.7, 5.2],
            vec![7.8, 8.9, 9.0, 4.3],
        ];
        let dims = Dimensions::from_vector(&vector)?;
        let mut m = Matrix::zero(dims.rows(), dims.cols());
        // 
        for i in 0..dims.rows() {
            for j in 0..dims.cols() {
                m.set(i, j, vector[i][j])?;
            }
        }
        // 
        for i in 0..dims.rows() {
            for j in 0..dims.cols() {
                assert_eq!(m[i][j], vector[i][j]);
            }
        }
        Ok(())
    }

    #[test]
    fn matrix_access_out_of_bounds() {
        let m = Matrix::random(5, 8);
        assert!(m.get(5, 0).is_err());
        assert!(m.get(0, 8).is_err());
        assert!(m.get(5, 8).is_err());
        assert!(m.get(5, 9).is_err());
        assert!(m.get(6, 9).is_err());
    }

    #[test]
    fn matrix_access_index_trait() -> MathResult<()> {
        let sizes: [(usize, usize); 6] = [
            (0, 0),
            (1, 0),
            (1, 1),
            (1, 5),
            (5, 1),
            (7, 11)
        ];
        for (rows, cols) in sizes {
            let m = Matrix::random(rows, cols);
            for i in 0..m.rows() {
                for j in 0..m.cols() {                
                    assert_eq!(m[i][j], m.get(i, j)?, "Incorrect value via indexing at {}:{} for dim {:?}", i, j, m.dimensions);
                }
            }
        }   
        Ok(())
    }

    #[test]
    fn matrix_access_index_mut_trait() -> MathResult<()> {
        let vector = vec![
            vec![1.2, 2.3, 3.4, 6.1],
            vec![4.5, 5.6, 6.7, 5.2],
            vec![7.8, 8.9, 9.0, 4.3],
        ];
        let dims = Dimensions::from_vector(&vector)?;
        let mut m = Matrix::zero(dims.rows(), dims.cols());

        for i in 0..m.rows() {
            for j in 0..m.cols() {
                m[i][j] = vector[i][j];
            }
        }

        for i in 0..m.rows() {
            for j in 0..m.cols() {
                assert_eq!(m.get(i, j)?, vector[i][j], "Value at {}:{} was written incorrectly", i, j);
            }
        }

        Ok(())
    }

    #[test]
    fn matrix_clone() -> MathResult<()> {
        let v = vec![
            vec![1.0, 2.0, 3.0], 
            vec![2.0, 3.0, 4.0]
        ];
        let m1 = Matrix::from_vector(&v)?;
        let m2 = m1.clone();
        assert_eq!(m1, m2, "Matrices should be equal after clone");
        Ok(())
    }

    #[test]
    fn matrix_mean() -> MathResult<()> {
        let m = Matrix::from_vector(&vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
        ])?;
        let mean = m.mean();
        assert!(f64::abs(mean - 2.5) < f64::EPSILON, "Matrix mean implemented incorrectly. Value {}", mean);
        Ok(())
    }
}
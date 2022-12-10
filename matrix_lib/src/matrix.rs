use std::vec;

use super::dimensions::Dimensions;
use super::errors::*;

#[derive(Clone)]
pub struct Matrix {
    dimensions: Dimensions,
    content: Vec<f64>,
}

impl Matrix {
    /// primary initializer
    pub fn new<P>(rows: usize, cols: usize, mut producer: P) -> Self where P: FnMut(usize, usize) -> f64 {
        let mut content = vec![0.0f64; rows * cols];
        for i in 0..rows {
            for j in 0..cols {
                content[i * cols + j] = producer(i, j);
            }
        }
        Matrix {
            dimensions: Dimensions::new(rows, cols),
            content,
        }
    }

    pub fn vector(vector: &Vec<f64>) -> MathResult<Self> {
        let dims = Dimensions::new(vector.len(), 1);
        Ok(Matrix::new(dims.rows(), dims.cols(), |i, _| vector[i]))
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

    pub fn modify_other<Op: Fn(f64, f64) -> f64>(&mut self, other: &Matrix, operation: Op) -> MathResult<()> {
        if self.is_same_size(other) {
            for i in 0..self.content.len() {
                self.content[i] = operation(self.content[i], other.content[i]);
            }
            Ok(())
        } else {
            Err(MathError::IncorrectMatricesDimensions("element wise mut".to_string(), self.dimensions, other.dimensions))
        }        
    }

    pub fn modify<Op: Fn(f64) -> f64>(&mut self, operation: Op) {
        for i in 0..self.content.len() {
            self.content[i] = operation(self.content[i]);
        }
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

pub fn map<Op: Fn(f64, f64) -> f64>(a: &Matrix, b: &Matrix, operation: Op) -> MathResult<Matrix> {
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
                assert_eq!(m[i][j], value, "Unexpected matrix value");
            }
        }
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
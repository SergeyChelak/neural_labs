use super::{
    dimensions::Dimensions,
    matrix::Matrix,
    errors::*,
};
use rand::Rng;

impl Matrix {  
    /// convenience initializers
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

    pub fn random(rows: usize, cols: usize) -> Self {
        let mut rng = rand::thread_rng();
        Matrix::new(rows, cols, |_, _| rng.gen::<f64>())
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols, |_, _| 0.0)
    }

    pub fn empty() -> Self {
        Matrix::zero(0, 0)
    }

    pub fn from_vector(vector: &Vec<Vec<f64>>) -> MathResult<Self> {
        let dims = Dimensions::from_vector(vector)?;
        Ok(Matrix::new(dims.rows(), dims.cols(), |i, j| vector[i][j]))
    }

    pub fn from_scalar(scalar: f64) -> MathResult<Self> {
        Self::from_vector(&vec![vec![scalar]])
    }
}

pub fn random_matrices(amount: usize, dimensions: Dimensions) -> Vec<Matrix> {
    let mut result: Vec<Matrix> = Vec::with_capacity(amount);
    for _ in 0..amount {
        let item = Matrix::random(dimensions.rows(), dimensions.cols());
        result.push(item);
    }
    result
}
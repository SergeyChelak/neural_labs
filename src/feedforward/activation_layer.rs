use crate::math::{errors::MathResult, matrix::*};
use super::layer::*;

struct Activation {
    activation: fn(f64) -> f64,
    activation_prime: fn(f64) -> f64,
    input: Matrix
}

impl Layer for Activation {
    fn forward(&mut self, input: Matrix) -> MathResult<Matrix> {
        let matrix = Matrix::map(&input, self.activation);
        self.input = input;
        Ok(matrix)
    }

    fn backward(&mut self, output_gradient: &Matrix, learning_rate: f64) -> MathResult<Matrix> {
        let matrix = Matrix::map(&self.input, self.activation_prime);
        Matrix::mul(output_gradient, &matrix)
    }
}
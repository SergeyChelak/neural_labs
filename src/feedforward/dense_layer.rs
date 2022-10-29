use crate::math::{errors::MathResult, matrix::*};
use super::layer::*;

pub struct Dense {
    weight: Matrix,
    bias: Matrix,
    input: Matrix
}

impl Dense {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        let weight = Matrix::random(output_size, input_size);
        let bias = Matrix::random(output_size, 1);
        Self {
            weight,
            bias,
            input: Matrix::empty()
        }
    } 
}

impl Layer for Dense {
    fn forward(&mut self, input: Matrix) -> MathResult<Matrix> {
        self.input = input;
        Matrix::product(&self.weight, &self.input)
    }

    fn backward(&mut self, output_gradient: &Matrix, learning_rate: f64) -> MathResult<Matrix> {
        let weight_gradient = Matrix::product(output_gradient, &self.input.transpose())?;
        _ = self.weight.minus_assign(
            &Matrix::mul_scalar(&weight_gradient, learning_rate)
        );

        _ = self.bias.minus_assign(
            &Matrix::mul_scalar(&output_gradient, learning_rate)
        );

        Matrix::product(&self.weight.transpose(), output_gradient)
    }
}
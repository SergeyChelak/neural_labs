use super::layer::*;
use matrix_lib::{
    errors::*,
    matrix::Matrix,
};

pub struct Convolution {
    //
}

impl Convolution {
    pub fn new(input_shape: usize, kernel_size: usize, depth: usize) -> Self {
        Self {}
    }
}

impl Layer for Convolution {
    fn eval(&self, input: &Matrix) -> MathResult<Matrix> {
        todo!()
    }

    fn forward(&mut self, input: Matrix) -> MathResult<Matrix> {
        todo!()
    }

    fn backward(&mut self, output_gradient: &Matrix, learning_rate: f64) -> MathResult<Matrix> {
        todo!()
    }
}
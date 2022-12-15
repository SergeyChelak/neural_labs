use std::vec;

use super::layer::*;
use matrix_lib::{
    dimensions::Dimensions, 
    errors::*,
    matrix::Matrix,
};

pub struct Convolution {
    input_depth: usize,
    kernels: Vec<Matrix>,
    biases: Vec<Matrix>,
    input: Matrix,
}

impl Convolution {
    pub fn new(input_dimensions: Dimensions, input_depth: usize, kernel_dimension: usize, kernel_depth: usize) -> Self {
        Self {
            input_depth,
            kernels: vec![],
            biases: vec![],
            input: Matrix::empty(),
        }
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
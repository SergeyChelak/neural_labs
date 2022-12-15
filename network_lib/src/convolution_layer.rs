use super::layer::*;
use matrix_lib::{
    dimensions::Dimensions, 
    errors::*,
    matrix::Matrix, matrix_convenience::random_matrices,
};

pub struct Convolution {
    input_depth: usize,
    kernels: Vec<Matrix>,
    biases: Vec<Matrix>,
    input: Matrix,
}

impl Convolution {
    pub fn new(input_dimensions: Dimensions, input_depth: usize, kernel_size: usize, depth: usize) -> Self {
        let kernel_dimension = Dimensions::square(kernel_size);
        let output_dimension = Dimensions::new(
            input_dimensions.rows() - kernel_size + 1, 
            input_dimensions.cols() - kernel_size + 1
        );
        Self {
            input_depth,
            kernels: random_matrices(depth, kernel_dimension),
            biases: random_matrices(depth, output_dimension),
            input: Matrix::empty(),
        }
    }
}

impl Layer for Convolution {
    fn eval(&self, input: &Matrix) -> MathResult<Matrix> {
        todo!()
    }

    fn forward(&mut self, input: Matrix) -> MathResult<Matrix> {
        self.input = input;
        self.eval(&self.input)
    }

    fn backward(&mut self, output_gradient: &Matrix, learning_rate: f64) -> MathResult<Matrix> {
        todo!()
    }
}
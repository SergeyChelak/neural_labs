use super::layer::*;
use matrix_lib::{
    dimensions::Dimensions, 
    errors::*,
    matrix::Matrix, 
    matrix_convenience::*,
};

pub struct Convolution {
    input_depth: usize,
    depth: usize,
    kernels: Vec<Vec<Matrix>>,
    biases: Vec<Matrix>,
    input: Vec<Matrix>,
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
            depth,
            kernels: random_matrices2d(Dimensions::new(depth, input_depth), kernel_dimension),
            biases: random_matrices(depth, output_dimension),
            input: vec![],
        }
    }
// }
//
// TODO: make layer with generic input parameter... 
// impl Layer for Convolution {
    fn eval(&self, input: &Vec<Matrix>) -> MathResult<Vec<Matrix>> {
        let mut output = self.biases.clone();
        for i in 0..self.depth {
            for j in 0..self.input_depth {
                output[i] += correlate2d(&self.input[j], &self.kernels[i][j]);
            }
        }
        Ok(output)
    }

    fn forward(&mut self, input: Vec<Matrix>) -> MathResult<Vec<Matrix>> {
        self.input = input;
        self.eval(&self.input)
    }

    fn backward(&mut self, output_gradient: &Matrix, learning_rate: f64) -> MathResult<Matrix> {
        todo!()
    }
}

fn correlate2d(input: &Matrix, kernel: &Matrix) -> Matrix {
    todo!()
}
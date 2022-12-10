use matrix_lib::{
    errors::MathResult, 
    matrix::*
};

pub trait Layer {
    fn eval(&self, input: &Matrix) -> MathResult<Matrix>;

    fn forward(&mut self, input: Matrix) -> MathResult<Matrix>;

    fn backward(&mut self, output_gradient: &Matrix, learning_rate: f64) -> MathResult<Matrix>;
}
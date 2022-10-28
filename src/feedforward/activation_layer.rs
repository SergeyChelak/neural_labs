use crate::math::{errors::MathResult, matrix::*};
use super::layer::*;

struct Activation {
    // TODO: add function and prime
}

impl Layer for Activation {
    fn forward(&mut self, input: Matrix) -> MathResult<Matrix> {
        todo!()
    }

    fn backward(&mut self, output_gradient: &Matrix, learning_rate: f64) -> MathResult<Matrix> {
        todo!()
    }
}
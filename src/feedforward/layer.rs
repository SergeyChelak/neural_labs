use crate::math::matrix::*;

pub trait Layer {
    fn forward(&self, input: &Matrix) -> Matrix;

    fn backward(&mut self, output_gradient: &Matrix, learning_rate: f64);
}
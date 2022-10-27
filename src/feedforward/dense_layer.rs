use crate::math::matrix::*;
use super::layer::*;

struct Dense {
    weight: Matrix,
    bias: Matrix,
}

impl Dense {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        let weight = Matrix::random(output_size, input_size);
        let bias = Matrix::random(output_size, 1);
        Self {
            weight,
            bias,
        }
    } 
}

impl Layer for Dense {
    fn forward(&self, input: &Matrix) -> Matrix {
        // self.input = input ???
        Matrix::product(&self.weight, input).unwrap()
    }

    fn backward(&mut self, output_gradient: &Matrix, learning_rate: f64) {
        todo!()
    }
}
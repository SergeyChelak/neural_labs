use matrix_lib::{
    errors::MathResult, 
    matrix::*,
    matrix_functions::*,
};
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
    fn eval(&self, input: &Matrix) -> MathResult<Matrix> {
        let mut prod = product(&self.weight, &input)?;
        prod += &self.bias;
        Ok(prod)
    }

    fn forward(&mut self, input: Matrix) -> MathResult<Matrix> {
        self.input = input;
        self.eval(&self.input)
    }

    fn backward(&mut self, output_gradient: &Matrix, learning_rate: f64) -> MathResult<Matrix> {
        let weight_gradient = product(output_gradient, &self.input.transpose())?;
        self.weight.sub_assign(&weight_gradient.mul(learning_rate))?;
        self.bias.sub_assign(&output_gradient.mul(learning_rate))?;
        self.weight.transpose().product(output_gradient)
    }
}
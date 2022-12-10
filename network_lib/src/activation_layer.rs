use matrix_lib::{
    errors::MathResult, 
    matrix::*,
    matrix_functions::mul,
};
use super::layer::*;

pub struct Activation {
    activation: fn(f64) -> f64,
    activation_prime: fn(f64) -> f64,
    input: Matrix
}

impl Activation {
    pub fn tanh() -> Self {
        Self {
            activation: f64::tanh,
            activation_prime: |x| 1.0 - x.tanh().powi(2),
            input: Matrix::empty()
        }
    }

    pub fn sigmoid() -> Self {
        Self {
            activation: self::sigmoid,
            activation_prime: |x| {
                let s = sigmoid(x);
                s * (1.0 - s)
            },
            input: Matrix::empty(),
        }
    }
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-x))
}

impl Layer for Activation {
    fn eval(&self, input: &Matrix) -> MathResult<Matrix> {
        Ok(
            input.map(self.activation)
        )
    }

    fn forward(&mut self, input: Matrix) -> MathResult<Matrix> {
        let result = self.eval(&input);
        self.input = input;
        result
    }

    fn backward(&mut self, output_gradient: &Matrix, _learning_rate: f64) -> MathResult<Matrix> {
        let matrix = self.input.map(self.activation_prime);
        output_gradient.mul(&matrix)
    }
}
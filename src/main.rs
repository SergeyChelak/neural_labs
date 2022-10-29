mod math;
mod feedforward;

use feedforward::{dense_layer::Dense, activation_layer::Activation, layer::Layer};
use math::{errors::MathResult, matrix::Matrix};

pub struct FeedforwardNetwork {
    layers: Vec<Box<dyn Layer>>,
}

impl FeedforwardNetwork {
    pub fn new(layers: Vec<Box<dyn Layer>>) -> Self {
        Self {
            layers,
        }
    }

    // TODO: implement as function call
    pub fn eval(&self, input: Matrix) -> MathResult<Matrix> {
        let mut output = input;
        for layer in self.layers.iter() {
            output = layer.eval(&output)?;
        }
        Ok(output)
    }

    pub fn predict(&mut self, input: Matrix) -> MathResult<Matrix> {
        let mut output = input;
        for layer in self.layers.iter_mut() {
            output = layer.forward(output)?;
        }
        Ok(output)
    }

    pub fn train(&mut self) {
        // TODO: implement
    }
}

fn main() {

    // xor training
    let network = FeedforwardNetwork::new(vec![
        Box::new(Dense::new(2, 3)),
        Box::new(Activation::tanh()),
        Box::new(Dense::new(3, 1)),
        Box::new(Activation::tanh()),
    ]);

    println!("Neural Labs, Ukraine 2022")
}

use super::{
    layer::Layer,
    data_source::TrainDataSource,
};
use matrix_lib::{
    matrix::Matrix,
    matrix_functions::*,
    errors::MathResult,
};

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
    pub fn eval(&self, input: &Matrix) -> MathResult<Matrix> {
        let mut output = input.clone();
        for layer in self.layers.iter() {
            output = layer.eval(&output)?;
        }
        Ok(output)
    }

    pub fn train(&mut self, epochs: usize, learning_rate: f64, data_source: &TrainDataSource) -> MathResult<f64> {
        let mut global_error = f64::NAN;
        for _ in 0..epochs {
            let mut error: f64 = 0.0;
            let data = data_source.content();
            for item in data {
                let mut output = item.input.clone();
                for layer in self.layers.iter_mut() {
                    output = layer.forward(output)?;
                }
                error += mse(&item.output, &output)?;
                let mut grad = mse_prime(&item.output, &output)?;
                for layer in self.layers.iter_mut().rev() {
                    grad = layer.backward(&grad, learning_rate)?;
                }
                error /= item.input.dimensions().size() as f64;
            }
            global_error = error;
        }
        Ok(global_error)
    }
}

fn mse(a: &Matrix, b: &Matrix) -> MathResult<f64> {
    Ok(
        sub(a, b)?
            .powi(2)
            .mean()
    )
}

fn mse_prime(a: &Matrix, b: &Matrix) -> MathResult<Matrix> {
    let mut tmp = b.sub(a)?;
    let dim = tmp.dimensions();
    tmp *= 2.0 / dim.size() as f64;
    Ok(tmp)
}

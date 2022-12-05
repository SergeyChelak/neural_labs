mod math;
mod feedforward;

use feedforward::{dense_layer::Dense, activation_layer::Activation, layer::Layer};
use math::{errors::*, matrix::Matrix};

pub struct TrainItem {
    input: Matrix,
    output: Matrix,
}

pub struct TrainDataSource {
    data: Vec<TrainItem>
}

impl TrainDataSource {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, input: Matrix, output: Matrix) {
        let item = TrainItem {
            input,
            output
        };
        self.data.push(item);
    }

    pub fn content(&self) -> &Vec<TrainItem> {
        &self.data
    }
}

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

    pub fn train(&mut self, epochs: usize, learning_rate: f64, data_source: &TrainDataSource) -> MathResult<()>{
        for epoch in 0..epochs {
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
                error /= item.input.dimensions().rows() as f64;
            }
            println!("{} of {}, error = {}", epoch + 1, epochs, error);
        }
        Ok(())
    }
}

fn mse(a: &Matrix, b: &Matrix) -> MathResult<f64> {
    let tmp = Matrix::element_wise(a, b, |x, y| {
        let r = x - y;
        r * r
    })?;
    Ok(tmp.mean())
}

fn mse_prime(a: &Matrix, b: &Matrix) -> MathResult<Matrix> {
    let mut tmp = Matrix::sub(b, a)?;
    let dim = tmp.dimensions();
    tmp.multiplicate_assign(2.0 / (dim.rows() * dim.cols()) as f64);
    Ok(tmp)
}

fn xor_data_source() -> MathResult<TrainDataSource> {
    let mut data = TrainDataSource::new();
    data.push(Matrix::create_vector(&vec![0.0, 0.0])?, Matrix::create_vector(&vec![0.0])?);
    data.push(Matrix::create_vector(&vec![1.0, 0.0])?, Matrix::create_vector(&vec![1.0])?);
    data.push(Matrix::create_vector(&vec![0.0, 1.0])?, Matrix::create_vector(&vec![1.0])?);
    data.push(Matrix::create_vector(&vec![1.0, 1.0])?, Matrix::create_vector(&vec![0.0])?);
    Ok(data)
}

fn main() -> MathResult<()> {

    // xor training
    let mut network = FeedforwardNetwork::new(vec![
        Box::new(Dense::new(2, 3)),
        Box::new(Activation::sigmoid()),
        Box::new(Dense::new(3, 1)),
        Box::new(Activation::sigmoid()),
    ]);

    let data_source = xor_data_source()?;
    _ = network.train(10_000, 0.01, &data_source);

    for item in data_source.content() {
        let input = &item.input;
        let eval = network.eval(input);
        println!("case {:?}, eval = {:?}, expected = {:?}", input, eval, item.output);
    }
    
    Ok(())
}

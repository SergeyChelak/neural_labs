use network_lib::{
    dense_layer::Dense,
    activation_layer::Activation, 
    network::FeedforwardNetwork,
    data_source::TrainDataSource
};
use matrix_lib::{
    errors::MathResult, 
    matrix::Matrix
};

fn xor_data_source() -> MathResult<TrainDataSource> {
    let mut data = TrainDataSource::new();
    data.push(Matrix::vector(&vec![0.0, 0.0])?, Matrix::vector(&vec![0.0])?);
    data.push(Matrix::vector(&vec![1.0, 0.0])?, Matrix::vector(&vec![1.0])?);
    data.push(Matrix::vector(&vec![0.0, 1.0])?, Matrix::vector(&vec![1.0])?);
    data.push(Matrix::vector(&vec![1.0, 1.0])?, Matrix::vector(&vec![0.0])?);
    Ok(data)
}

fn main() -> MathResult<()> {
    // xor training
    let mut network = FeedforwardNetwork::new(vec![
        Box::new(Dense::new(2, 3)),
        Box::new(Activation::tanh()),
        Box::new(Dense::new(3, 1)),
        Box::new(Activation::tanh()),
    ]);
    let data_source = xor_data_source()?;
    _ = network.train(10_000, 0.1, &data_source);
    for item in data_source.content() {
        let input = &item.input;
        let eval = network.eval(input);
        println!("case {:?}, eval = {:?}, expected = {:?}", input, eval, item.output);
    } 
    Ok(())
}

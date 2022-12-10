pub mod layer;
pub mod dense_layer;
pub mod activation_layer;
pub mod data_source;
pub mod network;

// TODO: make as integration test
#[cfg(test)]
mod tests {
    use super::{
        dense_layer::Dense,
        activation_layer::Activation, 
        network::FeedforwardNetwork,
        data_source::TrainDataSource
    };

    use matrix_lib::{
        errors::MathResult, 
        matrix::Matrix,
        matrix_functions::sub,
    };
    
    fn xor_data_source() -> MathResult<TrainDataSource> {
        let mut data = TrainDataSource::new();
        data.push(Matrix::vector(&vec![0.0, 0.0])?, Matrix::vector(&vec![0.0])?);
        data.push(Matrix::vector(&vec![1.0, 0.0])?, Matrix::vector(&vec![1.0])?);
        data.push(Matrix::vector(&vec![0.0, 1.0])?, Matrix::vector(&vec![1.0])?);
        data.push(Matrix::vector(&vec![1.0, 1.0])?, Matrix::vector(&vec![0.0])?);
        Ok(data)
    }
    
    #[test]
    fn network_xor_training() -> MathResult<()> {
        // xor training
        let mut network = FeedforwardNetwork::new(vec![
            Box::new(Dense::new(2, 3)),
            Box::new(Activation::tanh()),
            Box::new(Dense::new(3, 1)),
            Box::new(Activation::tanh()),
        ]);
        let data_source = xor_data_source()?;
        _ = network.train(30_000, 0.05, &data_source);
        let eps = 1e-1;
        for item in data_source.content() {
            let input = &item.input;
            let eval = network.eval(input)?;
            assert!(sub(&eval, &item.output)?.mean() < eps, "Wrong evaluated value for case {:?}, value = {:?}, expected = {:?}", input, eval, item.output);
        } 
        Ok(())
    }

}
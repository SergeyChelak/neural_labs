pub type TensorIndex = Vec<usize>;
pub type TensorBounds = Vec<usize>;

#[derive(Debug)]
pub enum TensorError {
    IndexOutOfBounds,
    IncompatibleTensorShapes,
    IncorrectShape,
}

pub type TensorResult<T> = Result<T, TensorError>;

// pub trait TensorPairWise<T: Copy> {
//     fn pair_wise<F>(&self, other: &Tensor<T>, func: F) -> TensorResult<Tensor<T>> where F: Fn(T, T) -> T;
// }
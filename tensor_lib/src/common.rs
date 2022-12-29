pub type TensorIndex = Vec<usize>;
pub type TensorBounds = Vec<usize>;

pub enum TensorError {
    IndexOutOfBounds,
    IncompatibleTensorShapes,
}

pub type TensorResult<T> = Result<T, TensorError>;
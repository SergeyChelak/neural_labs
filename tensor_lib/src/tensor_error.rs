pub enum TensorError {
    IndexOutOfBounds,
    IncompatibleTensorShapes,
}

pub type TensorResult<T> = Result<T, TensorError>;
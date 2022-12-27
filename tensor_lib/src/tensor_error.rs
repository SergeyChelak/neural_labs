pub enum TensorError {
    IndexOutOfBounds,
}

pub type TensorResult<T> = Result<T, TensorError>;
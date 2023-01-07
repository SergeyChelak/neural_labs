pub type TensorIndex = Vec<usize>;
pub type TensorBounds = Vec<usize>;

#[derive(Debug)]
pub enum TensorError {
    IndexOutOfBounds,
    IncompatibleTensorShapes,
    IncorrectShape,
}

pub type TensorResult<T> = Result<T, TensorError>;

pub trait ElementWise<T: Copy> {
    fn element_wise<F>(&mut self, func: F) where F: Fn(T) -> T;
}

pub trait ElementWiseOperations<T: Copy> {
    fn mul_assign(&mut self, rhs: T);

    fn div_assign(&mut self, rhs: T);
}
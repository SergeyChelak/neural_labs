pub type TensorIndex = Vec<usize>;
pub type TensorBounds = Vec<usize>;

#[derive(Debug, PartialEq, Eq)]
pub enum TensorError {
    IndexOutOfBounds,
    IncompatibleTensorShapes,
    IncorrectShape,
    ModifyingSharedTensorBuffer,
}

pub type TensorResult<T> = Result<T, TensorError>;

pub trait ElementWise<T: Copy> {
    fn element_wise<F>(&mut self, func: F) -> TensorResult<()> where F: Fn(T) -> T;
}

pub trait ElementWiseOperations<T: Copy> {
    fn mul_assign(&mut self, rhs: T) -> TensorResult<()>;

    fn div_assign(&mut self, rhs: T) -> TensorResult<()>;
}

pub trait PairWise<T: Copy>: Sized {
    fn pair_wise<F>(&self, other: &Self, func: F) -> TensorResult<Self> where F: Fn(T, T) -> T;
}
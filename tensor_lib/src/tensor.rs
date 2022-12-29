use std::ops::Deref;

use super::tensor_error::*;

pub type TensorIndex = Vec<usize>;
pub type TensorShape = Vec<usize>;

pub struct Tensor<T> {
    buffer: Vec<T>,
    shape: TensorShape,
}

impl<T: Copy> Tensor<T> {
    pub fn new() -> Self {
        todo!("create new tesor filled with zeros")
    }
        
    pub fn element_wise<F>(&mut self, func: F) where F: Fn(T) -> T {
        self.buffer.iter_mut().for_each(|elem| {
            *elem = func(*elem);
        });
    }

    pub fn pair_wise<F>(&self, other: &Tensor<T>) -> TensorResult<Tensor<T>> where F: Fn(T) -> T {
        todo!("tensor pair wise")
    }

    pub fn get(&self, index: &TensorIndex) -> TensorResult<T> {
        if self.is_valid_index(index) {
            Ok(self.get_unchecked(index))
        } else {
            Err(TensorError::IndexOutOfBounds)
        }
    }

    pub fn get_unchecked(&self, index: &TensorIndex) -> T {
        todo!("get unchecked tensor element")
    }

    pub fn set(&mut self, index: &TensorIndex, value: T) -> TensorResult<()> {
        if self.is_valid_index(index) {
            Ok(self.set_unchecked(index, value))
        } else {
            Err(TensorError::IndexOutOfBounds)
        }
    }

    pub fn set_unchecked(&mut self, index: &TensorIndex, value: T) {
        todo!("set unchecked tensor element")
    }

    fn buffer_index(&self, index: &TensorIndex) -> usize {
        todo!("internal buffer index")
    }

    pub fn is_same_shape(&self, other: &Tensor<T>) -> bool {
        todo!("Validate shape")
    }

    pub fn is_valid_index(&self, index: &TensorIndex) -> bool {
        todo!("Validate index")
    }

    #[inline(always)]
    pub fn rank(&self) -> usize {
        self.shape.len()
    }
}

impl<T: Copy> Tensor<T> {
    
}
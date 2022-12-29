use super::common::*;
use super::tensor_shape::TensorShape;

pub struct Tensor<T> {
    buffer: Vec<T>,
    shape: TensorShape,
}

impl<T: Copy> Tensor<T> {
    pub fn new(shape: TensorBounds, value: T) -> Self {
        let shape = TensorShape::new(shape);
        Self {
            buffer: vec![value; shape.count()],
            shape,
        }
    }
        
    pub fn element_wise<F>(&mut self, func: F) where F: Fn(T) -> T {
        self.buffer.iter_mut().for_each(|elem| {
            *elem = func(*elem);
        });
    }

    pub fn pair_wise<F>(&self, other: &Tensor<T>, func: F) -> TensorResult<Self> where F: Fn(T, T) -> T {
        if self.shape.is_same_shape(&other.shape) {
            return Err(TensorError::IncompatibleTensorShapes);
        }
        let buffer: Vec<T> = self.buffer.iter()
            .zip(other.buffer.iter())
            .map(|(slf, othr)| func(*slf, *othr))
            .collect();
        Ok(Self {
            buffer,
            shape: self.shape.clone(),
        })
    }

    pub fn get(&self, index: &TensorIndex) -> TensorResult<T> {
        if self.shape.is_valid_index(index) {
            Ok(self.get_unchecked(index))
        } else {
            Err(TensorError::IndexOutOfBounds)
        }
    }

    pub fn get_unchecked(&self, index: &TensorIndex) -> T {
        let buf_idx = self.shape.buffer_index(index);
        self.buffer[buf_idx]
    }

    pub fn set(&mut self, index: &TensorIndex, value: T) -> TensorResult<()> {
        if self.shape.is_valid_index(index) {
            Ok(self.set_unchecked(index, value))
        } else {
            Err(TensorError::IndexOutOfBounds)
        }
    }

    pub fn set_unchecked(&mut self, index: &TensorIndex, value: T) {
        let buf_idx = self.shape.buffer_index(index);
        self.buffer[buf_idx] = value;
    }
}
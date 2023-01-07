use core::slice::{
    Iter,
    IterMut
};
use std::rc::Rc;
use super::common::*;
use super::tensor_shape::TensorShape;

pub struct Tensor<T> {
    buffer_ref: Rc<Vec<T>>,
    shape: TensorShape,
}

impl<T: Copy> Tensor<T> {
    pub fn new(shape: TensorBounds, value: T) -> Self {
        let shape = TensorShape::new(shape);
        Self {
            buffer_ref: Rc::new(vec![value; shape.count()]),
            shape,
        }
    }

    fn buffer<'a>(&'a self) -> &'a [T] {
        let (beg, end) = self.shape.absolute_bounds();
        &self.buffer_ref[beg..end]
    }

    fn buffer_mut<'a>(&'a mut self) -> &'a mut [T] {
        let buffer = Rc::get_mut(&mut self.buffer_ref).unwrap();
        let (beg, end) = self.shape.absolute_bounds();
        &mut buffer[beg..end]
    }

    #[inline(always)]
    pub fn iter(&self) -> Iter<T> {
        self.buffer().iter()
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.buffer_mut().iter_mut()
    }
        
    pub fn element_wise<F>(&mut self, func: F) where F: Fn(T) -> T {
        self.iter_mut().for_each(|elem| {
            *elem = func(*elem);
        });
    }

    pub fn pair_wise<F>(&self, other: &Tensor<T>, func: F) -> TensorResult<Self> where F: Fn(T, T) -> T {
        if self.shape.is_same_shape(&other.shape) {
            return Err(TensorError::IncompatibleTensorShapes);
        }
        let buffer: Vec<T> = self.iter()
            .zip(other.buffer_ref.iter())
            .map(|(slf, othr)| func(*slf, *othr))
            .collect();
        Ok(Self {
            buffer_ref: Rc::new(buffer),
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
        self.buffer()[buf_idx]
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
        let buffer = self.buffer_mut();
        buffer[buf_idx] = value;
    }
}
use super::tensor_error::*;

pub type TensorIndex = Vec<usize>;
pub type TensorShape = Vec<usize>;

pub struct Tensor<T> {
    buffer: Vec<T>,
    shape: TensorShape,
    shape_offsets: TensorShape,
}

impl<T> Tensor<T>  {
    pub fn empty() -> Self {
        Self {
            buffer: vec![],
            shape: vec![],
            shape_offsets: vec![],
        }
    }   
}

impl<T: Copy> Tensor<T> {
    pub fn new(shape: TensorShape, value: T) -> Self {
        if shape.len() == 0 {
            return Self::empty();
        }
        let shape_offsets = Self::shape_offsets(&shape);
        let buffer_size = shape.iter().fold(1, |acc, v| acc * v);
        let buffer = vec![value; buffer_size];
        Self {
            buffer,
            shape,
            shape_offsets
        }
    }

    fn shape_offsets(shape: &TensorShape) -> TensorShape {
        let size = shape.len();
        if size == 0 {
            return vec![];
        }
        let mut result: TensorShape = Vec::with_capacity(size);
        result.push(1);
        for i in 0..size-1 {
            let rate = result[i] * shape[size - i - 1];
            result.push(rate);
        }
        result.reverse();
        result
    }
        
    pub fn element_wise<F>(&mut self, func: F) where F: Fn(T) -> T {
        self.buffer.iter_mut().for_each(|elem| {
            *elem = func(*elem);
        });
    }

    pub fn pair_wise<F>(&self, other: &Tensor<T>, func: F) -> TensorResult<Self> where F: Fn(T, T) -> T {
        if self.is_same_shape(other) {
            return Err(TensorError::IncompatibleTensorShapes);
        }
        let buffer: Vec<T> = self.buffer.iter()
            .zip(other.buffer.iter())
            .map(|(slf, othr)| func(*slf, *othr))
            .collect();
        Ok(Self {
            buffer,
            shape: self.shape.clone(),
            shape_offsets: self.shape_offsets.clone(),
        })
    }

    pub fn get(&self, index: &TensorIndex) -> TensorResult<T> {
        if self.is_valid_index(index) {
            Ok(self.get_unchecked(index))
        } else {
            Err(TensorError::IndexOutOfBounds)
        }
    }

    pub fn get_unchecked(&self, index: &TensorIndex) -> T {
        let buf_idx = self.buffer_index(index);
        self.buffer[buf_idx]
    }

    pub fn set(&mut self, index: &TensorIndex, value: T) -> TensorResult<()> {
        if self.is_valid_index(index) {
            Ok(self.set_unchecked(index, value))
        } else {
            Err(TensorError::IndexOutOfBounds)
        }
    }

    pub fn set_unchecked(&mut self, index: &TensorIndex, value: T) {
        let buf_idx = self.buffer_index(index);
        self.buffer[buf_idx] = value;
    }

    fn buffer_index(&self, index: &TensorIndex) -> usize {
        self.shape_offsets.iter()
            .zip(index.iter())
            .map(|(offs, idx)| offs * idx)
            .fold(0usize, |acc, v| acc + v)
    }

    pub fn is_same_shape(&self, other: &Tensor<T>) -> bool {
        if !self.is_same_rank(other) {
            return false;
        }
        self.shape.iter()
            .zip(other.shape.iter())
            .map(|(a, b)| a == b)
            .fold(true, |acc, v| acc && v)
    }

    pub fn is_valid_index(&self, index: &TensorIndex) -> bool {
        if index.len() != self.shape.len() {
            return false;
        }
        self.shape.iter()
            .zip(index.iter())
            .map(|(shape, index)| shape > index )
            .fold(true, |acc, v| acc && v)
    }

    #[inline(always)]
    pub fn rank(&self) -> usize {
        self.shape.len()
    }

    #[inline(always)]
    pub fn is_same_rank(&self, other: &Tensor<T>) -> bool {
        self.rank() != other.rank()
    }
}
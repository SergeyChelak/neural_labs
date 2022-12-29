use super::common::*;

#[derive(Clone)]
pub struct TensorShape {
    shape: TensorBounds,
    offsets: TensorBounds,
}

impl TensorShape {
    pub fn new(shape: TensorBounds) -> Self {
        if shape.len() == 0 {
            return Self::empty();
        }
        let shape_offsets = Self::shape_offsets(&shape);
        Self {
            shape,
            offsets: shape_offsets
        }
    }

    pub fn empty() -> Self {
        Self {
            shape: vec![],
            offsets: vec![],
        }
    }

    fn shape_offsets(shape: &TensorBounds) -> TensorBounds {
        let size = shape.len();
        if size == 0 {
            return vec![];
        }
        let mut result: TensorBounds = Vec::with_capacity(size);
        result.push(1);
        for i in 0..size-1 {
            let rate = result[i] * shape[size - i - 1];
            result.push(rate);
        }
        result.reverse();
        result
    }

    pub fn count(&self) -> usize {
        if self.shape.len() > 0 {
            self.shape.iter().fold(1, |acc, v| acc * v)
        } else {
            0
        }
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

    pub fn buffer_index(&self, index: &TensorIndex) -> usize {
        self.offsets.iter()
            .zip(index.iter())
            .map(|(offs, idx)| offs * idx)
            .fold(0usize, |acc, v| acc + v)
    }

    pub fn is_same_shape(&self, other: &TensorShape) -> bool {
        if !self.is_same_rank(other) {
            return false;
        }
        self.shape.iter()
            .zip(other.shape.iter())
            .map(|(a, b)| a == b)
            .fold(true, |acc, v| acc && v)
    }

    #[inline(always)]
    pub fn is_same_rank(&self, other: &TensorShape) -> bool {
        self.rank() != other.rank()
    }

    #[inline(always)]
    pub fn rank(&self) -> usize {
        self.shape.len()
    }
}

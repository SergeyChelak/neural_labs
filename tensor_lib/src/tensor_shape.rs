use super::common::*;

#[derive(Clone)]
pub struct TensorShape {
    shape: TensorBounds,
    offsets: TensorBounds,
    absolute_offset: usize,
    count: usize,
}

impl TensorShape {
    pub fn new(shape: TensorBounds) -> Self {
        let (count, offsets) = Self::shape_parameters(&shape);
        Self {
            shape,
            offsets,
            absolute_offset: 0usize,
            count,
        }
    }

    fn shape_parameters(shape: &TensorBounds) -> (usize, TensorBounds) {
        let size = shape.len();
        if size == 0 {
            return (0, vec![]);
        }
        let mut offsets: TensorBounds = Vec::with_capacity(size);
        offsets.push(1);
        for i in 0..size {
            let rate = offsets[i] * shape[size - i - 1];
            offsets.push(rate);
        }
        let count = offsets.pop().unwrap_or(0);
        offsets.reverse();
        (count, offsets)
    }

    pub fn count(&self) -> usize {
        self.count
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
            .fold(self.absolute_offset, |acc, v| acc + v)
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

    pub fn absolute_bounds(&self) -> (usize, usize) {
        (self.absolute_offset, self.absolute_offset + self.count)
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

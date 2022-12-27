use super::tensor_error::*;

pub type TensorFloat = f64;
pub type TensorPosition = Vec<usize>;
pub type TensorSize = Vec<usize>;

static ZERO_DIMENSION: TensorSize = vec![];

pub enum Tensor {
    Scalar(TensorFloat),
    Array(Vec<Tensor>, TensorSize),
}

impl Tensor {
    pub fn new<P>(dimensions: TensorSize, producer: P) -> Tensor where P: Fn(&TensorPosition) -> TensorFloat {
        let mut location: TensorPosition = TensorPosition::with_capacity(dimensions.len());
        Self::tensor(&mut location, &dimensions[..], &producer)
    }

    fn tensor<P>(location: &mut TensorPosition, dimensions: &[usize], producer: &P) -> Tensor where P: Fn(&TensorPosition) -> TensorFloat {
        if dimensions.len() > 0 {
            let dim = *dimensions.first().unwrap();
            let child_dimension = &dimensions[1..];
            let mut child_tensors: Vec<Tensor> = Vec::with_capacity(dim);
            for i in 0..dim {
                location.push(i);
                let tensor = Self::tensor(location, child_dimension, producer);
                _ = location.pop();
                child_tensors.push(tensor);
            }
            Tensor::Array(child_tensors, dimensions.to_vec())
        } else {
            Tensor::Scalar(producer(location))
        }
    }

    pub fn dimensions(&self) -> &TensorSize {
        match self {
            Tensor::Array(_, size) => &size,
            _ => &ZERO_DIMENSION,
        }
    }

    fn is_valid_position(&self, position: &TensorPosition) -> bool {
        let dim_len = self.dimensions().len();
        if dim_len != position.len() {
            return false;
        }
        for i in 0..dim_len {
            if self.dimensions()[i] <= position[i] {
                return false;
            }
        }
        true
    }

    #[inline(always)]
    pub fn rank(&self) -> usize {
        self.dimensions().len()
    }

    #[inline(always)]
    pub fn value(&self) -> Option<TensorFloat> {
        if let Tensor::Scalar(value) = self {
            Some(*value)
        } else {
            None
        }
    }
}

impl std::ops::Index<usize> for Tensor {
    type Output = Tensor;

    fn index(&self, index: usize) -> &Self::Output {
        if let Tensor::Array(vector, _) = self {
            &vector[index]
        } else {
            panic!()
        }
    }
}
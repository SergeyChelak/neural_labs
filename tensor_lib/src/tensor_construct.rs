use super::{
    tensor::Tensor,
    common::*,
};

impl<T: Copy + Default> Tensor<T> {
    pub fn vector(arr: &Vec<T>) -> TensorResult<Self> {
        let size = arr.len();
        if size == 0 {
            return Err(TensorError::IncorrectShape);
        }
        let mut tensor = Tensor::<T>::new(vec![size], T::default());
        for i in 0..size {
            tensor.set(&vec![i], arr[i])?;
        }
        Ok(tensor)
    }

    pub fn matrix(arr: &Vec<Vec<T>>) -> TensorResult<Self> {
        let rows = arr.len();
        if rows == 0 {
            return Err(TensorError::IncorrectShape);
        }
        let cols = arr[0].len();
        if cols == 0 {
            return Err(TensorError::IncorrectShape);
        }
        let mut tensor = Tensor::<T>::new(vec![rows, cols], T::default());
        for i in 0..rows {
            for j in 0..cols {
                tensor.set(&vec![i, j], arr[i][j])?;
            }
        }
        Ok(tensor)
    }
}
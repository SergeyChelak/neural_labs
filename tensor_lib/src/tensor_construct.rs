use super::{
    tensor::Tensor,
    common::*,
};

impl<T: Copy + Default> Tensor<T> {
    pub fn vector(arr: &Vec<T>) -> TensorResult<Self> {
        let size = arr.len();
        let mut tensor = Tensor::<T>::new(vec![size], T::default());
        for i in 0..size {
            tensor.set(&vec![i], arr[i])?;
        }
        Ok(tensor)
    }

    pub fn matrix(arr: &Vec<Vec<T>>) -> TensorResult<Self> {
        todo!()
    }
}
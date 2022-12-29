use super::{
    tensor::Tensor,
};

pub struct SliceTensor<'a, T> {
    _tensor: &'a Tensor<T>,
    // TODO: design mapping to parent tensor...
}

impl<'a, T> SliceTensor<'a, T> {
    pub fn new(_tensor: &'a Tensor<T>) -> Self {
        // Self {
        //     tensor
        // }
        todo!("Not designed")
    }
}
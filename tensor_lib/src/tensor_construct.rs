use super::{
    tensor::Tensor,
    common::*,
};

use rand::Rng;

impl<T: Copy + Default> Tensor<T> {
    pub fn vector(arr: &Vec<T>) -> TensorResult<Self> {
        let size = shape1d(arr)?;
        let mut tensor = Tensor::<T>::new(vec![size], T::default());
        for i in 0..size {
            tensor.set(&vec![i], arr[i])?;
        }
        Ok(tensor)
    }

    pub fn matrix(arr: &Vec<Vec<T>>) -> TensorResult<Self> {
        let (rows, cols) = shape2d(arr)?;
        let mut tensor = Tensor::<T>::new(vec![rows, cols], T::default());
        for i in 0..rows {
            for j in 0..cols {
                tensor.set(&vec![i, j], arr[i][j])?;
            }
        }
        Ok(tensor)
    }

    pub fn tensor3d(arr: &Vec<Vec<Vec<T>>>) -> TensorResult<Self> {
        let (depth, rows, cols) = shape3d(arr)?;
        let mut tensor = Tensor::<T>::new(vec![depth, rows, cols], T::default());
        for d in 0..depth {
            for i in 0..rows {
                for j in 0..cols {
                    tensor.set(&vec![d, i, j], arr[d][i][j])?;
                }
            }
        }
        Ok(tensor)
    }
}

impl Tensor<f64> {
    pub fn rand(bounds: TensorBounds) -> Self {
        let mut tensor = Self::new(bounds, f64::default());
        _ = tensor.randomize();
        tensor
    }

    fn randomize(&mut self) -> TensorResult<()> {
        self.element_wise(|_| {
            let mut rng = rand::thread_rng();
            rng.gen::<f64>()
        })
    }
}

fn shape1d<T>(vec: &Vec<T>) -> TensorResult<usize> {
    Ok(
        size(vec)?
    )
}

fn shape2d<T>(vec: &Vec<Vec<T>>) -> TensorResult<(usize, usize)> {
    let size = size_checked(vec)?;
    Ok(
        (size, shape1d(&vec[0])?)
    )
}

fn shape3d<T>(vec: &Vec<Vec<Vec<T>>>) -> TensorResult<(usize, usize, usize)> {
    let size = size_checked(vec)?;
    let (rows, cols) = shape2d(&vec[0])?;
    Ok((size, rows, cols))
}

fn size<T>(arr: &Vec<T>) -> TensorResult<usize> {
    let size = arr.len();
    if size == 0 {
        Err(TensorError::IncorrectShape)
    } else {
        Ok(size)
    }
}

fn size_checked<T>(arr: &Vec<Vec<T>>) -> TensorResult<usize> {
    let size = size(arr)?;
    let len = arr[0].len();
    for i in 1..size {
        if len != arr[i].len() {
            return Err(TensorError::IncorrectShape);
        }
    }
    Ok(size)
}
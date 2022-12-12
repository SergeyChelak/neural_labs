use super::{
    matrix::Matrix,
    matrix::map,
    errors::*,
};

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.is_same_size(other) {
            let rows = self.rows();
            let cols = self.cols();
            for i in 0..rows {
                for j in 0..cols {
                    if (self[i][j] - other[i][j]).abs() > f64::EPSILON {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }
}

impl Eq for Matrix {
    //
}

impl Matrix {
    pub fn add(&self, other: &Matrix) -> MathResult<Self> {
        add(self, other)
    }

    pub fn sub(&self, other: &Matrix) -> MathResult<Self> {
        sub(self, other)
    }

    pub fn div(&self, other: &Matrix) -> MathResult<Self> {
        div(self, other)
    }

    pub fn product(&self, other: &Matrix) -> MathResult<Self> {
        product(self, other)
    }

    pub fn transpose(&self) -> Self {
        let (rows, cols) = (self.rows(), self.cols());
        Self::new(cols, rows, |i, j| {
            self.get_unchecked(j, i)
        })
    }

    pub fn powi(&self, power: i32) -> Self {
        Self::map(self, |x| x.powi(power))
    }
}

pub trait MatrixMultiplication<T> {
    type ResultType;
    fn mul(&self, value: T) -> Self::ResultType;
}

impl MatrixMultiplication<f64> for Matrix {
    type ResultType = Matrix;
    fn mul(&self, value: f64) -> Self::ResultType {
        self.map(|x| x * value)
    }
}

impl MatrixMultiplication<&Matrix> for Matrix {
    type ResultType = MathResult<Matrix>;
    fn mul(&self, value: &Matrix) -> Self::ResultType {
        mul(self, value)
    }
}

/// Elementwise sum
#[inline]
pub fn add(first: &Matrix, second: &Matrix) -> MathResult<Matrix> {
    map(first, second, |x, y| x + y)
}

/// Elementwise subtraction
#[inline]
pub fn sub(first: &Matrix, second: &Matrix) -> MathResult<Matrix> {
    map(first, second, |x, y| x - y)
}

/// Elementwise multiplication
#[inline]
pub fn mul(first: &Matrix, second: &Matrix) -> MathResult<Matrix> {
    map(first, second, |x, y| x * y)
}

/// Elementwise division
#[inline]
pub fn div(first: &Matrix, second: &Matrix) -> MathResult<Matrix> {
    map(first, second, |x, y| x / y)
}

/// Matrix product
pub fn product(first: &Matrix, second: &Matrix) -> MathResult<Matrix> {
    let (rows, fc) = (first.rows(), first.cols());
    let (sr, cols) = (second.rows(), second.cols());
    if fc == sr {
        let mut matrix = Matrix::zero(rows, cols);
        for i in 0..rows {            
            for j in 0..cols {
                let mut sum = 0.0f64;
                for k in 0..fc {
                    sum += first.get_unchecked(i, k) * second.get_unchecked(k, j);
                }
                matrix.set_unchecked(i, j, sum);
            }
        }
        Ok(matrix)
    } else {
        Err(MathError::IncorrectMatricesDimensions("product".to_string(), first.dimensions(), second.dimensions()))
    }        
}
use super::errors::*;

#[derive(Copy, Clone, Debug)]
pub struct Dimensions {
    pub rows: usize,
    pub cols: usize,
}

impl Dimensions {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }

    pub fn square(dim: usize) -> Self {
        Self::new(dim, dim)
    }

    pub fn from_vector<T>(vec: &Vec<Vec<T>>) -> MathResult<Self> {
        let rows = vec.len();
        if rows == 0 {
            return Ok(Self::new(0, 0));
        }
        let cols = vec[0].len();
        for i in 1..rows {
            if vec[i].len() != cols {
                return Err(MathError::IncorrectVectorDimensions);
            }
        }
        Ok(Self::new(rows, cols))
    }

    #[inline(always)]
    pub fn is_valid_position(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }

    #[inline(always)]
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    #[inline(always)]
    pub fn rows(&self) -> usize {
        self.rows
    }

    #[inline(always)]
    pub fn cols(&self) -> usize {
        self.cols
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.rows * self.cols
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

impl PartialEq for Dimensions {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols
    }
}

impl Eq for Dimensions {
    //
}
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
}

impl PartialEq for Dimensions {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols
    }
}

impl Eq for Dimensions {
    //
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions_init_from_vector() {
        let v = vec![vec![1, 2, 3], vec![2, 3, 4]];
        assert_eq!(Dimensions::from_vector(&v), Ok(Dimensions::new(2, 3)));

        let v: Vec<Vec<()>> = vec![];
        assert_eq!(Dimensions::from_vector(&v), Ok(Dimensions::new(0, 0)));

        let v = vec![vec![1, 2, 3], vec![2, 3]];
        assert_eq!(Dimensions::from_vector(&v), Err(MathError::IncorrectVectorDimensions));

        let v: Vec<Vec<()>> = vec![vec![()]];
        assert_eq!(Dimensions::from_vector(&v), Ok(Dimensions::new(1, 1)));
    }

    #[test]
    fn dimensions_is_square() {
        let d = Dimensions::new(3, 5);
        assert!(!d.is_square(), "Dims are not square");

        let d = Dimensions::new(5, 5);
        assert!(d.is_square(), "Dims expected to be square");
    }
}

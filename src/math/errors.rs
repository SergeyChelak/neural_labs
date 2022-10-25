use std::fmt::{self, write};
use super::dimensions::Dimensions;

#[derive(PartialEq)]
pub enum MathError {
    IncorrectVectorDimensions,
    IncorrectPosition(usize, usize),
    IncorrectMatricesDimensions(String, Dimensions, Dimensions),
}

impl MathError {
    pub fn description(&self) -> String {
        match self {
            MathError::IncorrectVectorDimensions => 
                "Initializing 2d vector must have the same column number in each row".to_string(),
            MathError::IncorrectPosition(row, col) => 
                format!("Row {} and/or col {} are/is out of bounds", row, col),
            MathError::IncorrectMatricesDimensions(op_name, dim1, dim2) => 
                format!("Can't perform operation '{}' with matrices with dimensions {:?} and {:?}", op_name, dim1, dim2),
        }
    }
}

impl fmt::Debug for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

pub type MathResult<T> = std::result::Result<T, MathError>;

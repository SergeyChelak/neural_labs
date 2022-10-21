use super::matrix::*;
use std::fmt;

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix [{}x{}]", self.rows(), self.cols())?;
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                write!(f, "{:8.3}", self.get(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
use super::{
    matrix::Matrix,
    errors::*,
};
use std::ops::{
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
};

impl Matrix {
    pub fn add_assign(&mut self, other: &Matrix) -> MathResult<()> {
        self.modify_other(other, |a, b| a + b)        
    }

    pub fn sub_assign(&mut self, other: &Matrix) -> MathResult<()> {
        self.modify_other(other, |a, b| a - b)
    }

    pub fn mul_assign(&mut self, scalar: f64) {
        self.modify(|x| x * scalar)
    }

    pub fn div_assign(&mut self, scalar: f64) {
        self.modify(|x| x / scalar)
    }
}

impl AddAssign<&Self> for Matrix {
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(rhs).expect("add_assign operator: inappropriate matrix sizes");
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs).expect("add_assign operator: inappropriate matrix sizes");
    }
}

impl SubAssign<&Self> for Matrix {
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(rhs).expect("sub_assign operator: inappropriate matrix sizes");
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(&rhs).expect("sub_assign operator: inappropriate matrix sizes");
    }
}

impl MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, rhs: f64) {
        self.mul_assign(rhs);
    }
}

impl DivAssign<f64> for Matrix {
    fn div_assign(&mut self, rhs: f64) {
        self.div_assign(rhs);
    }
}
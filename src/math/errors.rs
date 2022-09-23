use std::ops::Range;

pub trait MathError {
    fn description(&self) -> String;
}

pub struct IncorrectIndex {
    index: usize,
    valid_range: Range<usize>
}

impl IncorrectIndex {
    pub fn new(index: usize, range: Range<usize>) -> Self {
        IncorrectIndex { index, valid_range: range }
    }
}

impl MathError for IncorrectIndex {
    fn description(&self) -> String {
        format!("Incorrect index {}, allowed range {}..{}", self.index, self.valid_range.start, self.valid_range.end)
    }
}
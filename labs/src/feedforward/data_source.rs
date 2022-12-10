use crate::math::matrix::Matrix;

pub struct TrainItem {
    pub input: Matrix,
    pub output: Matrix,
}

pub struct TrainDataSource {
    data: Vec<TrainItem>
}

impl TrainDataSource {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, input: Matrix, output: Matrix) {
        let item = TrainItem {
            input,
            output
        };
        self.data.push(item);
    }

    pub fn content(&self) -> &Vec<TrainItem> {
        &self.data
    }
}

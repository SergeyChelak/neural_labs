pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    content: Vec<Vec<T>>
}

impl<T> Matrix<T> {
    pub fn new<F>(rows: usize, cols: usize, initializer: F) -> Self where F: Fn(usize, usize) -> T {
        // TODO: initialize properly
        let mut content: Vec<Vec<T>> = Vec::new();
        for i in 0..rows {
            for j in 0..cols {
                //content[i][j] = initializer(i, j);
            }
        }
        Matrix {
            rows,
            cols,
            content
        }
    }

    pub fn rows(&self) -> &usize {
        &self.rows
    }

    pub fn cols(&self) -> &usize {
        &self.cols
    }

    pub fn get_unchecked(&self, row: usize, col: usize) -> &T {
        todo!()
    }

    pub fn set_unchecked(&mut self, row: usize, cols: usize, value: T) {
        todo!()
    }
}
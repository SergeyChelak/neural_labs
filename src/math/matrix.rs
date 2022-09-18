pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    content: Vec<Vec<T>>
}

impl<T> Matrix<T> {
    pub fn new<F>(rows: usize, cols: usize, initializer: F) -> Self where F: Fn(usize, usize) -> T {
        let mut content: Vec<Vec<T>> = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut vector: Vec<T> = Vec::with_capacity(cols);
            for j in 0..cols {
                vector.push(initializer(i, j));
            }
            content.push(vector);
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
        &self.content[row][col]
    }

    pub fn set_unchecked(&mut self, row: usize, col: usize, value: T) {
        self.content[row][col] = value;
    }
}
pub struct Matrix {
    rows: usize,
    cols: usize,
    content: Vec<Vec<f64>>
}

type MatrixInitializer = fn(usize, usize) -> f64;

impl Matrix {
    pub fn new(rows: usize, cols: usize, initializer: MatrixInitializer) -> Self {
        let mut content: Vec<Vec<f64>> = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut vector: Vec<f64> = Vec::with_capacity(cols);
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

    pub fn identity(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols, |row, col| {
            if row == col { 1.0 } else { 0.0 }
        })
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols, |_, _| { 0.0 })
    }

    pub fn from_array(rows: usize, cols: usize, array: Vec<f64>) -> Self {
        todo!()
    }

    pub fn rows(&self) -> &usize {
        &self.rows
    }

    pub fn cols(&self) -> &usize {
        &self.cols
    }

    pub fn get_unchecked(&self, row: usize, col: usize) -> &f64 {
        &self.content[row][col]
    }

    pub fn set_unchecked(&mut self, row: usize, col: usize, value: f64) {
        self.content[row][col] = value;
    }
}
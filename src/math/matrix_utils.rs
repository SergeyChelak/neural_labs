use super::matrix::Matrix;
use rand::Rng;

impl Matrix {    
    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = rand::thread_rng();
        Matrix::new(rows, cols, |_, _| rng.gen::<f64>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn matrix_utils_random() {
        let m = Matrix::random(50, 80);
        for i in 0..m.rows() {
            for j in 0..m.cols() {
                let v = m.get(i, j);
                assert!(v >= 0.0 && v < 1.0, "Incorrect initial random value")
            }
        }
    }

}
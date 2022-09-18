mod math;

fn main() {
    let m: math::matrix::Matrix<f64> = math::matrix::Matrix::new(2, 2, |row, col| { 0.0 });
    println!("Created matrix of size: {} x {}", m.rows(), m.cols());
}

use super::{
    matrix::Matrix,
    errors::MathResult,
    dimensions::Dimensions,
};

pub fn cross_correlation(input: &Matrix, kernel: &Matrix) -> MathResult<Matrix> {
    let (input_rows, input_cols) = input.dimensions().as_tuple();
    let (kernel_rows, kernel_cols) = kernel.dimensions().as_tuple();
    // TODO: add guard that validates matrix & kernel sizes
    let (rows, cols) = Dimensions::new(
        input_rows - kernel_rows + 1,
        input_cols - kernel_cols + 1
    ).as_tuple();
    let mut output = Matrix::zero(rows, cols);
    for out_i in 0..rows {
        for out_j in 0..cols {
            let mut sum = 0.0f64;
            for i in 0..kernel_rows {
                for j in 0..kernel_cols {
                    sum += kernel.get_unchecked(i, j) * input.get_unchecked(i + out_i, j + out_j);
                }
            }
            output.set_unchecked(out_i, out_j, sum);
        }
    }
    Ok(output)
}
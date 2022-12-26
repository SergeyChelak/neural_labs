use super::{
    matrix::Matrix,
    errors::*,
    dimensions::Dimensions,
};

pub fn cross_correlation(input: &Matrix, kernel: &Matrix) -> MathResult<Matrix> {
    let (input_rows, input_cols) = input.dimensions().as_tuple();
    let (kernel_rows, kernel_cols) = kernel.dimensions().as_tuple();
    if input_rows < kernel_rows || input_cols < kernel_cols {
        return Err(MathError::IncorrectMatricesDimensions(
            "Cross Correlation".to_string(), input.dimensions(), kernel.dimensions())
        );
    }
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

fn padding_expand(input: &Matrix, kernel_size: &Dimensions, fill_value: f64) -> MathResult<Matrix> {
    let (input_rows, input_cols) = input.dimensions().as_tuple();
    let (kernel_rows, kernel_cols) = kernel_size.as_tuple();
    if kernel_cols == 0 || kernel_rows == 0 {
        return Err(MathError::IncorrectMatricesDimensions(
            "Padding matrix expand".to_string(), input.dimensions(), kernel_size.to_owned())
        );
    }
    let row_offset = kernel_rows - 1;
    let col_offset = kernel_cols - 1;
    let row_range = row_offset..=input_rows;
    let col_range = col_offset..=input_cols;
    let matrix = Matrix::new(
        input_rows + 2 * row_offset, 
        input_cols + 2 * col_offset, 
        |i, j| {
            if row_range.contains(&i) && col_range.contains(&j) {
                input[i - row_offset][j - col_offset]
            } else {
                fill_value
            }
        }
    );
    Ok(matrix)
}

pub fn full_convolution(input: &Matrix, kernel: &Matrix) -> MathResult<Matrix> {
    let expanded = padding_expand(input, &kernel.dimensions(), 0.0)?;
    cross_correlation(&expanded, kernel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn padding_expand_test() -> MathResult<()> {
        let input = Matrix::from_vector(&vec![
            vec![1.0, 6.0, 2.0],
            vec![5.0, 3.0, 1.0],
            vec![7.0, 0.0, 4.0],
        ])?;
    
        let kernel = Matrix::from_vector(&vec![
            vec![ 1.0, 2.0],
            vec![-1.0, 0.0],
        ])?;

        let expected = Matrix::from_vector(&vec![
            vec![0.0, 0.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 6.0, 2.0, 0.0],
            vec![0.0, 5.0, 3.0, 1.0, 0.0],
            vec![0.0, 7.0, 0.0, 4.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0, 0.0],
        ])?;
        let result = padding_expand(&input, &kernel.dimensions(), 0.0)?;
        println!("{:?}", result);
        assert!(expected == result, "Padding expand works incorrectly");
        Ok(())
    }
}
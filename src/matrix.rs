use std::{fmt::Display, ops::Mul};

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Display + Copy + Clone + Default + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    /* Construtor da struct Matrix */
    pub fn empty(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }

    pub fn new(data: Vec<Vec<T>>) -> Self {
        let count_rows = data.len();

        if !data.iter().all(|row| row.len() == data[0].len()) {
            panic!("All rows must have the same length");
        }

        let count_cols = data[0].len();

        Self {
            rows: count_rows,
            cols: count_cols,
            data: data.into_iter().flatten().collect(),
        }
    }

    /* Acessa um elemento da matriz */
    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[row * self.cols + col]
    }

    /* Define um elemento da matriz */
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.cols + col] = value;
    }

    pub fn transpose(&self) -> Self {
        let mut result = Matrix::empty(self.cols, self.rows);

        for row in 0..self.rows {
            for col in 0..self.cols {
                result.set(col, row, self.get(row, col));
            }
        }

        result
    }

    pub fn identity(size: usize) -> Self {
        let mut result = Matrix::empty(size, size);

        for i in 0..size {
            result.set(i, i, T::default());
        }

        result
    }
}

impl<T> Display for Matrix<T>
where
    T: Display + Copy + Clone + Default + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("Matrix({}x{}) [\n", self.rows, self.cols);
        let mut col_widths = vec![1; self.cols];

        for row in 0..self.rows {
            for col in 0..self.cols {
                let value = format!("{}", self.get(row, col));
                col_widths[col] = col_widths[col].max(value.len());
            }
        }

        for row in 0..self.rows {
            result.push_str("  ");

            for col in 0..self.cols {
                let value = format!("{}", self.get(row, col));
                let padding = String::from(" ").repeat(col_widths[col] - value.len());

                result.push_str(padding.as_str());
                result.push_str(&value);

                if col != self.cols - 1 {
                    result.push_str("  ");
                }
            }

            result.push('\n');
        }

        return write!(f, "{}]", result);
    }
}

impl Mul for Matrix<f64> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("The number of columns of the first matrix must be equal to the number of rows of the second matrix");
        }

        let mut result = Matrix::empty(self.rows, rhs.cols);

        for row in 0..self.rows {
            for col in 0..rhs.cols {
                let mut sum = 0.0;

                for i in 0..self.cols {
                    sum += self.get(row, i) * rhs.get(i, col);
                }

                result.set(row, col, sum);
            }
        }

        result
    }
}
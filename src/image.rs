use std::fmt::{Display, Formatter};

use crate::{matrix::Matrix, point};

pub struct Image(pub Matrix<u8>);

impl Image {
    pub fn empty(width: usize, height: usize) -> Self {
        Self(Matrix::empty(height, width))
    }

    pub fn new(pixels: Matrix<u8>) -> Self {
        Self(pixels)
    }

    pub fn transform(&mut self, transform: Matrix<f64>) -> &mut Self {
        let old_image = self.0.clone();

        self.0 = Matrix::empty(old_image.rows, old_image.cols);

        for row in 0..old_image.rows {
            for col in 0..old_image.cols {
                let pixel = point!(col as f64, row as f64);
                let new_pixel = transform.clone() * pixel;

                let row = new_pixel.get(1, 0) as usize;
                let col = new_pixel.get(0, 0) as usize;

                if row >= self.0.rows || col >= self.0.cols {
                    continue;
                }

                self.0.set(
                    row,
                    col,
                    old_image.get(row, col),
                );
            }
        }

        return self
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for row in 0..self.0.rows {
            for col in 0..self.0.cols {
                let value = self.0.get(row, col);
                
                if value == 0 {
                    result.push_str(".");
                } else {
                    result.push_str("*");
                }
            }

            result.push('\n');
        }

        write!(f, "{}", result)
    }
}
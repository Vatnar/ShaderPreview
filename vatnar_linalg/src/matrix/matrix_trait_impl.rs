use super::Matrix;
use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix")?;
        for row in 0..self.rows {
            for col in 0..self.cols {
                let idx = row * self.cols + col;
                write!(f, "{} ", self.data[idx])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
/// Use [`Matrix::row_eq`] to check if two matrices are row equivalent.
/// This method only checks for exact structural equality: rows, columns, and data.
impl PartialEq<Self> for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.data == other.data
    }
}

impl Eq for Matrix {}

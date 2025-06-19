use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}
use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix")?;
        for row in 0..self.rows {
            for col in 0..self.cols {
                let idx = row * self.cols + col;
                write!(f, "{} ", self.data[idx])?;
            }
            writeln!(f)?; // new line after each row
        }
        Ok(())
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // You can simply reuse Display formatting for Debug:
        write!(f, "{}", self)
    }
}

// TODO consider defining submatrices

pub struct MatrixView<'a> {
    parent: &'a Matrix,
    row_range: RangeInclusive<usize>,
    col_range: RangeInclusive<usize>,
}

impl Matrix {
    pub fn empty(rows: usize, cols: usize) -> Self {
        let data: Vec<f64> = Vec::with_capacity(rows * cols);
        Matrix { data, rows, cols }
    }

    pub fn new(rows: usize, cols: usize, array: &[f64]) -> Self {
        let mut matrix = Self::empty(rows, cols);
        matrix.insert(array);
        matrix
    }
}

impl<'a> MatrixView<'a> {
    pub fn clone(&self) -> Matrix {
        self.parent
            .submatrix(self.row_range.clone(), self.col_range.clone())
    }
}
impl Matrix {
    /// Creates a submatrix from the original matrix by selecting specific rows and columns.
    ///
    /// The indices for rows and columns are **1-based and inclusive**.\
    /// This is to make it match with the size of matrices
    /// For example, consider this 4×4 matrix:
    ///
    /// ```text
    /// 1 1 1 1
    /// 4 2 2 6
    /// 3 3 3 3
    /// 4 4 4 4
    /// ```
    ///
    /// To extract the submatrix:
    ///
    /// ```text
    /// 1 1
    /// 2 2
    /// ```
    ///
    /// You would pass the ranges `1..=2` for rows and `2..=3` for columns.
    ///
    /// # Arguments
    ///
    /// * `row_range` - Inclusive range of rows to include (starting at 1).
    /// * `col_range` - Inclusive range of columns to include (starting at 1).
    ///
    /// # Returns
    ///
    /// A new `Matrix` containing the selected submatrix.
    pub fn submatrix(
        &self,
        row_range: RangeInclusive<usize>,
        col_range: RangeInclusive<usize>,
    ) -> Self {
        let row_range = (row_range.start() - 1..=row_range.end() - 1);
        let col_range = (col_range.start() - 1..=col_range.end() - 1);

        let mut data = Vec::with_capacity(row_range.clone().count() * col_range.clone().count());
        for outer in row_range.clone() {
            // walk columns
            for j in col_range.clone() {
                let pos_of_current_val = outer * self.cols + j;
                // add to submatrix at the pos_of_current_val (index into vec)
                data.push(self.data[pos_of_current_val]);
            }
        }
        Matrix {
            data,
            rows: row_range.count(),
            cols: col_range.count(),
        }
    }
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
    pub fn view(
        &self,
        row_range: RangeInclusive<usize>,
        col_range: RangeInclusive<usize>,
    ) -> MatrixView {
        MatrixView {
            parent: &self,
            row_range,
            col_range,
        }
    }
    pub fn insert(&mut self, array: &[f64]) {
        assert_eq!(self.rows * self.cols, array.len());
        self.data = array.into();
    }
    pub fn inverse(&self) -> Self {
        // TODO Augment matrix by identity matrix, then transform matrix into identity,
        // that will give augmented part = inverse
        // TODO Implement solving first, consider having it "modular" so it works with
        // bigger augmented matrix, or maybe specify what is augmented and not, or perhaps submatrix
        self.clone()
    }

    /// Reduces given matrix to echelon form \
    /// use [`Matrix::reduced_echelon`] for reduced echelon form
    // might need to take in if its augmented or not
    pub fn echelon(&self) -> Self {
        // Check if already in echelon form
        // then do it
        todo!();
    }

    /// Reduces given matrix to reduced echelon form \
    /// use [`Matrix::echelon`] for echelon form
    pub fn reduced_echelon(&self) -> Self {
        todo!();
    }

    pub fn row_eq(&self, other: &Self) -> bool {
        todo!()
    }
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

// Allows access to public vec functions
// impl Deref for Matrix {
//     type Target = Vec<f64>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.data
//     }
// }
#[macro_export]
macro_rules! to_f64 {
    ($($x:expr),* $(,)?) => {
        &[$($x as f64),*]
    };
}

/// Use [`Matrix::row_eq`] to check if two matrices are row equivalent.
/// This method only checks for exact structural equality: rows, columns, and data.
impl PartialEq<Self> for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.data == other.data
    }
}

impl Eq for Matrix {}

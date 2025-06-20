pub use matrix_view::MatrixView;
use std::ops::RangeInclusive;
mod macros;
mod matrix_solve;
mod matrix_trait_impl;
mod matrix_view;

/// Used for storing matrices and doing matrix operations
/// # Examples
/// ```
/// use vatnar_linalg::{to_f64, Matrix};
///  let m = to_f64!(
///             1, 2, 3, 4,
///             5, 5, 4, 3,
///             2, 3, -4, 3,
///             3, 1, 3, -4,);
/// let m = Matrix::new(4, 4, m);
/// ```
/// You can also row reduce by using [`Matrix::echelon`] and [`Matrix::reduced_echelon`]
/// ```
/// m.echelon();
/// ```
#[derive(Clone)]
pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
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

// methods
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
        let zero_based_rows = (row_range.start() - 1)..=(row_range.end() - 1);
        let zero_based_cols = (col_range.start() - 1)..=(col_range.end() - 1);

        let data: Vec<f64> = zero_based_rows
            .clone()
            .flat_map(|row| {
                zero_based_cols.clone().map(move |col| {
                    let idx = row * self.cols + col;
                    self.data[idx]
                })
            })
            .collect();

        Matrix {
            data,
            rows: zero_based_rows.count(),
            cols: zero_based_cols.count(),
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
        MatrixView::new(self, row_range, col_range)
    }

    pub fn insert(&mut self, array: &[f64]) {
        self.data = array.into();
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
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl Matrix {
    pub fn truncate(&self, decimals: usize) -> Self {
        let factor = 10f64.powi(decimals as i32);
        let truncated: Vec<f64> = self
            .data
            .iter()
            .map(|x| (x * factor).round() / factor)
            .collect();
        Matrix::new(self.rows, self.cols, truncated.as_slice())
    }
}

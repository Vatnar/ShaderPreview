//! [`Matrix`] type
use matrix_view::MatrixView;
use std::ops::RangeInclusive;

pub mod macros;
pub mod matrix_view;
pub mod solve;

mod trait_impl;

/// Used for storing matrices and doing matrix operations
///
/// # Examples
/// ```
/// use vatnar_linalg::{to_f64, Matrix};
/// let m = to_f64!(
///             1, 2, 3, 4,
///             5, 5, 4, 3,
///             2, 3, -4, 3,
///             3, 1, 3, -4,);
/// let m = Matrix::new(4, 4, m);
///```
/// You can also row reduce by using [`Matrix::echelon`] and [`Matrix::reduced_echelon`]
#[derive(Clone)]
pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

// ===== Constructors =====
impl Matrix {
    /// Creates an empty matrix with the given size,
    /// and pre-allocates a container for storing future values
    ///
    /// # Arguments
    /// * `rows` - row count of matrix
    /// * `cols` - column count of matrix
    ///
    /// # Examples
    /// Create an empty matrix4x4
    /// ```
    /// use vatnar_linalg::Matrix;
    /// let matrix = Matrix::empty(4,4);
    /// assert_eq!(matrix.rows(), 4);
    /// assert_eq!(matrix.cols(), 4);
    /// ```
    pub fn empty(rows: usize, cols: usize) -> Self {
        let data: Vec<f64> = Vec::with_capacity(rows * cols);
        Matrix { data, rows, cols }
    }

    /// Creates a matrix filled with data in the flat slice `array`
    ///
    /// # Arguments
    /// * `rows` - row count of matrix
    /// * `cols` - column count of matrix
    /// * `array` - [`std::f64`] flat array slice with array data
    ///
    /// # Panics
    /// If the length of given array slice does not equal the capacity of the matrix
    ///
    /// # Examples
    /// ```
    /// use vatnar_linalg::{to_f64, Matrix};
    ///
    /// #[rustfmt::skip]
    /// let data = to_f64!(
    ///     3, -2, -3, 3,
    ///     2, 3, 3, 2,
    /// );
    /// let matrix = Matrix::new(2, 4, data);
    ///
    /// assert_eq!(matrix.rows(), 2);
    /// assert_eq!(matrix.cols(), 4);
    /// assert_eq!(matrix.get(1, 1), 3.0);
    /// assert_eq!(matrix.get(2, 4), 2.0);
    /// ```
    pub fn new(rows: usize, cols: usize, array: &[f64]) -> Self {
        assert_eq!(rows * cols, array.len());
        let mut matrix = Self::empty(rows, cols);
        matrix.insert(array);
        matrix
    }
}

// ===== Methods =====
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
    /// # Examples
    ///
    /// ```
    /// use vatnar_linalg::{to_f64, Matrix};
    ///
    /// #[rustfmt::skip]
    /// let data = to_f64!(
    ///     1.0, 2.0, 3.0, 4.0,
    ///     5.0, 6.0, 7.0, 8.0,
    ///     9.0, 10.0, 11.0, 12.0,
    /// );
    /// let matrix = Matrix::new(3, 4, data);
    ///
    /// // Extract submatrix rows 1..=2, cols 2..=3 (1-based indexing)
    /// let sub = matrix.submatrix(1..=2, 2..=3);
    ///
    /// assert_eq!(sub.rows(), 2);
    /// assert_eq!(sub.cols(), 2);
    /// assert_eq!(sub.get(1, 1), 2.0);
    /// assert_eq!(sub.get(2, 2), 7.0);
    /// ```
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

    /// Returns the count of rows of matrix
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the count of columns of matrix
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns a [`MatrixView`] (virtual) of original matrix based on inclusive 1-indexed ranges
    ///
    /// # Arguments
    /// * `row_range` [`RangeInclusive`]
    pub fn view(
        &self,
        row_range: RangeInclusive<usize>,
        col_range: RangeInclusive<usize>,
    ) -> MatrixView {
        MatrixView::new(self, row_range, col_range)
    }

    /// Inserts array slice of [`std::f64`] into empty [`Matrix`]
    pub fn insert(&mut self, array: &[f64]) {
        self.data = array.into();
    }

    // TODO check if actually works
    /// Check if given matrices are row equivalent
    /// # Arguments
    /// * `other` - other [`Matrix`]
    pub fn row_eq(&self, other: &Self) -> bool {
        self.reduced_echelon(0) == other.reduced_echelon(0)
    }

    /// Returns the capacity of matrix
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Returns the length of the matrix (flattened)
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if matrix is empty (not zeroed out)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Truncates all digits of decimals after `decimals`
    ///
    /// Useful for rounding off floating point errors
    ///
    /// # Arguments
    /// * `decimals` - amounts of decimals that should count
    pub fn truncate(&self, decimals: usize) -> Self {
        let factor = 10f64.powi(decimals as i32);
        let truncated: Vec<f64> = self
            .data
            .iter()
            .map(|x| (x * factor).round() / factor)
            .collect();
        Matrix::new(self.rows, self.cols, truncated.as_slice())
    }

    /// Returns the value at given `row` and `col`
    ///
    /// # Arguments
    /// * `row` - row of value to return
    /// * `col` - column of value to return
    pub fn get(&self, row: usize, col: usize) -> f64 {
        assert!(row - 1 <= self.rows && col - 1 <= self.cols);
        self.data[(row - 1) * self.cols + col - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::to_f64;

    #[test]
    fn test_matrix() {
        let mut m = Matrix::empty(2, 2);

        assert_eq!(m.len(), 0);
        m.insert(&[0.0, 0.0, 1.0, 0.0]);

        assert_eq!(m.capacity(), 4);

        #[rustfmt::skip]
        let m = to_f64!(
            1, 2, 3, 4,
            5, 5, 4, 3,
            2, 3, -4, 3,
            3, 1, 3, -4,);

        let m = Matrix::new(4, 4, m);
        assert_eq!(m.len(), 16);
        assert_eq!(m.capacity(), 16);

        let m2 = m.inverse();
    }
    #[test]
    fn test_matrix_eq() {
        #[rustfmt::skip]
        let m = to_f64!(
            1, 2, 3, 4,
            5, 5, 4, 3,
            2, 3, -4, 3,
            3, 1, 3, -4,);

        let m = Matrix::new(4, 4, m);

        let n = m.clone();

        assert_eq!(m, n)
    }
    #[test]

    fn test_matrix_reduce() {
        #[rustfmt::skip]
        let m = to_f64!(
            1, 2, 3, 4,
            5, 5, 4, 3,
            2, 3, -4, 3,
            3, 1, 3, -4,);

        let m = Matrix::new(4, 4, m);

        let n = m.echelon();
        let o = m.reduced_echelon(0);
    }
    #[test]
    fn test_matrix_sub() {
        #[rustfmt::skip]
        let m = to_f64!(
            1, 2, 3, 4,
            5, 5, 4, 3,
            2, 3, -4, 3,
            3, 1, 3, -4,);

        let m = Matrix::new(4, 4, m);

        let n = m.submatrix((1..=2), (2..=3));

        println!("m_rows: {} \nm_cols: {} \nm: {m}", m.rows(), m.cols());
        println!("n_rows: {} \nn_cols: {} \nn: {n}", n.rows(), n.cols());
    }

    #[test]
    fn test_matrix_viewsub() {
        #[rustfmt::skip]
        let m = to_f64!(
            1, 2, 3, 4,
            5, 5, 4, 3,
            2, 3, -4, 3,
            3, 1, 3, -4,);

        let m = Matrix::new(4, 4, m);

        let n = m.view((1..=2), (2..=3));
        let n = n.to_matrix();

        println!("m_rows: {} \nm_cols: {} \nm: {m}", m.rows(), m.cols());
        println!("n_rows: {} \nn_cols: {} \nn: {n}", n.rows(), n.cols());
    }

    #[test]
    fn test_matrix_echelon() {
        #[rustfmt::skip]
        let m = to_f64!(
            1, 2, 3, 4,
            5, 5, 4, 3,
            2, 3, -4, 3,
            3, 1, 3, -4,);
        let m = Matrix::new(4, 4, m);
        let n = m.echelon();

        println!("{m}");
        println!("{n}");
    }
    #[test]
    fn test_matrix_echelon2() {
        #[rustfmt::skip]
        let m = to_f64!(
            3, -2, -3, 3,
            2, 3, 3, 2,);
        let m = Matrix::new(2, 4, m);
        let n = m.echelon();

        let u = m.reduced_echelon(0);

        println!("{m}");
        println!("{n}");
        println!("{u}")
    }

    #[test]
    fn test_matrix_inverse() {
        #[rustfmt::skip]
        let m = to_f64!(
        2.0, 1.0, 1.0,
        1.0, 3.0, 2.0,
        1.0, 0.0, 0.0
    );
        let m = Matrix::new(3, 3, m);
        let m_inverse = m.inverse();
        println!("{m}, {m_inverse}")
    }

    // #[test]
    // fn test_matrix_inverse() {
    //     #[rustfmt::skip]
    // let m = to_f64!(
    //     2.0, 1.0, 1.0,
    //     1.0, 3.0, 2.0,
    //     1.0, 0.0, 0.0
    // );
    //     let m = Matrix::new(3, 3, m);
    //     let m_inv = m.inverse();
    //
    //     // Compute A × A⁻¹
    //     let product = m.clone() * m_inv.clone();
    //
    //     // Compare with identity
    //     let identity = Matrix::identity(3);
    //
    //     // Allow floating point tolerance
    //     for i in 0..3 {
    //         for j in 0..3 {
    //             let a = product[(i, j)];
    //             let b = identity[(i, j)];
    //             assert!((a - b).abs() < 1e-8, "Mismatch at ({}, {}): got {}, expected {}", i, j, a, b);
    //         }
    //     }
    // }
}

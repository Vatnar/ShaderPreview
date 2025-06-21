//! [`MatrixView`] type
use crate::Matrix;
use std::ops::RangeInclusive;

/// Used to virtually represent a submatrix of a given parent matrix
pub struct MatrixView<'a> {
    parent: &'a Matrix,
    row_range: RangeInclusive<usize>,
    col_range: RangeInclusive<usize>,
}
impl<'a> MatrixView<'a> {
    /// Creates a new [`MatrixView`]
    ///
    /// # Arguments
    /// * `parent` - Reference to parent [`Matrix`]
    /// * `row_range` - Inclusive range of rows
    /// * `col_range` - Inclusive range of columns
    pub fn new(
        parent: &'a Matrix,
        row_range: RangeInclusive<usize>,
        col_range: RangeInclusive<usize>,
    ) -> Self {
        MatrixView {
            parent,
            row_range,
            col_range,
        }
    }

    /// Clones parent matrix and extracts a submatrix
    pub fn to_matrix(&self) -> Matrix {
        self.parent
            .submatrix(self.row_range.clone(), self.col_range.clone())
    }
}

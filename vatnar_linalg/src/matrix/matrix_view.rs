use crate::Matrix;
use std::ops::RangeInclusive;

pub struct MatrixView<'a> {
    parent: &'a Matrix,
    row_range: RangeInclusive<usize>,
    col_range: RangeInclusive<usize>,
}
impl<'a> MatrixView<'a> {
    pub fn clone(&self) -> Matrix {
        self.parent
            .submatrix(self.row_range.clone(), self.col_range.clone())
    }
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
}

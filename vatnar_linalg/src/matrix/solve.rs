//! Heavier [`Matrix`] operations regarding solving the linear system
use super::Matrix;
use row::Row;

mod row;
// TODO refactoring heavily needed

impl Matrix {
    /// Computes the inverse of the matrix using [`Matrix::reduced_echelon`]
    pub fn inverse(&self) -> Self {
        assert_eq!(self.rows, self.cols); // Wont make sense if not i think TODO check this up

        // row number * "0" + 1 +  total row - row number * "0"
        let mut out_data = Vec::with_capacity(self.rows * (self.cols + self.rows));

        for row in 0..self.rows {
            let start = row * self.cols;
            let end = start + self.cols;
            out_data.extend_from_slice(&self.data[start..end]);

            for col in 0..self.rows {
                if row == col {
                    out_data.push(1.0);
                } else {
                    out_data.push(0.0);
                }
            }
        }

        let mut out = self.clone();
        out.data = out_data;
        out.cols = self.cols + self.rows;
        let out = out.reduced_echelon(out.cols - self.cols);

        out.submatrix(1..=out.rows, self.cols + 1..=out.cols)
            .truncate(5)
    }

    /// Reduces given matrix to echelon form \
    /// use [`Matrix::echelon_aug`] for reducing with aug
    pub fn echelon(&self) -> Self {
        self.echelon_aug(0)
    }

    /// Reduces given matrix to echelon_form
    /// # Arguments
    /// * `augmented_size` - Amount of columns of augmented part, 0 if no augmented part
    pub fn echelon_aug(&self, augmented_size: usize) -> Self {
        let mut rows = self.extract_rows();

        // Do this for every column (except augmented_size at end)
        // start solve (forward process)

        let mut completed_rows = 0; // dont calculate for "finished rows" in this step
        for current_col in 0..self.cols - augmented_size {
            if completed_rows == self.rows {
                break;
            }
            let mut highest_row = (completed_rows, rows[completed_rows][current_col].abs());

            for (i, _) in rows
                .iter()
                .enumerate()
                .take(self.rows)
                .skip(completed_rows + 1)
            {
                if rows[i][current_col].abs() > highest_row.1.abs() {
                    highest_row = (i, rows[i][current_col]);
                }
            }
            // found highest row
            rows.swap(completed_rows, highest_row.0);
            let pivot = rows[completed_rows][current_col];
            if pivot.abs() < f64::EPSILON {
                continue;
            }

            for i in completed_rows + 1..self.rows {
                let factor = rows[i][current_col] / pivot;

                rows[i] = rows[i].clone() - factor * rows[completed_rows].clone();
            }
            completed_rows += 1;
        }
        // rows should now be in echelon form

        let matrix = Matrix::from_rows(rows);
        matrix.truncate(5)
    }

    /// Creates a matrix from a vector of [`Row`]
    fn from_rows(rows: Vec<Row>) -> Matrix {
        let mut matrix = Matrix::empty(rows.len(), rows[0].len());
        for row in rows {
            matrix.data.extend(row.values);
        }

        matrix
    }

    /// Creates a vector of [`Row`] from Matrix
    fn extract_rows(&self) -> Vec<Row> {
        let mut rows: Vec<Row> = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            let start = i * self.cols;
            let end = start + self.cols;
            let slice = self.data[start..end].to_vec();

            rows.push(Row { values: slice })
        }
        rows
    }

    /// Reduces given matrix to reduced echelon form \
    /// use [`Matrix::echelon`] for echelon form
    ///
    /// # Arguments
    /// * `augmented_size` - Amount of columns of augmented part, 0 if no augmented part
    pub fn reduced_echelon(&self, augmented_size: usize) -> Self {
        let mut rows = self.echelon_aug(augmented_size).extract_rows();

        // normalize
        for row in &mut rows {
            if let Some((_, &lead)) = row
                .iter()
                .enumerate()
                .find(|&(_, &x)| x.abs() > f64::EPSILON)
            {
                let scale = 1.0 / lead;
                for val in row.values.iter_mut() {
                    *val *= scale;
                }
            }
        }

        for pivot_row_idx in 0..rows.len() {
            let (before, after) = rows.split_at_mut(pivot_row_idx);
            let (pivot_row, rest) = after.split_first_mut().unwrap();

            let pivot_col = match pivot_row.iter().position(|&x| x.abs() > f64::EPSILON) {
                Some(i) => i,
                None => continue,
            };

            for target_row in before.iter_mut().chain(rest.iter_mut()) {
                let factor = target_row[pivot_col];
                for j in 0..pivot_row.len() {
                    target_row[j] -= factor * pivot_row[j];
                }
            }
        }

        Self::from_rows(rows).truncate(5)
    }
}

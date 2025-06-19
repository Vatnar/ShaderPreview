use super::Matrix;

impl Matrix {
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
}

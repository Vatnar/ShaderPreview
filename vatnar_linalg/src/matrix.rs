use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    columns: usize,
}

// Allows access to public vec functions
impl Deref for Matrix {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
#[macro_export]
macro_rules! to_f64 {
    ($($x:expr),* $(,)?) => {
        &[$($x as f64),*]
    };
}
impl Matrix {
    pub fn empty(rows: usize, columns: usize) -> Self {
        let data: Vec<f64> = Vec::with_capacity((rows * columns) as usize);
        Matrix {
            data,
            rows,
            columns,
        }
    }

    pub fn new(rows: usize, columns: usize, array: &[f64]) -> Self {
        let mut matrix = Self::empty(rows, columns);
        matrix.insert(array);
        matrix
    }
}

impl Matrix {
    pub fn insert(&mut self, array: &[f64]) {
        assert_eq!(self.rows * self.columns, array.len());
        self.data = array.into();
    }
    pub fn inverse(&self) -> Self {
        // TODO Augment matrix by identity matrix, then transform matrix into identity,
        // that will give augmented part = inverse
        // TODO Implement solving first, consider having it "modular" so it works with
        // bigger augmented matrix, or maybe specify what is augmented and not, or perhaps submatrix
        self.clone()
    }
}

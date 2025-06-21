//! Internal row type for representing rows in a matrix
use std::ops::{Deref, Mul, Sub};
use std::ops::{Index, IndexMut};

/// Used internally for representing rows, not the fastest but good for now TODO
#[derive(Clone)]
pub(super) struct Row {
    pub(crate) values: Vec<f64>,
}
impl Deref for Row {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl Mul<f64> for Row {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Row {
            values: self.values.into_iter().map(|v| v * rhs).collect(),
        }
    }
}
impl Mul<Row> for f64 {
    type Output = Row;

    fn mul(self, rhs: Row) -> Self::Output {
        Row {
            values: rhs.values.into_iter().map(|v| v * self).collect(),
        }
    }
}
impl Sub for Row {
    type Output = Row;

    fn sub(self, rhs: Row) -> Row {
        assert_eq!(self.values.len(), rhs.values.len(), "Row length mismatch");

        let values = self
            .values
            .into_iter()
            .zip(rhs.values)
            .map(|(a, b)| a - b)
            .collect();

        Row { values }
    }
}

impl Index<usize> for Row {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<usize> for Row {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

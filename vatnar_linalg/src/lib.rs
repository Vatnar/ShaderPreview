//! Linear algebra library by Vatnar made for learning (not the most important)
//! supplies Matrix operations, points, vectors, transformations, etc
pub use matrix::{Matrix, matrix_view::MatrixView};
pub use point::Point2;
pub use scalar::Scalar;
pub use vector::Vector2;

pub mod matrix;
pub mod point;
pub mod vector;

mod scalar;

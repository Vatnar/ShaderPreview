#![allow(unused)]
pub use point::Point2;
pub use vector::Vector2;
mod scalar;

pub mod vector {
    pub use vector2::Vector2;

    mod vector2;
}
pub mod point {
    pub use point2::Point2;
    mod point2;
}

mod matrix;

#[cfg(test)]
mod tests {
    use super::*;
    use matrix::Matrix;

    #[test]
    fn test_matrix() {
        let mut m = Matrix::empty(2, 2);

        assert_eq!(m.len(), 0);
        m.insert(&[0.0, 0.0, 1.0, 0.0]);

        assert_eq!(m.capacity(), 4);

        let m = to_f64!(1, 2, 3, 4, 5, 5, 4, 3, 2, 3, -4, 3, 3, 1, 3, -4,);

        let m = Matrix::new(4, 4, m);
        assert_eq!(m.len(), 16);
        assert_eq!(m.capacity(), 16);

        let m2 = m.inverse();
    }
}

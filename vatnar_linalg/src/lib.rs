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
pub use matrix::{Matrix, MatrixView};
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

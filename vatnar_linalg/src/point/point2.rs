//! [`Point2`] type
use crate::Vector2;
use crate::scalar::Scalar;
use num_traits::CheckedSub;
mod trait_impl;

/// Represents a point in 2d space
/// # Members
/// * `x` and `y` - coordinates of point, of type T which implements [`Scalar`]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Point2<T: Scalar> {
    pub x: T,
    pub y: T,
}

impl<T> Point2<T>
where
    T: Scalar + CheckedSub,
{
    /// Performs a checked subtraction of two points
    /// Only works on points on types that implement [`CheckedSub`]
    ///
    /// # Arguments
    /// * `rhs` - point to subtract
    pub fn checked_sub(self, rhs: Vector2<T>) -> Option<Point2<T>> {
        Some(Point2::new(
            self.x.checked_sub(&rhs.x)?,
            self.y.checked_sub(&rhs.y)?,
        ))
    }
}

impl<T> From<(T, T)> for Point2<T>
where
    T: Scalar,
{
    /// Create point from tuple
    fn from(tuple: (T, T)) -> Self {
        Point2 {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T> From<Point2<T>> for (T, T)
where
    T: Scalar,
{
    /// Create tuple of point
    fn from(val: Point2<T>) -> Self {
        (val.x, val.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2_vector2_interaction() {
        let mut p = Point2::new(5, 3);
        let v = (34, 23).into();
        p += v;
        assert_eq!(p, (39, 26).into());

        let mut p = Point2::new(5, 3);
        p += v;
        assert_eq!(p, (39, 26).into());
    }
}

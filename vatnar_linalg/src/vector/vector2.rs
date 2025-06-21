//! [`Vector2`] type
mod trait_impl;

use crate::scalar::Scalar;
use num_traits::{CheckedSub, Float};

/// Holds a vector with coordinates `x`, `y` that implements [`Scalar`]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vector2<T: Scalar> {
    pub x: T,
    pub y: T,
}

impl<T: Scalar> Vector2<T> {
    /// Creates a new vector from coordinates
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x, y }
    }
}

impl<T> Vector2<T>
where
    T: Float,
{
    /// Returns magnitude of vector
    pub fn mag(self) -> T {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Returns dot product of two vectors
    pub fn dot(self, v: Self) -> T {
        self.x * v.x + self.y * v.y
    }

    /// Returns angle between two vectors
    pub fn angle(self, v: Self) -> T {
        (self.dot(v) / (self.mag() * v.mag())).acos()
    }

    /// Returns magnitude by using dot product
    pub fn dot_mag(u: T, v: T, angle: T) -> T {
        u * v * angle.sin()
    }
}

impl<T: Scalar + CheckedSub> Vector2<T> {
    /// subtraction for unsigned types
    pub fn checked_sub(self, rhs: Self) -> Option<Vector2<T>> {
        Some(Vector2::new(
            self.x.checked_sub(&rhs.x)?,
            self.y.checked_sub(&rhs.y)?,
        ))
    }
}

impl<T> From<(T, T)> for Vector2<T>
where
    T: Scalar,
{
    /// Create Vector from tuple
    fn from(tuple: (T, T)) -> Self {
        Vector2 {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T> From<Vector2<T>> for (T, T)
where
    T: Scalar,
{
    /// create tuple from vector
    fn from(val: Vector2<T>) -> Self {
        (val.x, val.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //use paste::paste;

    #[test]
    fn test_vector2_from_tuple() {
        let v = Vector2::from((32, 24));
        println!("{v}");
    }

    #[test]
    fn test_vector2_add() {
        let a = Vector2::new(5, 2);
        let b = Vector2::new(-3, 4);
        let c = a + b;
        assert_eq!(c, Vector2::new(2, 6));

        let c = a + (-3, 4).into();
        assert_eq!(c, Vector2::new(2, 6));
    }
    #[test]
    fn test_vector2_addassign() {
        let mut a = Vector2::new(1, 2);
        let b = Vector2::new(3, 2);
        a += b;
        assert_eq!(a, Vector2::new(4, 4))
    }

    #[test]
    fn test_vector2_sub() {
        // signed
        let a = Vector2::new(5, 2);
        let b = Vector2::new(-3, 4);
        let c = a - b;
        assert_eq!(c, Vector2::new(8, -2));

        // unsigned
        let a = Vector2::new(5u32, 2u32);
        let b = Vector2::new(3u32, 1u32);
        let c = a.checked_sub(b);
        assert!(c.is_some());
        assert_eq!(c.unwrap(), Vector2::new(2, 1));

        let a = Vector2::new(5u32, 2u32);
        let b = Vector2::new(3u32, 4u32);
        let c = a.checked_sub(b);
        assert!(c.is_none());
    }

    #[test]
    fn test_vector2() {
        let v = Vector2::new(1.0, 1.0);

        assert!((v.mag() - 2.0.sqrt()).abs() < 1e-4);
        assert!((v.angle((0.0, 1.0).into()) - 45.0.to_radians()).abs() < 1e-4);

        let dot = v.dot((0.0, 1.0).into());
        let dot_mag = Vector2::dot_mag(v.mag(), 1.0, 45.0.to_radians());
        assert!((dot_mag - dot).abs() < 1e-4);
        assert!((dot - 1.0).abs() < 1e-4);
    }
}

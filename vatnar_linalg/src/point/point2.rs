use crate::Vector2;
use crate::scalar::Scalar;
use num_traits::{CheckedSub, Signed};
use std::fmt::{Display, Formatter};
use std::{fmt, ops};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Point2<T: Scalar> {
    pub x: T,
    pub y: T,
}

impl<T: Display + Scalar> Display for Point2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> Point2<T>
where
    T: Scalar,
{
    pub fn new(x: T, y: T) -> Point2<T> {
        Point2 { x, y }
    }
}

impl<T> ops::Add<Vector2<T>> for Point2<T>
where
    T: Scalar,
{
    type Output = Point2<T>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Point2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl<T> ops::AddAssign<Vector2<T>> for Point2<T>
where
    T: Scalar,
{
    fn add_assign(&mut self, rhs: Vector2<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T> ops::Sub<Vector2<T>> for Point2<T>
where
    T: Signed + Scalar,
{
    type Output = Point2<T>;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        Point2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl<T> ops::SubAssign<Vector2<T>> for Point2<T>
where
    T: Scalar + Signed,
{
    fn sub_assign(&mut self, rhs: Vector2<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.x;
    }
}

impl<T> Point2<T>
where
    T: Scalar + CheckedSub,
{
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

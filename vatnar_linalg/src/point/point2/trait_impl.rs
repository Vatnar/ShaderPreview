use crate::scalar::Scalar;
use crate::{Point2, Vector2};
use num_traits::Signed;
use std::fmt::{Display, Formatter};
use std::{fmt, ops};

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

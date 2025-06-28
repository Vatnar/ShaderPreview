use crate::Vector2;
use crate::scalar::Scalar;
use num_traits::Signed;
use std::fmt::{Display, Formatter};
use std::ops::Mul;
use std::{fmt, ops};

impl<T: Display + Scalar> Display for Vector2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<T> ops::Add<Self> for Vector2<T>
where
    T: Scalar,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> ops::AddAssign<Self> for Vector2<T>
where
    T: Scalar,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}
impl<T> ops::Sub for Vector2<T>
where
    T: Scalar + Signed,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> ops::SubAssign for Vector2<T>
where
    T: Scalar + Signed,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.x;
    }
}

impl Mul<f64> for Vector2<i32> {
    type Output = Vector2<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2::new(self.x as f64 * rhs, self.y as f64 * rhs)
    }
}

impl Mul<f64> for Vector2<f64> {
    type Output = Vector2<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<i32> for Vector2<i32> {
    type Output = Vector2<i32>;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl<T, S> ops::MulAssign<S> for Vector2<T>
where
    T: Scalar + ops::Mul<S, Output = T>,
    S: Copy,
{
    fn mul_assign(&mut self, scalar: S) {
        self.x = self.x * scalar;
        self.y = self.y * scalar;
    }
}

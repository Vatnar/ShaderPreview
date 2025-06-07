use num_traits::Num;
use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar:
    Num
    + Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialOrd
    + PartialEq
{
}

impl<T> Scalar for T where
    T: Num
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + PartialOrd
        + PartialEq
{
}

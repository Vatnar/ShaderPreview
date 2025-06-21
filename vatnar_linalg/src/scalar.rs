//! [`Scalar`] trait
use num_traits::Num;
use std::ops::{Add, Div, Mul, Sub};

/// [`Scalar`] trait to specify what types a vector or point can be created with \
/// Required traits: \
/// [`Copy`] \
/// [`Add`] \
/// [`Sub`] \
/// [`Mul`] \
/// [`Div`] \
/// [`PartialOrd`] \
/// [`PartialEq`]
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
/// Blanket implementation, implement on all types that implement traits: \

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

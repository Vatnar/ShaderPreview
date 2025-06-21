//! macros for interacting with [`Matrix`], [`to_f64`]
/// Converts list of numbers to a slice of [`std::f64`]
///
/// Useful for creating slices for [`Matrix::insert`] and [`Matrix::new`]
/// # Examples
/// ```
/// use vatnar_linalg::to_f64;
/// let m = to_f64!(
///             1, 2, 3, 4,
///             5, 5, 4, 3,
///             2, 3, -4, 3,
///             3, 1, 3, -4,);
/// ```
#[macro_export]
macro_rules! to_f64 {
    ($($x:expr),* $(,)?) => {
        &[$($x as f64),*]
    };
}

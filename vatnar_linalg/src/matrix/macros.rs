#[macro_export]
macro_rules! to_f64 {
    ($($x:expr),* $(,)?) => {
        &[$($x as f64),*]
    };
}

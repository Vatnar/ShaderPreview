use num_traits::Num;

pub struct Vector2<T: Num + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Copy> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x, y }
    }
}

// region operators

// endregion

// region Conversions
impl<T> From<(T, T)> for Vector2<T>
where
    T: Copy + Num,
{
    fn from(tuple: (T, T)) -> Self {
        Vector2 {
            x: tuple.0,
            y: tuple.1,
        }
    }
}
// endregion

#[cfg(test)]
mod tests {
    use super::*;

    //use paste::paste;

    #[test]
    fn test_vector2_from_tuple() {
        let v = Vector2::from((32.3, 24.3));
    }
}

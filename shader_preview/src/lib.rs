use crate::WindowError::InvalidSize;
use vatnar_linalg::Vector2;
pub mod error;

// Window setup, event handling, imgui stuff

// Linalg library
// Rendering

use error::WindowError;
pub fn create_window<T>(size: T) -> Result<(), WindowError>
where
    T: Into<Vector2<u32>>,
{
    let size = size.into();
    if size.x > 1000 || size.y > 1000 {
        Err(InvalidSize)
    } else {
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn test_create_window_toobig() {
        create_window((1100, 300)).unwrap();
    }
}

use crate::WindowError::InvalidSize;
use vatnar_linalg::Vector2;
// Window setup, event handling, imgui stuff

// Linalg library
// Rendering

pub mod error {
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    pub enum Error {
        WindowError(WindowError),
    }
    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::WindowError(inner) => write!(f, "Window Error: {} ", inner),
            }
        }
    }
    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                Error::WindowError(inner) => Some(inner),
            }
        }
    }

    #[derive(Debug)]
    pub enum WindowError {
        InvalidSize,
    }

    impl Display for WindowError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                WindowError::InvalidSize => write!(f, "Invalid size of window"),
            }
        }
    }
    impl std::error::Error for WindowError {}

    impl From<WindowError> for Error {
        fn from(err: WindowError) -> Self {
            Error::WindowError(err)
        }
    }
}

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

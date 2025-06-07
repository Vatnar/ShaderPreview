use std::fmt::{Display, Formatter};
use std::panic::Location;

#[derive(Debug)]
pub enum ShaderPreviewError {
    WindowError(WindowError),
    OutOfBoundsError {
        message: String,
        location: &'static Location<'static>,
    },
}
impl ShaderPreviewError {
    #[track_caller]
    pub fn out_of_bounds(msg: impl Into<String>) -> Self {
        ShaderPreviewError::OutOfBoundsError {
            message: msg.into(),
            location: Location::caller(),
        }
    }
}
impl Display for ShaderPreviewError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderPreviewError::WindowError(inner) => write!(f, "Window Error: {} ", inner),
            ShaderPreviewError::OutOfBoundsError { message, location } => {
                write!(
                    f,
                    "Value went out of bounds: {} at {}:{}:{}",
                    message,
                    location.file(),
                    location.line(),
                    location.column()
                )
            }
        }
    }
}
impl std::error::Error for ShaderPreviewError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ShaderPreviewError::WindowError(inner) => Some(inner),
            ShaderPreviewError::OutOfBoundsError {
                message: _,
                location: _,
            } => None,
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

impl From<WindowError> for ShaderPreviewError {
    fn from(err: WindowError) -> Self {
        ShaderPreviewError::WindowError(err)
    }
}

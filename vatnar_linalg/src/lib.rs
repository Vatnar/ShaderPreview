pub use point::Point2;
pub use vector::Vector2;

mod scalar;

pub mod vector {
    pub use vector2::Vector2;

    mod vector2;
}
pub mod point {
    pub use point2::Point2;
    mod point2;
}

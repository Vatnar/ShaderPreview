use shader_preview::*;
use vatnar_linalg::Vector2;

fn main() -> Result<(), error::Error> {
    let size = Vector2::new(800, 600);
    create_window(size)?;

    Ok(())
}

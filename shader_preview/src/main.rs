use shader_preview::create_window;
use shader_preview::error::ShaderPreviewError;
use vatnar_linalg::{Point2, Vector2};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), ShaderPreviewError> {
    let size = Vector2::new(800, 600);
    create_window(size)?;

    let v: Vector2<f64> = Vector2::new(2.2, 3.3);

    let _u: Vector2<u32> = (2, 2).into();

    let mut origin = Point2::new(0.0, 0.0);

    origin += v * 3f64;

    let screen_pos: Point2<u32> = (600, 300).into();

    if let Some(screen_pos) = screen_pos.checked_sub((300, 300).into()) {
        println!("{screen_pos}")
    } else {
        return Err((ShaderPreviewError::out_of_bounds("Out of screen")));
    }

    Ok(())
}

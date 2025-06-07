use shader_preview::*;
use vatnar_linalg::{Point2, Vector2};

fn main() -> Result<(), error::Error> {
    let size = Vector2::new(800, 600);
    create_window(size)?;

    let v: Vector2<f64> = Vector2::new(2.2, 3.3);

    let _u: Vector2<u32> = (2, 2).into();

    let mut origin = Point2::new(0.0, 0.0);

    origin += v * 3f64;

    let screenpos: Point2<u32> = (300, 300).into();

    let screenpos = screenpos.checked_sub((300, 400).into());
    match screenpos {
        Some(t) => println!("Valid op"),
        None => println!("Out of bounds"),
    }

    Ok(())
}

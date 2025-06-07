use shader_preview::error::ShaderPreviewError;
use shader_preview::runer;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), ShaderPreviewError> {
    runer();
    Ok(())
}

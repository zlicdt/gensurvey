mod model;
mod render;
mod templates;
mod scripts;

use std::fs::File;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // Read survey spec path from args or default to ../example.jsonc
    let spec_path: PathBuf = std::env::args().nth(1).map(Into::into).unwrap_or_else(|| {
        PathBuf::from("../example.jsonc")
    });
    eprintln!("Loading survey spec: {}", spec_path.display());
    let file = File::open(&spec_path)?;
    let survey = model::Survey::from_reader(file)?;
    let html = render::render_full_html(&survey);
    let out_dir = PathBuf::from("dist");
    std::fs::create_dir_all(&out_dir)?;
    let out_file = out_dir.join("index.html");
    std::fs::write(&out_file, html)?;
    // Write external JS
    std::fs::write(out_dir.join("script.js"), scripts::SURVEY_SCRIPT)?;
    eprintln!("Generated: {}", out_file.display());
    eprintln!("You can serve the 'dist' directory with nginx or any static server.");
    Ok(())
}

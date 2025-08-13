mod model;
mod render;
mod templates;
mod scripts;

use std::fs::File;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // CLI parsing: require -i/--input <path>. Provide help if missing.
    // CLI parsing: require -o/--output <path>. Provide help if missing.
    let mut args = std::env::args().skip(1);
    let mut input: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-i" | "--input" => {
                if let Some(p) = args.next() { input = Some(PathBuf::from(p)); }
                else {
                    eprintln!("Error: -i/--input expects a path argument.\n");
                    print_usage();
                    std::process::exit(1);
                }
            }
            "-o" | "--output" => {
                if let Some(p) = args.next() { output = Some(PathBuf::from(p)); }
                else {
                    eprintln!("Error: -o/--output expects a directory path.\n");
                    print_usage();
                    std::process::exit(1);
                }
            }
            "-h" | "--help" => {
                print_usage();
                return Ok(());
            }
            other => {
                eprintln!("Unknown argument: {other}\n");
                print_usage();
                std::process::exit(1);
            }
        }
    }
    let spec_path = input.unwrap_or_else(|| {
        print_usage();
        std::process::exit(1);
    });
    let out_dir = output.unwrap_or_else(|| {
        print_usage();
        std::process::exit(1);
    });
    eprintln!("Loading survey spec: {}", spec_path.display());
    let file = File::open(&spec_path)?;
    let survey = model::Survey::from_reader(file)?;
    let html = render::render_full_html(&survey);
    // Prepare output directory: must not already exist to avoid conflicts.
    if out_dir.exists() {
        anyhow::bail!("Output directory already exists: {}", out_dir.display());
    }
    std::fs::create_dir_all(&out_dir)?;
    let out_file = out_dir.join("index.html");
    std::fs::write(&out_file, html)?;
    // Write external JS
    std::fs::write(out_dir.join("script.js"), scripts::SURVEY_SCRIPT)?;
    eprintln!("Generated: {}", out_file.display());
    eprintln!("You can serve the '{}' directory with nginx or any static server.", out_dir.display());
    Ok(())
}

fn print_usage() {
    eprintln!("Usage: gensurvey -i <survey.jsonc> -o <output_dir>\n\nGenerate static survey HTML into a new output directory. The output directory must not already exist.\n\nOptions:\n  -i, --input <PATH>        Path to survey spec file (.json or .jsonc)\n  -o, --output <DIR>        Output directory to create (must not exist)\n  -h, --help                Show this help message\n");
}

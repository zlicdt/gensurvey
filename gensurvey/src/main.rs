//
//  main.rs
//  gensurvey
//
//  Created by zlicdt on 2025/8/13.
//  Copyright (c) 2025 zlicdt. All rights reserved.
//
//  This file is part of gensurvey.
//
//  gensurvey is free software: you can redistribute it and/or modify
//  it under the terms of the GNU Affero General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  gensurvey is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU Affero General Public License for more details.
//
//  You should have received a copy of the GNU Affero General Public License
//  along with gensurvey. If not, see <https://www.gnu.org/licenses/>.
//
mod model;
mod render;
mod templates;
mod scripts;

use std::fs::File;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name="gensurvey",
    version,
    about="Generate static survey HTML from a JSON/JSONC spec",
    after_help = "Please refer to https://github.com/zlicdt/gensurvey/blob/main/scaffold/example.jsonc for the format."
)] 
struct Args {
    /// Path to survey spec file (.json / .jsonc)
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    input: PathBuf,

    /// Output directory to create (must not already exist)
    #[arg(short = 'o', long = "output", value_name = "DIR")]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let spec_path = args.input;
    let out_dir = args.output;
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

// Legacy print_usage removed; clap provides help.

mod grrs;
mod wcs;

use anyhow::{Context, Result};
use clap::Parser;
use grrs::model::grrs;
use wcs::model::wcs;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    
    if &args.pattern == "wcs" {
        wcs(&content);
    } else {
        grrs(&args.pattern, &content);
    }

    Ok(())
}

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Which command to run (grrs or wcs)
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Displays the lines that contain the given pattern.
    Grrs(GrrsCommand),

    /// Displays the line count and word count of a file.
    Wcs(WcsCommand),
}

#[derive(Debug, Args)]
pub struct GrrsCommand {
    /// Pattern you want to look for in the file.
    pub pattern: String,

    /// File path you want to look in.
    pub path: std::path::PathBuf,
}

#[derive(Debug, Args)]
pub struct WcsCommand {
    /// File path you want to analyze.
    pub path: std::path::PathBuf,
}

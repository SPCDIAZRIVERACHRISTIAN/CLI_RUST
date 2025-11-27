mod args;
mod grrs;
mod wcs;

use clap::Parser;
use crate::args::{Cli, Command};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Grrs(cmd) => {
            grrs::run(&cmd.pattern, &cmd.path)?
        }

        Command::Wcs(cmd) => {
            wcs::run(&cmd.path)?;
        }
    }
    Ok(())
}

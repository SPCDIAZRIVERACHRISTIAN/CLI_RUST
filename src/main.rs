mod args;
mod grrs;
mod wcs;

use crate::args::{Cli, Command};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Grrs(cmd) => grrs::run(&cmd.pattern, &cmd.path)?,

        Command::Wcs(cmd) => {
            wcs::run(&cmd.path)?;
        }
    }
    Ok(())
}

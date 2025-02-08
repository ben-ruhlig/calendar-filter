mod api;
mod cli;
mod config;
use anyhow::{Context, Result};
pub use api::canvas;

pub struct Args {
    pub setup: bool,
    pub sync: bool,
}

pub fn run(args: Args) -> Result<()> {
    if args.setup {
        cli::setup::run().context("setup failed.")?;
    }
    if args.sync {
        cli::sync::run().context("setup failed.")?;
    }
    Ok(())
}

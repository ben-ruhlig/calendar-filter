mod cli;
mod utils;
pub use utils::args::CliArgs;
pub use utils::canvas;
pub use utils::config;
use anyhow::{Context, Result};

pub fn run(args: CliArgs) -> Result<()> {
    if args.setup {
        let _ = cli::setup::run().context("setup failed.");
    }
    if args.sync {
        println!("Syncing courses");
    }
    Ok(())
}

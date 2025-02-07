mod args;
pub use args::CliArgs;
use std::error::Error;

pub fn run(args: CliArgs) -> Result<(), Box<dyn Error>> {
    if args.setup {
        println!("Initial setup");
    }
    if args.sync {
        println!("Syncing courses");
    }
    Ok(())
}
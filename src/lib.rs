mod setup;
mod utils;
pub use utils::args::CliArgs;
pub use utils::canvas;
pub use utils::config;

pub fn run(args: CliArgs) {
    if args.setup {
        setup::run();
    }
    if args.sync {
        println!("Syncing courses");
    }
}

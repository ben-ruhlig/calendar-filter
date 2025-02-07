use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "Canvas Calendar CLI",
    version = "1.0",
    author = "Benjamin Ruhlig <benjamin.ruhlig@gmail.com>",
    about = "Filters and syncs Canvas calendar events"
)]
pub struct CliArgs {
    #[arg(long, help = "Initial setup with Canvas API token")]
    pub setup: bool,

    #[arg(long, help = "Sync canvas courses")]
    pub sync: bool,
}

fn main() {
    let cliargs = CliArgs::parse();
    let args = canvas_calender_cli::Args {
        setup: cliargs.setup,
        sync: cliargs.sync
    };
    canvas_calender_cli::run(args).unwrap();
}

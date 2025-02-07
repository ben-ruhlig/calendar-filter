use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "Canvas Calendar CLI",
    version = "1.0",
    author = "Benjamin Ruhlig <benjamin.ruhlig@gmail.com>",
    about = "Filters and syncs Canvas calendar events"
)]
struct Args {
    #[arg(long, help = "Initial setup with Canvas API token")]
    setup: bool,

    #[arg(long, help = "Sync canvas courses")]
    sync: bool,
}

fn main() {
    let args = Args::parse();
    println!("Arg Setup: {:#?}", args.setup);
    println!("Arg Sync: {:#?}", args.sync);
}
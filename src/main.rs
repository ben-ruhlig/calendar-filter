use clap::Parser;

fn main() {
    let args = canvas_calender_cli::CliArgs::parse();
    let _ = canvas_calender_cli::run(args);
}
use clap::Parser;

fn main() {
    let args = canvas_calender_cli::CliArgs::parse();
    canvas_calender_cli::run(args).unwrap();
}

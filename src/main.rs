use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(
    name = "Canvas CLI",
    version = env!("CARGO_PKG_VERSION"),
    author = "Benjamin Ruhlig <benjamin.ruhlig@gmail.com>",
    about = "CanvasCLI is a powerful tool for UPenn students to interact with the Canvas LMS",
    long_about = "A command-line interface for managing Canvas LMS calendars and courses. \
                  Supports filtering events, publishing calendar feeds, and automated updates.",
    after_help = "For more information, visit: https://github.com/yourusername/canvascli",
    before_help = "⚡ Quick Start: Run 'canvascli setup' to begin"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Setup Canvas API connection
    Setup,
    
    /// Sync course data from Canvas API
    Sync,
    
    /// Course management commands
    Course {
        #[command(subcommand)]
        command: CourseCommands,
    },
    
    /// Calendar management commands
    Calendar {
        #[command(subcommand)]
        command: CalendarCommands,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum CourseCommands {
    /// List courses
    Ls {
        /// Show all courses including old/inactive
        #[arg(long)]
        all: bool,
        /// Show only courses with published feeds
        #[arg(long)]
        published: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum CalendarCommands {
    /// List calendar items
    Ls {
        /// Filter by course ID or name
        #[arg(long)]
        course: Option<String>,
        /// Show filtered calendar items
        #[arg(long)]
        filtered: bool,
    },
    /// Manage calendar filters (interactive UI)
    Filter,
    /// Publish calendar feeds
    Publish {
        /// Setup Google Drive API authentication
        #[arg(long)]
        setup: bool,
        /// Course ID or Course Name to publish
        #[arg(long)]
        course: Option<String>,
        /// Publish all active courses
        #[arg(long)]
        all: bool,
        /// Apply filters before publishing
        #[arg(long)]
        filtered: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Setup => {
            canvascli::run_setup().unwrap();
        }
        Commands::Sync => {
            canvascli::run_sync().unwrap();
        }
        Commands::Course { command } => {
            match command {
                CourseCommands::Ls { all, published } => {
                    canvascli::run_course_ls(*all, *published).unwrap();
                }
            }
        }
        Commands::Calendar { command } => {
            match command {
                CalendarCommands::Ls { course, filtered } => {
                    canvascli::run_calendar_ls(course, *filtered).unwrap();
                }
                CalendarCommands::Filter => {
                    canvascli::run_calendar_filter().unwrap();
                }
                CalendarCommands::Publish { setup, course, all, filtered } => {
                    canvascli::run_calendar_publish(*setup, course, *all, *filtered).unwrap();
                }
            }
        }
    }
}

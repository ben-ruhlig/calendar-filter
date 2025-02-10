//! Main entry point for the Canvas CLI application.
//!
//! This application provides command-line access to the Canvas LMS.
//! It supports functionalities such as setting up the API connection,
//! synchronizing courses, filtering calendar items, and publishing calendar feeds.

use clap::{Parser, Subcommand};

/// The main CLI structure.
///
/// This structure represents the complete set of command-line arguments supported
/// by the Canvas CLI. It uses the `clap` crate for argument parsing.
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
    /// The subcommand to execute.
    #[command(subcommand)]
    command: Commands,
}

/// Top-level commands available in the application.
///
/// This enum groups the primary functionalities like setup, synchronization,
/// course management, and calendar management.
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Setup the Canvas API connection.
    ///
    /// Use this command to configure or reconfigure the connection settings required
    /// to interact with the Canvas API.
    Setup,

    /// Synchronize course data from the Canvas API.
    ///
    /// This command triggers a sync operation that fetches the latest course data.
    Sync,

    /// Perform actions related to course management.
    Course {
        #[command(subcommand)]
        /// Subcommands for course management.
        command: CourseCommands,
    },

    /// Perform actions related to calendar management.
    Calendar {
        #[command(subcommand)]
        /// Subcommands for calendar management.
        command: CalendarCommands,
    },
}

/// Commands for managing courses.
#[derive(Subcommand, Debug, Clone)]
pub enum CourseCommands {
    /// List courses according to the specified filters.
    ///
    /// If no filter is provided, it returns only active courses.
    /// Active courese are those with course end dates in the past.
    Ls {
        /// Show all courses including old/inactive ones.
        #[arg(long)]
        all: bool,
        /// Show only courses with published calendar feeds.
        #[arg(long)]
        published: bool,
    },
}

/// Commands for managing calendars and publishing calendar feeds.
#[derive(Subcommand, Debug, Clone)]
pub enum CalendarCommands {
    /// List calendar items with filtering options.
    Ls {
        /// Filter calendar items by course ID or name.
        #[arg(long)]
        course: Option<String>,
        /// Show filtered calendar items.
        #[arg(long)]
        filtered: bool,
    },
    /// Launch the interactive UI for managing calendar filters.
    Filter,
    /// Publish calendar feeds.
    ///
    /// The publish command supports an optional setup subcommand
    /// and additional options to control which feeds to publish.
    Publish {
        /// Optional subcommand for publishing setup.
        ///
        /// If specified as "setup", initialization for publishing will occur.
        #[command(subcommand)]
        setup: Option<PublishSetupCommand>,
        /// The course ID or name for which to publish calendar feeds.
        #[arg(long)]
        course: Option<String>,
        /// Flag indicating that all active courses should be published.
        #[arg(long)]
        all: bool,
        /// Flag to apply filters before publishing calendar feeds.
        #[arg(long)]
        filtered: bool,
    },
}

/// Subcommand for the publish setup process.
///
/// Invoked when running "canvascli calendar publish setup".
#[derive(Subcommand, Debug, Clone)]
pub enum PublishSetupCommand {
    /// Run the publish setup.
    Setup,
}

/// Main entry point of the application.
///
/// This function parses the CLI arguments using `clap` and dispatches execution
/// according to the appropriate command.
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        // Executes the API setup command.
        Commands::Setup => {
            canvascli::run_setup().unwrap();
        }
        // Executes the sync command to fetch course data.
        Commands::Sync => {
            canvascli::run_sync().unwrap();
        }
        // Dispatches course subcommands.
        Commands::Course { command } => match command {
            CourseCommands::Ls { all, published } => {
                canvascli::run_course_ls(*all, *published).unwrap();
            }
        },
        // Dispatches calendar-related subcommands.
        Commands::Calendar { command } => match command {
            // Lists calendar items. Applies course filtering or shows all items.
            CalendarCommands::Ls { course, filtered } => {
                canvascli::run_calendar_ls(course, *filtered).unwrap();
            }
            // Launches the interactive filter UI.
            CalendarCommands::Filter => {
                canvascli::run_calendar_filter().unwrap();
            }
            // Handles the publish command.
            CalendarCommands::Publish {
                setup,
                course,
                all,
                filtered,
            } => {
                // If the "setup" subcommand is provided, run the publish setup.
                if let Some(PublishSetupCommand::Setup) = setup {
                    canvascli::run_calendar_publish_setup().unwrap();
                } else {
                    // Otherwise, process the publishing arguments.
                    canvascli::run_calendar_publish(course, *all, *filtered).unwrap();
                }
            }
        },
    }
}

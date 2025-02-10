mod api;
mod cli;
mod config;

use anyhow::{Context, Result};
pub use api::canvas;

pub struct Args {
    pub setup: bool,
    pub sync: bool,
}

pub fn run_setup() -> Result<()> {
    cli::setup::run().context("setup failed")
}

pub fn run_sync() -> Result<()> {
    cli::sync::run().context("sync failed")
}

pub fn run_course_ls(all: bool, published: bool) -> Result<()> {
    if all {
        cli::course::run_ls_all().context("course ls --all failed")
    } else if published {
        cli::course::run_published().context("course published failed")
    } else {
        cli::course::run_ls_active().context("course ls failed")
    }
}

pub fn run_calendar_ls(course: &Option<String>, filtered: bool) -> Result<()> {
    if filtered {
        cli::calendar::ls::run_filtered().context("calendar ls --filtered failed")
    } else if course.is_some() {
        cli::calendar::ls::run_course(course).context("calendar ls --course failed")
    } else {
        cli::calendar::ls::run_all().context("calendar ls failed")
    }
}

pub fn run_calendar_filter() -> Result<()> {
    cli::calendar::filter::run().context("calendar filter TUI failed")
}

pub fn run_calendar_publish(course: &Option<String>, all: bool, filtered: bool) -> Result<()> {
    if let Some(course_str) = course {
        cli::calendar::publish::publish_course(course_str)
            .context("publish course failed")
    } else if all {
        cli::calendar::publish::publish_all()
            .context("publish all failed")
    } else if filtered {
        cli::calendar::publish::publish_filtered()
            .context("publish filtered failed")
    } else {
        // Error is handled upstream here rather than inside publish.rs functions.
        Err(anyhow::anyhow!(
            "Invalid: Must provide a course argument or select a publish variant"
        ))
    }
}

pub fn run_calendar_publish_setup() -> Result<()> {
    cli::calendar::publish::publish_setup().context("publish setup failed")
}

pub fn run_calendar_publish_ls() -> Result<()> {
    cli::calendar::publish::publish_ls().context("publish setup failed")
}

/// Placeholder: Unpublish a published calendar feed.
pub fn run_calendar_unpublish(course: &Option<String>) -> Result<()> {
    if let Some(course) = course {
        cli::calendar::publish::unpublish_course(course).context("publish course failed")
    } else {
        println!("TODO: Implement better error message");
        println!("Invalid: Must provide a course by ID or Name");
        Ok(())
    }
}

/// Placeholder: Enable automatic sync and publish every 4 hours.
pub fn run_calendar_autoupdate_enable() -> Result<()> {
    println!("TODO: Implement auto-update enable (create cron job or schedule task)");
    Ok(())
}

/// Placeholder: Disable the scheduled auto-update task.
pub fn run_calendar_autoupdate_disable() -> Result<()> {
    println!("TODO: Implement auto-update disable (remove cron job or scheduled task)");
    Ok(())
}

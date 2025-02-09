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

pub fn run_calendar_publish(
    setup: bool,
    course: &Option<String>,
    all: bool,
    filtered: bool,
) -> Result<()> {
    if setup {
        cli::calendar::publish::publish_setup().context("publish setup failed")
    } else if course.is_some() {
        cli::calendar::publish::publish_course(course).context("publish course failed")
    } else if all {
        cli::calendar::publish::publish_all().context("publish all failed")
    } else if filtered {
        cli::calendar::publish::publish_filtered().context("publish filtered failed")
    } else {
        println!("Invalid: Must provide an argument");
        Ok(())
    }
}

use crate::canvas::CanvasClient;
use crate::config::{get_config_path, Config};
use anyhow::{Context, Ok, Result};
use std::fs;

pub fn run() -> Result<()> {
    let config_path = get_config_path().context("Failed to get config path")?;
    let config: Config = serde_json::from_str(
        &fs::read_to_string(&config_path).context("Failed to read config file")?,
    )
    .context("Failed to parse config file")?;

    let client = CanvasClient::new(config.api_token)?;
    let courses = client.get_courses().context("Failed to retrieve courses")?;

    // Alert user if there are no courses, returen.
    if courses.is_empty() {
        println!("You have no courses in canvas yet");
        return Ok(());
    }

    println!("Active courses:");
    courses
        .iter()
        .filter(|course| course.is_active())
        .for_each(|course| {
            println!("{}", course);
        });
    Ok(())
}

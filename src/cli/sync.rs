use crate::config::{get_config_path, Config};
use crate::canvas::CanvasClient;
use anyhow::{Context, Ok, Result};
use std::fs;

pub fn run () -> Result<()> {
    let config_path = get_config_path().context("Failed to get config path")?;
    let config: Config = serde_json::from_str(
        &fs::read_to_string(&config_path).context("Failed to read config file")?,
    )
    .context("Failed to parse config file")?;

    let client = CanvasClient::new(config.api_token)?;
    let courses = client.get_courses().context("Failed to retrieve courses")?;

    assert!(courses.len() != 0);

    // Debug: print out events to console
    for course in &courses {
        println!("{:#?}", course);
    }

    Ok(())
}
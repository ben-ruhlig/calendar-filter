use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Course {
    id: u64,
    name: String,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
    ics_cal: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_token: String,
    pub courses: Vec<Course>,
    pub last_sync: Option<String>,
}

pub fn get_config_path() -> Result<PathBuf> {
    // Try platform-specific config directory first
    if let Some(mut path) = dirs::config_dir() {
        path.push("canvas-calendar-cli");
        return Ok(path.join("config.json"));
    }

    // Fallback to current directory if config_dir() fails
    let mut path = std::env::current_dir().context("Failed to get current directory")?;
    path.push(".canvas-calendar-cli");
    fs::create_dir_all(&path).context("Failed to create config directory")?;
    Ok(path.join("config.json"))
}

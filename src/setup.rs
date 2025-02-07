use crate::config::{get_config_path, Config};
use anyhow::{Context, Result};
use std::fs;

pub fn run() -> Result<()> {
    println!("To set up the Canvas Calendar CLI, you'll need to generate an API token:");
    println!("1. Go to Canvas -> Account -> Settings");
    println!("2. Scroll to Approved Integrations");
    println!("3. Click '+ New Access Token'");
    println!("4. Enter a purpose (e.g., 'Calendar Sync') and click 'Generate Token'");
    println!("\nEnter your API token:");

    let mut token = String::new();
    std::io::stdin()
        .read_line(&mut token)
        .context("Failed to read API token")?;
    let token = token.trim().to_string();

    let config = Config {
        api_token: token,
        courses: Vec::new(),
        last_sync: None,
    };

    let config_path = get_config_path().context("Failed to get config path")?;
    fs::create_dir_all(config_path.parent().unwrap())
        .context("Failed to create config directory")?;
    fs::write(
        &config_path,
        serde_json::to_string_pretty(&config).context("Failed to serialize config")?,
    )
    .context("Failed to write config file")?;

    println!("Setup complete! Your API token has been saved.");
    Ok(())
}

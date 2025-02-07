use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command};
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tokio;
use tokio::task;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    api_token: String,
    calendar_filters: Vec<String>,
    last_sync: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Calendar {
    ics: String,
}

#[derive(Debug, Deserialize)]
struct Course {
    id: u64,
    name: String,
    start_at: Option<DateTime<Utc>>,
    end_at: Option<DateTime<Utc>>,
    calendar: Calendar,
}

struct CanvasClient {
    client: Client,
    base_url: String,
    config: Config,
}

impl CanvasClient {
    fn new(api_token: String) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("Bearer {}", api_token))
                .context("Failed to create authorization header")?,
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to build HTTP client")?;

        let config = Config {
            api_token,
            calendar_filters: Vec::new(),
            last_sync: None,
        };

        Ok(Self {
            client,
            base_url: "https://canvas.upenn.edu/api/v1".to_string(),
            config,
        })
    }

    async fn get_courses(&self) -> Result<Vec<Course>> {
        let url = format!("{}/courses", self.base_url);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send request")?
            .json::<Vec<Course>>()
            .await
            .context("Failed to parse courses")?;
        Ok(response)
    }

}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("Canvas Calendar CLI")
        .version("1.0")
        .author("Benjamin Ruhlig <benjamin.ruhlig@gmail.com>")
        .about("Filters and syncs Canvas calendar events")
        .arg(
            Arg::new("setup")
                .long("setup")
                .help("Initial setup with Canvas API token")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("sync")
                .long("sync")
                .help("Sync calendar events")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("setup") {
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
            calendar_filters: Vec::new(),
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
    }

    if matches.get_flag("sync") {
        let config_path = get_config_path().context("Failed to get config path")?;
        let config: Config = serde_json::from_str(
            &fs::read_to_string(&config_path).context("Failed to read config file")?,
        )
        .context("Failed to parse config file")?;

        let client = CanvasClient::new(config.api_token)?;

        // Spawn a task to fetch courses
        let courses_task = task::spawn(async move {
            client.get_courses().await
        });

        // Await the courses and sort them (right now, no benefit from async. future proof)
        let courses = courses_task.await.context("Failed to get courses")??;
        let sorted_courses = sort_courses(courses);

        // Debug: print out events to console
        println!("Fetched {} courses", sorted_courses.len());
        for course in &sorted_courses {
            println!("{:#?}", course);
        }

        let filters: Vec<String> = matches
            .get_many::<String>("filter")
            .unwrap_or_default()
            .cloned()
            .collect();

        let filtered_courses = client.filter_courses(sorted_courses, &filters);

        // Debug: print out filtered courses to console
        println!("Filtered {} courses", filtered_courses.len());
        for course in &filtered_courses {
            println!("{:#?}", course);
        }

        if matches.get_flag("publish") {
            let events: Vec<CalendarEvent> = vec![]; // Placeholder for actual events
            let ics_content = client.generate_ics(&events);
            let ics_path = "calendar.ics";
            fs::write(ics_path, ics_content).context("Failed to write .ics file")?;
            println!("Published calendar to {}", ics_path);
        }
    }

    Ok(())
}

fn sort_courses(mut courses: Vec<Course>) -> Vec<Course> {
    courses.sort_by(|a, b| {
        match (a.end_at, b.end_at) {
            (Some(end_a), Some(end_b)) => end_b.cmp(&end_a),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });
    courses
}

fn get_config_path() -> Result<PathBuf> {
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

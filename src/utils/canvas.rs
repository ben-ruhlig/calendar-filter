use crate::config::Config;
use crate::config::Course;
use anyhow::{Context, Result};
use std::time::Duration;
use ureq::Agent;

pub struct CanvasClient {
    agent: Agent,
    base_url: String,
    config: Config,
}

impl CanvasClient {
    fn new(api_token: String) -> Result<Self> {
        let agent = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(5)))
            .build()
            .into();

        let config = Config {
            api_token: api_token.clone(),
            courses: Vec::new(),
            last_sync: None,
        };

        Ok(Self {
            agent,
            base_url: "https://canvas.upenn.edu/api/v1".to_string(),
            config,
        })
    }

    fn get_courses(&self) -> Result<Vec<Course>> {
        let url = format!("{}/courses", self.base_url);
        let response: Vec<Course> = self
            .agent
            .get(&url)
            .header(
                "Authorization",
                &format!("Bearer {}", self.config.api_token),
            )
            .call()
            .context("Failed to send request")?
            .body_mut()
            .read_json::<Vec<Course>>()
            .context("Failed to parse courses")?;
        Ok(response)
    }
}

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct VelixConfig {
    pub api_url: String,
    pub api_key: String,
    pub timeout: Duration,
    pub max_retries: u32,
}

impl Default for VelixConfig {
    fn default() -> Self {
        Self {
            api_url: "https://api.velixbiometrics.com".to_string(),
            api_key: String::new(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }
}

impl VelixConfig {
    pub fn new(api_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            api_url: api_url.into(),
            api_key: api_key.into(),
            ..Default::default()
        }
    }
}

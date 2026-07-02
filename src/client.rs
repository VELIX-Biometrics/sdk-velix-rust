use reqwest::{header, Client};
use std::sync::Arc;

use crate::{
    config::VelixConfig,
    error::VelixError,
    modules::{checkin::CheckinModule, events::EventsModule, persons::PersonsModule, tenants::TenantsModule},
};

#[derive(Clone)]
pub struct VelixClient {
    pub(crate) http: Client,
    pub(crate) config: Arc<VelixConfig>,
}

impl VelixClient {
    pub fn new(config: VelixConfig) -> Result<Self, VelixError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "x-api-key",
            header::HeaderValue::from_str(&config.api_key)
                .map_err(|_| VelixError::Auth("Invalid API key format".to_string()))?,
        );

        let http = Client::builder()
            .timeout(config.timeout)
            .default_headers(headers)
            .user_agent("velix-rust-sdk/1.0.0")
            .build()?;

        Ok(Self {
            http,
            config: Arc::new(config),
        })
    }

    pub fn checkin(&self) -> CheckinModule {
        CheckinModule { client: self.clone() }
    }

    pub fn persons(&self) -> PersonsModule {
        PersonsModule { client: self.clone() }
    }

    pub fn events(&self) -> EventsModule {
        EventsModule { client: self.clone() }
    }

    pub fn tenants(&self) -> TenantsModule {
        TenantsModule { client: self.clone() }
    }

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.config.api_url.trim_end_matches('/'), path)
    }

    pub(crate) async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T, VelixError> {
        let status = resp.status().as_u16();
        match status {
            200..=299 => {
                let body: serde_json::Value = resp.json().await?;
                // identity-core wraps in { data: T }
                let payload = body.get("data").cloned().unwrap_or(body);
                Ok(serde_json::from_value(payload)?)
            }
            401 | 403 => Err(VelixError::Auth(resp.text().await.unwrap_or_default())),
            404 => Err(VelixError::NotFound(resp.text().await.unwrap_or_default())),
            429 => Err(VelixError::RateLimit { retry_after_secs: 60 }),
            _ => Err(VelixError::Http {
                status,
                message: resp.text().await.unwrap_or_default(),
            }),
        }
    }
}

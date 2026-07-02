use reqwest::{header, Client};
use std::sync::Arc;

use crate::{
    config::VelixConfig,
    error::VelixError,
    modules::{
        checkin::CheckinModule, events::EventsModule, persons::PersonsModule,
        tenants::TenantsModule,
    },
    retry::with_retry,
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
            .user_agent(concat!("velix-rust-sdk/", env!("CARGO_PKG_VERSION")))
            .build()?;

        Ok(Self {
            http,
            config: Arc::new(config),
        })
    }

    pub fn checkin(&self) -> CheckinModule {
        CheckinModule {
            client: self.clone(),
        }
    }

    pub fn persons(&self) -> PersonsModule {
        PersonsModule {
            client: self.clone(),
        }
    }

    pub fn events(&self) -> EventsModule {
        EventsModule {
            client: self.clone(),
        }
    }

    pub fn tenants(&self) -> TenantsModule {
        TenantsModule {
            client: self.clone(),
        }
    }

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.config.api_url.trim_end_matches('/'), path)
    }

    /// Executes an HTTP request built by `build`, applying the configured
    /// `max_retries` policy and centralizing status handling via
    /// `handle_response`. All modules should go through this instead of
    /// calling `send()` + `handle_response()` directly, so retry behaviour
    /// stays consistent across the SDK.
    pub(crate) async fn execute<T, B>(&self, mut build: B) -> Result<T, VelixError>
    where
        T: serde::de::DeserializeOwned,
        B: FnMut() -> reqwest::RequestBuilder,
    {
        let max_retries = self.config.max_retries;
        with_retry(max_retries, || {
            let req = build();
            async move {
                let resp = req.send().await?;
                self.handle_response(resp).await
            }
        })
        .await
    }

    pub(crate) async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T, VelixError> {
        let status = resp.status().as_u16();
        match status {
            200..=299 => {
                let bytes = resp.bytes().await?;
                if bytes.is_empty() {
                    // e.g. 204 No Content / empty 200 body — only valid for T = ()
                    return Ok(serde_json::from_value(serde_json::Value::Null)?);
                }
                let body: serde_json::Value = serde_json::from_slice(&bytes)?;
                // identity-core wraps in { data: T }
                let payload = body.get("data").cloned().unwrap_or(body);
                Ok(serde_json::from_value(payload)?)
            }
            401 | 403 => Err(VelixError::Auth(resp.text().await.unwrap_or_default())),
            404 => Err(VelixError::NotFound(resp.text().await.unwrap_or_default())),
            429 => {
                let retry_after_secs = parse_retry_after(resp.headers()).unwrap_or(60);
                Err(VelixError::RateLimit { retry_after_secs })
            }
            _ => Err(VelixError::Http {
                status,
                message: resp.text().await.unwrap_or_default(),
            }),
        }
    }
}

/// Parses the `Retry-After` header, which per RFC 9110 may be either an
/// integer number of seconds or an HTTP-date. Only the numeric-seconds form
/// is supported; HTTP-date values fall back to `None` (caller applies the
/// hardcoded default).
fn parse_retry_after(headers: &header::HeaderMap) -> Option<u64> {
    headers
        .get(header::RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.trim().parse::<u64>().ok())
}

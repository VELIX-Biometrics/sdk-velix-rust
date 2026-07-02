use crate::{client::VelixClient, error::VelixError};
use serde::Deserialize;

/// `MeResponse` — content of `Envelope.data` for `GET /v1/api/me/{personId}`.
#[derive(Debug, Deserialize)]
pub struct MeResponse {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: String,
}

pub struct MeModule {
    pub(crate) client: VelixClient,
}

impl MeModule {
    /// Wire contract: `GET /v1/api/me/{personId}` (scope `me:read`).
    pub async fn get(&self, person_id: &str) -> Result<MeResponse, VelixError> {
        let url = self.client.url(&format!("/v1/api/me/{person_id}"));
        self.client.execute(|| self.client.http.get(&url)).await
    }
}

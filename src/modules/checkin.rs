use crate::{client::VelixClient, error::VelixError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CheckinResult {
    pub passed: bool,
    pub person_id: Option<String>,
    pub person_name: Option<String>,
    pub age_policy_action: Option<String>,
}

#[derive(Serialize)]
struct FacialPayload<'a> {
    frame: &'a str,
}

pub struct CheckinModule {
    pub(crate) client: VelixClient,
}

impl CheckinModule {
    pub async fn facial(
        &self,
        tenant_slug: &str,
        frame_base64: &str,
    ) -> Result<CheckinResult, VelixError> {
        let url = self
            .client
            .url(&format!("/v1/checkin/{tenant_slug}/identify"));
        let payload = FacialPayload {
            frame: frame_base64,
        };
        let body = serde_json::to_string(&payload)?;

        self.client
            .execute(|| {
                self.client
                    .http
                    .post(&url)
                    .header("content-type", "application/json")
                    .body(body.clone())
            })
            .await
    }
}

use crate::{client::VelixClient, error::VelixError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Wire contract: `POST /v1/api/onboarding` (scope `onboarding:write`).
/// Mirrors `OnboardingRequest` in `lib-velix-contracts/openapi/public-api.yaml`.
#[derive(Debug, Default, Serialize)]
pub struct OnboardingRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub document: String,
    pub document_type: String,
    /// ISO 8601 (e.g. "1990-05-20"). Optional, but required to compute
    /// age/is_minor in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// JPEG frames, base64-encoded, without the `data:` URI prefix.
    pub frames: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_groups: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct FrameResult {
    pub frame_index: i32,
    pub quality_passed: bool,
    pub quality_score: f64,
    pub liveness_passed: bool,
}

/// `OnboardingResponse` — content of `Envelope.data` for `POST /v1/api/onboarding`.
#[derive(Debug, Deserialize)]
pub struct OnboardingResponse {
    pub person_id: String,
    pub identity_id: String,
    pub enrolled: bool,
    pub frames_processed: i32,
    pub frames_results: Vec<FrameResult>,
    pub embedding_id: Option<String>,
    pub message: String,
}

pub struct OnboardingModule {
    pub(crate) client: VelixClient,
}

impl OnboardingModule {
    pub async fn enroll(
        &self,
        request: OnboardingRequest,
    ) -> Result<OnboardingResponse, VelixError> {
        let url = self.client.url("/v1/api/onboarding");
        self.client
            .execute(|| self.client.http.post(&url).json(&request))
            .await
    }
}

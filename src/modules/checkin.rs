use crate::{client::VelixClient, error::VelixError};
use serde::{Deserialize, Serialize};

/// `LivenessSample` — one frame captured during a liveness challenge step.
#[derive(Debug, Serialize)]
pub struct LivenessSample {
    /// One of: `center`, `move_closer`, `move_away`.
    pub action: String,
    #[serde(rename = "imageBase64")]
    pub image_base64: String,
}

/// `LivenessBlock` — `token` is the nonce obtained from
/// `GET /v1/public/checkin/{tenantSlug}/liveness/challenge` (public endpoint,
/// no API key, shared between the public HMAC flow and this API-key flow).
#[derive(Debug, Serialize)]
pub struct LivenessBlock {
    pub token: String,
    pub samples: Vec<LivenessSample>,
}

#[derive(Debug, Default, Serialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
}

/// Wire contract: `POST /v1/api/checkin/identify` (scope `checkin:write`).
/// Mirrors `CheckinIdentifyRequest` in the public API spec exactly, including
/// camelCase fields (`imageBase64`, `topK`) that diverge from the mostly
/// snake_case wire casing used elsewhere in `/v1/api/*`.
#[derive(Debug, Default, Serialize)]
pub struct CheckinIdentifyRequest {
    #[serde(rename = "imageBase64")]
    pub image_base64: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    #[serde(rename = "topK", skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness: Option<LivenessBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

/// `LivenessResult` — the only liveness indicator exposed publicly is `ok`.
/// Liveness score is never returned by the API (VELIX security rule —
/// prevents binary-search attacks on the model).
#[derive(Debug, Deserialize)]
pub struct LivenessResult {
    pub ok: bool,
}

/// `CheckinIdentifyResponse` — real wire contract of
/// `CheckinService.identifyFace` (checkin.service.ts). Similarity score is
/// never exposed either — only the `match` boolean and subject identity.
#[derive(Debug, Deserialize)]
pub struct CheckinIdentifyResponse {
    #[serde(rename = "match")]
    pub matched: bool,
    #[serde(rename = "subjectId")]
    pub subject_id: Option<String>,
    #[serde(rename = "subjectName")]
    pub subject_name: Option<String>,
    pub liveness: LivenessResult,
    pub model: String,
}

pub struct CheckinModule {
    pub(crate) client: VelixClient,
}

impl CheckinModule {
    pub async fn identify(
        &self,
        request: CheckinIdentifyRequest,
    ) -> Result<CheckinIdentifyResponse, VelixError> {
        let url = self.client.url("/v1/api/checkin/identify");
        self.client
            .execute(|| self.client.http.post(&url).json(&request))
            .await
    }
}

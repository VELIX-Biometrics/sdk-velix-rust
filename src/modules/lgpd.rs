use crate::{client::VelixClient, error::VelixError};
use serde::{Deserialize, Serialize};

/// Wire contract: `POST /v1/api/deletion-request` (scope `lgpd:write`).
#[derive(Debug, Serialize)]
pub struct DeletionRequestBody {
    pub person_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DeletionRequestResponse {
    pub protocol_number: String,
    pub message: String,
}

pub struct LgpdModule {
    pub(crate) client: VelixClient,
}

impl LgpdModule {
    pub async fn request_deletion(
        &self,
        person_id: &str,
    ) -> Result<DeletionRequestResponse, VelixError> {
        let url = self.client.url("/v1/api/deletion-request");
        let body = DeletionRequestBody {
            person_id: person_id.to_string(),
        };
        self.client
            .execute(|| self.client.http.post(&url).json(&body))
            .await
    }
}

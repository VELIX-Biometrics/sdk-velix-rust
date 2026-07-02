use crate::{client::VelixClient, error::VelixError};
use serde::{Deserialize, Serialize};

/// Wire contract: `POST /v1/api/events/{id}/guests` (scope `events:write`).
/// Mirrors `CreateGuestRequest` exactly — note camelCase fields
/// (`birthDate`, `categoryId`, `companionOf`) alongside snake_case-free `cpf`/`phone`.
#[derive(Debug, Default, Serialize)]
pub struct CreateGuestRequest {
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpf: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "birthDate", skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    #[serde(rename = "categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(rename = "companionOf", skip_serializing_if = "Option::is_none")]
    pub companion_of: Option<String>,
}

/// `GuestResponse` — returned by both create and get-guest endpoints.
#[derive(Debug, Deserialize)]
pub struct GuestResponse {
    pub id: String,
    #[serde(rename = "eventId")]
    pub event_id: String,
    pub name: String,
    pub email: String,
    pub status: String,
    #[serde(rename = "categoryId")]
    pub category_id: Option<String>,
}

pub struct EventsModule {
    pub(crate) client: VelixClient,
}

impl EventsModule {
    /// `POST /v1/api/events/{id}/guests` (scope `events:write`).
    pub async fn create_guest(
        &self,
        event_id: &str,
        request: CreateGuestRequest,
    ) -> Result<GuestResponse, VelixError> {
        let url = self
            .client
            .url(&format!("/v1/api/events/{event_id}/guests"));
        self.client
            .execute(|| self.client.http.post(&url).json(&request))
            .await
    }

    /// `GET /v1/api/events/{id}/guests/{guestId}` (scope `events:read`).
    pub async fn get_guest(
        &self,
        event_id: &str,
        guest_id: &str,
    ) -> Result<GuestResponse, VelixError> {
        let url = self
            .client
            .url(&format!("/v1/api/events/{event_id}/guests/{guest_id}"));
        self.client.execute(|| self.client.http.get(&url)).await
    }
}

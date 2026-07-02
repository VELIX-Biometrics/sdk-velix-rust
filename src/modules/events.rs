use crate::{client::VelixClient, error::VelixError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct VelixEvent {
    pub id: String,
    pub name: String,
    pub status: String,
    pub tenant_id: String,
}

#[derive(Debug, Deserialize)]
pub struct EventsList {
    pub items: Vec<VelixEvent>,
    pub total: u64,
}

#[derive(Serialize)]
pub struct CreateEventDto {
    pub name: String,
    pub date: String,
    pub location: Option<String>,
}

#[derive(Serialize)]
pub struct EventConfigDto {
    pub color_primary: Option<String>,
    pub anti_passback: Option<bool>,
}

pub struct EventsModule {
    pub(crate) client: VelixClient,
}

impl EventsModule {
    pub async fn list(&self, page: u32, limit: u32) -> Result<EventsList, VelixError> {
        let url = self
            .client
            .url(&format!("/v1/events?page={page}&limit={limit}"));
        self.client.execute(|| self.client.http.get(&url)).await
    }

    pub async fn get(&self, id: &str) -> Result<VelixEvent, VelixError> {
        let url = self.client.url(&format!("/v1/events/{id}"));
        self.client.execute(|| self.client.http.get(&url)).await
    }

    pub async fn create(&self, dto: CreateEventDto) -> Result<VelixEvent, VelixError> {
        let url = self.client.url("/v1/events");
        self.client
            .execute(|| self.client.http.post(&url).json(&dto))
            .await
    }

    pub async fn configure(
        &self,
        id: &str,
        config: EventConfigDto,
    ) -> Result<VelixEvent, VelixError> {
        let url = self.client.url(&format!("/v1/events/{id}/config"));
        self.client
            .execute(|| self.client.http.patch(&url).json(&config))
            .await
    }
}

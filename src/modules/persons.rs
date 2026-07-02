use serde::{Deserialize, Serialize};
use crate::{client::VelixClient, error::VelixError};

#[derive(Debug, Deserialize, Clone)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub cpf: Option<String>,
    pub active: bool,
}

#[derive(Debug, Deserialize)]
pub struct PersonsList {
    pub items: Vec<Person>,
    pub total: u64,
}

#[derive(Serialize)]
pub struct CreatePersonDto {
    pub name: String,
    pub email: Option<String>,
    pub cpf: Option<String>,
}

#[derive(Serialize)]
pub struct EnrollDto {
    pub frames: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EnrollResult {
    pub enrolled: bool,
    pub quality_score: Option<f32>,
}

pub struct PersonsModule {
    pub(crate) client: VelixClient,
}

impl PersonsModule {
    pub async fn list(&self, page: u32, limit: u32) -> Result<PersonsList, VelixError> {
        let url = self.client.url(&format!("/v1/persons?page={page}&limit={limit}"));
        let resp = self.client.http.get(&url).send().await?;
        self.client.handle_response(resp).await
    }

    pub async fn get(&self, id: &str) -> Result<Person, VelixError> {
        let url = self.client.url(&format!("/v1/persons/{id}"));
        let resp = self.client.http.get(&url).send().await?;
        self.client.handle_response(resp).await
    }

    pub async fn create(&self, dto: CreatePersonDto) -> Result<Person, VelixError> {
        let url = self.client.url("/v1/persons");
        let resp = self.client.http.post(&url).json(&dto).send().await?;
        self.client.handle_response(resp).await
    }

    pub async fn update(&self, id: &str, dto: CreatePersonDto) -> Result<Person, VelixError> {
        let url = self.client.url(&format!("/v1/persons/{id}"));
        let resp = self.client.http.put(&url).json(&dto).send().await?;
        self.client.handle_response(resp).await
    }

    pub async fn delete(&self, id: &str) -> Result<(), VelixError> {
        let url = self.client.url(&format!("/v1/persons/{id}"));
        let resp = self.client.http.delete(&url).send().await?;
        let status = resp.status().as_u16();
        if (200..=299).contains(&status) {
            Ok(())
        } else {
            Err(VelixError::Http { status, message: resp.text().await.unwrap_or_default() })
        }
    }

    pub async fn enroll(&self, id: &str, frames: Vec<String>) -> Result<EnrollResult, VelixError> {
        let url = self.client.url(&format!("/v1/persons/{id}/enroll"));
        let resp = self.client.http.post(&url).json(&EnrollDto { frames }).send().await?;
        self.client.handle_response(resp).await
    }
}

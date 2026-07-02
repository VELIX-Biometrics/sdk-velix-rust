use crate::{client::VelixClient, error::VelixError};
use serde::{Deserialize, Serialize};

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
        let url = self
            .client
            .url(&format!("/v1/persons?page={page}&limit={limit}"));
        self.client.execute(|| self.client.http.get(&url)).await
    }

    pub async fn get(&self, id: &str) -> Result<Person, VelixError> {
        let url = self.client.url(&format!("/v1/persons/{id}"));
        self.client.execute(|| self.client.http.get(&url)).await
    }

    pub async fn create(&self, dto: CreatePersonDto) -> Result<Person, VelixError> {
        let url = self.client.url("/v1/persons");
        self.client
            .execute(|| self.client.http.post(&url).json(&dto))
            .await
    }

    pub async fn update(&self, id: &str, dto: CreatePersonDto) -> Result<Person, VelixError> {
        let url = self.client.url(&format!("/v1/persons/{id}"));
        self.client
            .execute(|| self.client.http.put(&url).json(&dto))
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<(), VelixError> {
        let url = self.client.url(&format!("/v1/persons/{id}"));
        self.client.execute(|| self.client.http.delete(&url)).await
    }

    pub async fn enroll(&self, id: &str, frames: Vec<String>) -> Result<EnrollResult, VelixError> {
        let url = self.client.url(&format!("/v1/persons/{id}/enroll"));
        let dto = EnrollDto { frames };
        self.client
            .execute(|| self.client.http.post(&url).json(&dto))
            .await
    }
}

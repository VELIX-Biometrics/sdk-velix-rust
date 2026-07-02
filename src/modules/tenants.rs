use serde::{Deserialize, Serialize};
use crate::{client::VelixClient, error::VelixError};

#[derive(Debug, Deserialize)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub plan: String,
    pub max_persons: Option<u32>,
}

#[derive(Serialize)]
pub struct TenantSettingsDto {
    pub require_liveness: Option<bool>,
    pub biometric_quality_level: Option<String>,
    pub geofence_radius_metros: Option<u32>,
    pub allow_offline_punch: Option<bool>,
    pub timezone: Option<String>,
    pub webhook_url: Option<String>,
}

pub struct TenantsModule {
    pub(crate) client: VelixClient,
}

impl TenantsModule {
    pub async fn me(&self) -> Result<Tenant, VelixError> {
        let url = self.client.url("/v1/tenants/me");
        let resp = self.client.http.get(&url).send().await?;
        self.client.handle_response(resp).await
    }

    pub async fn update_settings(&self, settings: TenantSettingsDto) -> Result<Tenant, VelixError> {
        let url = self.client.url("/v1/tenants/me/settings");
        let resp = self.client.http.put(&url).json(&settings).send().await?;
        self.client.handle_response(resp).await
    }
}

use crate::{client::VelixClient, error::VelixError};
use serde::{Deserialize, Serialize};

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
        self.client.execute(|| self.client.http.get(&url)).await
    }

    pub async fn update_settings(&self, settings: TenantSettingsDto) -> Result<Tenant, VelixError> {
        let url = self.client.url("/v1/tenants/me/settings");
        self.client
            .execute(|| self.client.http.put(&url).json(&settings))
            .await
    }
}

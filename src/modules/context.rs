use crate::{client::VelixClient, error::VelixError};
use serde_json::{json, Value};

/// `/v1/contexts/*` — Identity Context (Velix.ID). BearerAuth (JWT de sessão).
pub struct ContextModule {
    pub(crate) client: VelixClient,
}

impl ContextModule {
    /// `POST /v1/contexts`.
    pub async fn create(&self, payload: Value) -> Result<Value, VelixError> {
        let url = self.client.url("/v1/contexts");
        self.client.execute(|| self.client.http.post(&url).json(&payload)).await
    }

    /// `GET /v1/contexts/{id}`.
    pub async fn get(&self, id: &str) -> Result<Value, VelixError> {
        let url = self.client.url(&format!("/v1/contexts/{id}"));
        self.client.execute(|| self.client.http.get(&url)).await
    }

    /// `GET /v1/contexts`.
    pub async fn list(&self) -> Result<Value, VelixError> {
        let url = self.client.url("/v1/contexts");
        self.client.execute(|| self.client.http.get(&url)).await
    }

    /// `PATCH /v1/contexts/{id}`.
    pub async fn update(&self, id: &str, payload: Value) -> Result<Value, VelixError> {
        let url = self.client.url(&format!("/v1/contexts/{id}"));
        self.client.execute(|| self.client.http.patch(&url).json(&payload)).await
    }

    /// `DELETE /v1/contexts/{id}` — soft delete.
    pub async fn remove(&self, id: &str) -> Result<(), VelixError> {
        let url = self.client.url(&format!("/v1/contexts/{id}"));
        self.client.execute(|| self.client.http.delete(&url)).await
    }

    /// `POST /v1/contexts/{contextId}/authorize` — Authorization Engine.
    pub async fn authorize(&self, context_id: &str, payload: Value) -> Result<Value, VelixError> {
        let url = self.client.url(&format!("/v1/contexts/{context_id}/authorize"));
        self.client.execute(|| self.client.http.post(&url).json(&payload)).await
    }

    /// `GET /v1/contexts/{contextId}/authorization-decisions`.
    pub async fn list_authorization_decisions(&self, context_id: &str) -> Result<Value, VelixError> {
        let url = self
            .client
            .url(&format!("/v1/contexts/{context_id}/authorization-decisions"));
        self.client.execute(|| self.client.http.get(&url)).await
    }

    /// `POST /v1/contexts/{contextId}/link-requests` — solicita vínculo cross-tenant
    /// (consentimento pendente da pessoa). Nunca cria membership diretamente: retorna
    /// 202 (PENDING). A API pública não expõe approve/reject — o consentimento
    /// acontece fora do SDK (magic link/notificação).
    pub async fn create_link_request(&self, context_id: &str, payload: Value) -> Result<Value, VelixError> {
        let url = self.client.url(&format!("/v1/contexts/{context_id}/link-requests"));
        self.client.execute(|| self.client.http.post(&url).json(&payload)).await
    }
}

pub struct ContextMembershipModule {
    pub(crate) client: VelixClient,
}

impl ContextMembershipModule {
    /// `POST /v1/contexts/{contextId}/memberships`.
    pub async fn create(&self, context_id: &str, payload: Value) -> Result<Value, VelixError> {
        let url = self.client.url(&format!("/v1/contexts/{context_id}/memberships"));
        self.client.execute(|| self.client.http.post(&url).json(&payload)).await
    }

    /// `GET /v1/contexts/{contextId}/memberships`.
    pub async fn list_by_context(&self, context_id: &str) -> Result<Value, VelixError> {
        let url = self.client.url(&format!("/v1/contexts/{context_id}/memberships"));
        self.client.execute(|| self.client.http.get(&url)).await
    }

    /// `GET /v1/identities/{identityId}/memberships`.
    pub async fn list_by_identity(&self, identity_id: &str) -> Result<Value, VelixError> {
        let url = self.client.url(&format!("/v1/identities/{identity_id}/memberships"));
        self.client.execute(|| self.client.http.get(&url)).await
    }

    /// `PATCH /v1/memberships/{membershipId}/status`. `status = "revoked"` é a saída
    /// de contexto (definitiva, sem carência, task #834).
    pub async fn update_status(&self, membership_id: &str, status: &str) -> Result<Value, VelixError> {
        let url = self.client.url(&format!("/v1/memberships/{membership_id}/status"));
        let body = json!({ "status": status });
        self.client.execute(|| self.client.http.patch(&url).json(&body)).await
    }

    /// `POST /v1/memberships/{membershipId}/roles`.
    pub async fn add_roles(&self, membership_id: &str, role_ids: &[String]) -> Result<Value, VelixError> {
        let url = self.client.url(&format!("/v1/memberships/{membership_id}/roles"));
        let body = json!({ "roleIds": role_ids });
        self.client.execute(|| self.client.http.post(&url).json(&body)).await
    }

    /// `POST /v1/memberships/{membershipId}/roles/remove`.
    pub async fn remove_roles(&self, membership_id: &str, role_ids: &[String]) -> Result<Value, VelixError> {
        let url = self.client.url(&format!("/v1/memberships/{membership_id}/roles/remove"));
        let body = json!({ "roleIds": role_ids });
        self.client.execute(|| self.client.http.post(&url).json(&body)).await
    }
}

pub struct ContextRoleModule {
    pub(crate) client: VelixClient,
}

impl ContextRoleModule {
    /// `POST /v1/context-roles`.
    pub async fn create(&self, payload: Value) -> Result<Value, VelixError> {
        let url = self.client.url("/v1/context-roles");
        self.client.execute(|| self.client.http.post(&url).json(&payload)).await
    }

    /// `GET /v1/context-roles?contextType=...`.
    pub async fn list(&self, context_type: &str) -> Result<Value, VelixError> {
        let url = self.client.url("/v1/context-roles");
        self.client
            .execute(|| self.client.http.get(&url).query(&[("contextType", context_type)]))
            .await
    }

    /// `POST /v1/context-roles/{roleId}/permissions`.
    pub async fn link_permissions(&self, role_id: &str, permission_ids: &[String]) -> Result<Value, VelixError> {
        let url = self.client.url(&format!("/v1/context-roles/{role_id}/permissions"));
        let body = json!({ "permissionIds": permission_ids });
        self.client.execute(|| self.client.http.post(&url).json(&body)).await
    }
}

pub struct ContextPermissionModule {
    pub(crate) client: VelixClient,
}

impl ContextPermissionModule {
    /// `POST /v1/context-permissions`.
    pub async fn create(&self, payload: Value) -> Result<Value, VelixError> {
        let url = self.client.url("/v1/context-permissions");
        self.client.execute(|| self.client.http.post(&url).json(&payload)).await
    }

    /// `GET /v1/context-permissions?category=...`.
    pub async fn list(&self, category: Option<&str>) -> Result<Value, VelixError> {
        let url = self.client.url("/v1/context-permissions");
        self.client
            .execute(|| {
                let req = self.client.http.get(&url);
                match category {
                    Some(c) => req.query(&[("category", c)]),
                    None => req,
                }
            })
            .await
    }
}

pub struct AuthorizationTokenModule {
    pub(crate) client: VelixClient,
}

impl AuthorizationTokenModule {
    /// `POST /v1/authorization-tokens/validate` — valida (e opcionalmente consome)
    /// um token vat_*.
    pub async fn validate(&self, token: &str, consume: bool) -> Result<Value, VelixError> {
        let url = self.client.url("/v1/authorization-tokens/validate");
        let body = json!({ "token": token, "consume": consume });
        self.client.execute(|| self.client.http.post(&url).json(&body)).await
    }
}

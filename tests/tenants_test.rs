use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use velix_sdk::{VelixClient, VelixConfig};

#[tokio::test]
async fn test_tenants_me() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/tenants/me"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {
                "id": "tenant-uuid", "name": "Acme Corp", "slug": "acme",
                "plan": "enterprise", "max_persons": 1000
            }
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vx_test_key")).unwrap();
    let tenant = client.tenants().me().await.unwrap();

    assert_eq!(tenant.id, "tenant-uuid");
    assert_eq!(tenant.slug, "acme");
    assert_eq!(tenant.plan, "enterprise");
}

#[tokio::test]
async fn test_tenants_update_settings() {
    let server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/v1/tenants/me/settings"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {
                "id": "tenant-uuid", "require_liveness": true, "timezone": "America/Sao_Paulo"
            }
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vx_test_key")).unwrap();
    let tenant = client.tenants().update_settings(velix_sdk::UpdateTenantSettingsInput {
        require_liveness: Some(true),
        timezone: Some("America/Sao_Paulo".to_string()),
        ..Default::default()
    }).await.unwrap();

    assert!(tenant.require_liveness);
    assert_eq!(tenant.timezone.as_deref(), Some("America/Sao_Paulo"));
}

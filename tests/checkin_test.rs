use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use velix_sdk::{VelixClient, VelixConfig};

#[tokio::test]
async fn test_facial_checkin_passed() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/checkin/acme/identify"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {
                "passed": true,
                "person_id": "uuid-123",
                "person_name": "João Silva"
            }
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vx_test_key")).unwrap();
    let result = client.checkin().facial("acme", "base64frame==").await.unwrap();

    assert!(result.passed);
    assert_eq!(result.person_id.as_deref(), Some("uuid-123"));
}

#[tokio::test]
async fn test_facial_checkin_denied() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/checkin/acme/identify"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": { "passed": false }
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vx_test_key")).unwrap();
    let result = client.checkin().facial("acme", "base64frame==").await.unwrap();

    assert!(!result.passed);
}

use velix_sdk::{VelixClient, VelixConfig};
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_deletion_request() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/api/deletion-request"))
        .and(body_json(serde_json::json!({ "person_id": "p-1" })))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "data": { "protocol_number": "PR-001", "message": "requested" }
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vlx_test_key")).unwrap();
    let result = client.lgpd().request_deletion("p-1").await.unwrap();

    assert_eq!(result.protocol_number, "PR-001");
}

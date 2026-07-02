use velix_sdk::{VelixClient, VelixConfig};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_me() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/api/me/p-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {
                "id": "p-1",
                "name": "Maria",
                "email": "maria@acme.com",
                "phone": null,
                "photo_url": null,
                "created_at": "2026-01-01T00:00:00.000Z"
            }
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vlx_test_key")).unwrap();
    let me = client.me().get("p-1").await.unwrap();

    assert_eq!(me.id, "p-1");
    assert_eq!(me.name, "Maria");
}

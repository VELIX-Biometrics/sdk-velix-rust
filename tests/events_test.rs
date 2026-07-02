use velix_sdk::{CreateGuestRequest, VelixClient, VelixConfig};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_create_guest() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/api/events/evt-1/guests"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "data": {
                "id": "g-1",
                "eventId": "evt-1",
                "name": "Carlos",
                "email": "carlos@acme.com",
                "status": "invited",
                "categoryId": null
            }
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vlx_test_key")).unwrap();
    let guest = client
        .events()
        .create_guest(
            "evt-1",
            CreateGuestRequest {
                name: "Carlos".to_string(),
                email: "carlos@acme.com".to_string(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    assert_eq!(guest.id, "g-1");
    assert_eq!(guest.event_id, "evt-1");
}

#[tokio::test]
async fn test_get_guest_not_found() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/api/events/evt-1/guests/nonexistent"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "error": { "message": "Guest not found", "code": "NOT_FOUND" }
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vlx_test_key")).unwrap();
    let err = client
        .events()
        .get_guest("evt-1", "nonexistent")
        .await
        .unwrap_err();

    assert!(matches!(err, velix_sdk::VelixError::NotFound(_)));
}

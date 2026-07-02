use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};
use velix_sdk::{VelixClient, VelixConfig};

#[tokio::test]
async fn test_events_list() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/events"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {
                "items": [{"id": "evt-1", "name": "Tech Summit", "status": "active"}],
                "total": 1, "page": 1, "limit": 20
            }
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vx_test_key")).unwrap();
    let result = client.events().list(1, 20).await.unwrap();

    assert_eq!(result.total, 1);
    assert_eq!(result.items.len(), 1);
    assert_eq!(result.items[0].id, "evt-1");
}

#[tokio::test]
async fn test_events_get() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/events/evt-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {"id": "evt-1", "name": "Tech Summit", "status": "active"}
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vx_test_key")).unwrap();
    let event = client.events().get("evt-1").await.unwrap();

    assert_eq!(event.id, "evt-1");
    assert_eq!(event.name, "Tech Summit");
}

#[tokio::test]
async fn test_events_get_not_found() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/events/nonexistent"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "Event not found", "code": "NOT_FOUND"
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vx_test_key")).unwrap();
    let err = client.events().get("nonexistent").await.unwrap_err();

    assert!(matches!(err, velix_sdk::VelixError::NotFound(_)));
}

#[tokio::test]
async fn test_events_create() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/events"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "data": {"id": "evt-new", "name": "New Event", "status": "draft"}
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vx_test_key")).unwrap();
    let event = client.events().create("New Event").await.unwrap();

    assert_eq!(event.id, "evt-new");
    assert_eq!(event.status, "draft");
}

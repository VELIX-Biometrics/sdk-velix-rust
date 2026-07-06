use velix_sdk::modules::checkin::CheckinIdentifyRequest;
use velix_sdk::{VelixClient, VelixConfig};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_checkin_identify_matched() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/api/checkin/identify"))
        .and(header("x-api-key", "vlx_test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {
                "match": true,
                "subjectId": "uuid-123",
                "subjectName": "Ana Silva",
                "liveness": { "ok": true },
                "model": "adaface"
            }
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vlx_test_key")).unwrap();
    let result = client
        .checkin()
        .identify(CheckinIdentifyRequest {
            image_base64: "base64frame==".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

    assert!(result.matched);
    assert_eq!(result.subject_id.as_deref(), Some("uuid-123"));
    assert!(result.liveness.ok);
}

#[tokio::test]
async fn test_checkin_identify_not_matched() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/api/checkin/identify"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {
                "match": false,
                "subjectId": null,
                "subjectName": null,
                "liveness": { "ok": true },
                "model": "adaface"
            }
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vlx_test_key")).unwrap();
    let result = client
        .checkin()
        .identify(CheckinIdentifyRequest {
            image_base64: "base64frame==".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

    assert!(!result.matched);
    // Similarity score is never present in the response — only booleans are
    // exposed. Serde would simply ignore an extra field if the backend sent
    // one, but the response type itself has no field to deserialize a score
    // into, which is the real guarantee we want to assert here.
    assert!(result.subject_id.is_none());
}

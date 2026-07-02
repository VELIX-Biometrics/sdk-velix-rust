use velix_sdk::{OnboardingRequest, VelixClient, VelixConfig};
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_onboarding_enroll() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/api/onboarding"))
        .and(body_json(serde_json::json!({
            "name": "Maria",
            "frames": ["frame1", "frame2", "frame3"]
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "data": {
                "person_id": "p-1",
                "identity_id": "i-1",
                "enrolled": true,
                "frames_processed": 3,
                "frames_results": [],
                "embedding_id": "e-1",
                "message": "ok"
            }
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vlx_test_key")).unwrap();
    let result = client
        .onboarding()
        .enroll(OnboardingRequest {
            name: "Maria".to_string(),
            frames: vec!["frame1".into(), "frame2".into(), "frame3".into()],
            ..Default::default()
        })
        .await
        .unwrap();

    assert!(result.enrolled);
    assert_eq!(result.person_id, "p-1");
}

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use velix_sdk::{VelixClient, VelixConfig};
use velix_sdk::modules::persons::CreatePersonDto;

#[tokio::test]
async fn test_list_persons() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/persons"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {
                "items": [
                    { "id": "uuid-1", "name": "Maria", "active": true }
                ],
                "total": 1
            }
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vx_test_key")).unwrap();
    let list = client.persons().list(1, 20).await.unwrap();

    assert_eq!(list.total, 1);
    assert_eq!(list.items[0].name, "Maria");
}

#[tokio::test]
async fn test_create_person() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/persons"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "data": { "id": "uuid-new", "name": "Carlos", "active": true }
        })))
        .mount(&server)
        .await;

    let client = VelixClient::new(VelixConfig::new(server.uri(), "vx_test_key")).unwrap();
    let person = client.persons().create(CreatePersonDto {
        name: "Carlos".to_string(),
        email: None,
        cpf: None,
    }).await.unwrap();

    assert_eq!(person.name, "Carlos");
}

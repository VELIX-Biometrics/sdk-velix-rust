// Smoke test de contrato — `cargo run --example smoke_test`.
use velix_sdk::{modules::events::CreateGuestRequest, modules::onboarding::OnboardingRequest, VelixClient, VelixConfig};

const IMG: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=";

fn result(step: &str, ok: bool, detail: &str) {
    println!("RESULT:rust:{}:{}:{}", step, if ok { "PASS" } else { "FAIL" }, detail);
}

fn reachable(msg: &str) -> bool {
    let m = msg.to_lowercase();
    !["route not found", "no route", "401", "403"].iter().any(|s| m.contains(s))
}

#[tokio::main]
async fn main() {
    let api_url = std::env::var("API_BASE_URL").unwrap();
    let api_key = std::env::var("VELIX_API_KEY").unwrap();
    let config = VelixConfig::new(api_url, api_key);
    let client = VelixClient::new(config).expect("client init");

    let mut person_id: Option<String> = None;
    match client
        .onboarding()
        .enroll(OnboardingRequest {
            name: "Smoke Test Rust".to_string(),
            email: None,
            phone: None,
            document: None,
            document_type: None,
            external_id: None,
            metadata: None,
            frames: vec![IMG.to_string(), IMG.to_string(), IMG.to_string()],
            role: None,
            access_groups: None,
        })
        .await
    {
        Ok(r) => {
            person_id = Some(r.person_id.clone());
            result("onboarding", true, &format!("person_id={}", r.person_id));
        }
        Err(e) => result("onboarding", false, &e.to_string()),
    }

    match client
        .checkin()
        .identify(velix_sdk::modules::checkin::CheckinIdentifyRequest {
            image_base64: IMG.to_string(),
            images: None,
            top_k: None,
            liveness: None,
            location: None,
        })
        .await
    {
        Ok(r) => result("checkin", true, &format!("matched={}", r.matched)),
        Err(e) => result("checkin", false, &e.to_string()),
    }

    if let Some(pid) = &person_id {
        match client.lgpd().request_deletion(pid).await {
            Ok(_) => result("lgpd", true, "deletion-request ok"),
            Err(e) => result("lgpd", false, &e.to_string()),
        }
        match client.me().get(pid).await {
            Ok(_) => result("me", true, "got response"),
            Err(e) => result("me", false, &e.to_string()),
        }
    }

    let dummy = "00000000-0000-0000-0000-000000000000";
    match client
        .events()
        .create_guest(
            dummy,
            CreateGuestRequest {
                name: "Guest Smoke".to_string(),
                email: "guest@smoke.test".to_string(),
                cpf: None,
                phone: None,
                birth_date: None,
                category_id: None,
                companion_of: None,
            },
        )
        .await
    {
        Ok(_) => result("events_create", true, "endpoint reachable"),
        Err(e) => result("events_create", reachable(&e.to_string()), &e.to_string()),
    }

    match client.events().get_guest(dummy, dummy).await {
        Ok(_) => result("events_get", true, "endpoint reachable"),
        Err(e) => result("events_get", reachable(&e.to_string()), &e.to_string()),
    }
}

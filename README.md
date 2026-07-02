# velix-sdk — Rust SDK ![version](https://img.shields.io/badge/version-0.1.0--alpha.1-orange)

> ⚠️ **Alpha / pre-release.** This SDK targets a public API surface that does not yet fully exist on the VELIX backend (see internal task #593). Endpoints and auth may not work against production. Do not use in production integrations yet.

Official Rust SDK for the VELIX Biometrics platform — facial access control B2B SaaS.

## Requirements

- Rust 2021 edition (stable toolchain)
- tokio async runtime

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
velix-sdk = "0.1.0-alpha.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use velix_sdk::{VelixClient, VelixConfig};

#[tokio::main]
async fn main() -> Result<(), velix_sdk::VelixError> {
    let client = VelixClient::new(VelixConfig {
        api_url: std::env::var("VELIX_API_URL").unwrap(),
        api_key: std::env::var("VELIX_API_KEY").unwrap(),
        ..Default::default()
    })?;

    let result = client.checkin().facial("tenant-slug", &frame_base64).await?;
    println!("passed: {}", result.passed);
    Ok(())
}
```

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `VELIX_API_URL` | Yes | API base URL (`https://api.velixbiometrics.com`) |
| `VELIX_API_KEY` | Yes | Tenant API key (`vx_live_...` or `vx_sandbox_...`) |

## Modules

| Module | Methods |
|--------|---------|
| `client.checkin()` | `facial()`, `qr()`, `pin()`, `get_history()` |
| `client.persons()` | `list()`, `get()`, `create()`, `update()`, `delete()`, `enroll()` |
| `client.events()` | `list()`, `get()`, `create()`, `configure()` |
| `client.tenants()` | `me()`, `update_settings()` |

## Checkin Module

```rust
let checkin = client.checkin();

// Facial identification (base64 JPEG frame)
let result = checkin.facial("tenant-slug", &frame_base64).await?;
// result.passed == true
// result.person_id == Some("uuid".to_string())
// result.person_name == Some("João Silva".to_string())

// QR code checkin
let result = checkin.qr("tenant-slug", &qr_token).await?;

// PIN checkin
let result = checkin.pin("tenant-slug", &pin).await?;

// Paginated history
let history = checkin.get_history("tenant-slug", 1, 20).await?;
```

## Persons Module

```rust
let persons = client.persons();

// List
let list = persons.list(1, 20).await?;

// Get by ID
let person = persons.get("uuid").await?;

// Create
let created = persons.create(CreatePersonInput {
    name: "João Silva".into(),
    email: Some("joao@company.com".into()),
    external_id: Some("EMP-001".into()),
}).await?;

// Update
persons.update("uuid", UpdatePersonInput { name: Some("João B. Silva".into()), ..Default::default() }).await?;

// Enroll biometrics (minimum 3 base64 frames)
persons.enroll("uuid", vec![frame1, frame2, frame3]).await?;

// Delete
persons.delete("uuid").await?;
```

## Events Module

```rust
let events = client.events();

let list    = events.list(1, 20).await?;
let event   = events.get("uuid").await?;
let created = events.create(CreateEventInput { name: "Conference 2026".into() }).await?;
events.configure("uuid", EventConfig { check_in_open: Some(true), ..Default::default() }).await?;
```

## Tenants Module

```rust
let tenant = client.tenants().me().await?;
client.tenants().update_settings(TenantSettings { require_liveness: Some(true), ..Default::default() }).await?;
```

## Error Handling

```rust
use velix_sdk::{VelixError, AuthError, BiometricError, RateLimitError};

match client.checkin().facial("slug", &frame).await {
    Ok(result) => println!("passed: {}", result.passed),
    Err(VelixError::Auth(e))       => eprintln!("Invalid API key: {}", e),
    Err(VelixError::Biometric(e))  => eprintln!("Face not recognized: {}", e),
    Err(VelixError::RateLimit(e))  => eprintln!("Rate limit — retry after {:?}", e.retry_after),
    Err(e)                         => eprintln!("Unexpected error: {}", e),
}
```

## Running Tests

```bash
cargo test
cargo test -- --nocapture    # show println! output
cargo test checkin           # filter by name
```

## Local Development

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt
```

## Get an API Key

Access the dashboard at **velixbiometrics.com** → Settings → API Keys → New Key.

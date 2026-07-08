# velix-sdk — Rust SDK ![version](https://img.shields.io/badge/version-0.1.0--alpha.1-orange)

> ⚠️ **Alpha / pre-release**, mas já publicado e confirmado funcionando de ponta a ponta contra a API real de staging (onboarding, LGPD, me, events). **crates.io:** https://crates.io/crates/velix-sdk

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

    let result = client
        .checkin()
        .identify(velix_sdk::CheckinIdentifyRequest {
            image_base64: frame_base64,
            ..Default::default()
        })
        .await?;
    println!("matched: {}", result.matched);
    Ok(())
}
```

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `VELIX_API_URL` | Yes | API base URL (`https://api.velixbiometrics.com`) |
| `VELIX_API_KEY` | Yes | API key (`vlx_...`), sent as `x-api-key` (alt: `Authorization: Bearer vlx_...`) |

Default HTTP client timeout is 30000ms (30s), overridable via `VelixConfig.timeout`.

## Modules

All modules map 1:1 to the 6 real endpoints under `/v1/api/*` in
`api-velix-identity-core`. Every response is auto-unwrapped from the
`{ "data": T }` envelope — the SDK only ever exposes `T`.

| Module | Method | Endpoint | Scope |
|--------|--------|----------|-------|
| `client.onboarding()` | `enroll()` | `POST /v1/api/onboarding` | `onboarding:write` |
| `client.checkin()` | `identify()` | `POST /v1/api/checkin/identify` | `checkin:write` |
| `client.lgpd()` | `request_deletion()` | `POST /v1/api/deletion-request` | `lgpd:write` |
| `client.me()` | `get()` | `GET /v1/api/me/{personId}` | `me:read` |
| `client.events()` | `create_guest()` | `POST /v1/api/events/{id}/guests` | `events:write` |
| `client.events()` | `get_guest()` | `GET /v1/api/events/{id}/guests/{guestId}` | `events:read` |

**Velix Time** has no endpoints in the public API spec today (see
`x-velix-sdk-contract-notes` in `public-api.yaml` — scopes `time:read`/
`time:write` are reserved but unmounted). This SDK intentionally has no
`time()` module; do not add one that no-ops or fakes data.

Liveness score is **never** exposed by the API — only `passed`/`matched`
booleans. This is a deliberate security property (prevents binary-search
attacks against the biometric model), not an SDK limitation.

## Identity Context

Contexts, roles, permissions, memberships, link-requests (cross-tenant consent)
and the authorization engine. BearerAuth (session JWT), not `x-api-key`. See
`code/lib/lib-velix-contracts/openapi/public-api.yaml`, tag `Identity Context`.

| Module | Methods | Endpoint |
|--------|---------|----------|
| `client.contexts()` | `create/get/list/update/remove`, `authorize`, `list_authorization_decisions`, `create_link_request` | `/v1/contexts/*` |
| `client.memberships()` | `create`, `list_by_context`, `list_by_identity`, `update_status`, `add_roles`, `remove_roles` | `/v1/contexts/:id/memberships`, `/v1/identities/:id/memberships`, `/v1/memberships/*` |
| `client.context_roles()` | `create`, `list`, `link_permissions` | `/v1/context-roles*` |
| `client.context_permissions()` | `create`, `list` | `/v1/context-permissions` |
| `client.authorization_tokens()` | `validate` | `POST /v1/authorization-tokens/validate` |

```rust
let context = client.contexts().create(json!({"name": "Matriz SP", "contextType": "location"})).await?;
let decision = client.contexts().authorize(context_id, json!({"identityId": identity_id, "permission": "access:enter"})).await?;
let membership = client.memberships().create(context_id, json!({"identityId": identity_id, "roleIds": [role_id]})).await?;
// context exit (definitive, no grace period)
client.memberships().update_status(membership_id, "revoked").await?;
// cross-tenant link — stays PENDING until the person consents via magic link
client.contexts().create_link_request(context_id, json!({"identityId": identity_id})).await?;
client.authorization_tokens().validate("vat_...", false).await?;
```

## Onboarding Module

```rust
let result = client.onboarding().enroll(velix_sdk::OnboardingRequest {
    name: "João Silva".into(),
    email: Some("joao@company.com".into()),
    frames: vec![frame1, frame2, frame3], // base64 JPEG, no data: URI prefix
    ..Default::default()
}).await?;
// result.enrolled, result.person_id, result.identity_id
```

## Checkin Module

```rust
let result = client.checkin().identify(velix_sdk::CheckinIdentifyRequest {
    image_base64: frame_base64,
    top_k: Some(3),
    liveness: Some(velix_sdk::LivenessBlock {
        token: nonce_token, // from GET /v1/public/checkin/{tenantSlug}/liveness/challenge
        samples: vec![velix_sdk::LivenessSample {
            action: "center".into(),
            image_base64: sample_frame,
        }],
    }),
    ..Default::default()
}).await?;
// result.matched, result.subject_id, result.subject_name, result.liveness.ok
```

## LGPD Module

```rust
let result = client.lgpd().request_deletion("person-uuid").await?;
// result.protocol_number
```

## Me Module

```rust
let me = client.me().get("person-uuid").await?;
// me.id, me.name, me.email, me.photo_url
```

## Events Module

```rust
let guest = client.events().create_guest("event-uuid", velix_sdk::CreateGuestRequest {
    name: "Carlos".into(),
    email: "carlos@acme.com".into(),
    ..Default::default()
}).await?;

let guest = client.events().get_guest("event-uuid", "guest-uuid").await?;
// guest.status reflects checkin status
```

## Error Handling

```rust
use velix_sdk::VelixError;

match client.checkin().identify(request).await {
    Ok(result) => println!("matched: {}", result.matched),
    Err(VelixError::Auth(e)) => eprintln!("Invalid API key: {}", e),
    Err(VelixError::RateLimit { retry_after_secs }) => {
        eprintln!("Rate limit — retry after {}s", retry_after_secs)
    }
    Err(e) => eprintln!("Unexpected error: {}", e),
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

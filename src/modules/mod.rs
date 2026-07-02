pub mod checkin;
pub mod events;
pub mod lgpd;
pub mod me;
pub mod onboarding;

// Velix Time has no exposed API-key surface today (see public-api.yaml
// "COBERTURA PARCIAL" note — no `/v1/api/time/*` controller exists in
// api-velix-identity-core). Intentionally not implemented — do not add a
// `time` module that silently no-ops or returns fake data.

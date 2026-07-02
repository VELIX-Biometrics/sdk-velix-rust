pub mod client;
pub mod config;
pub mod error;
pub mod modules;
pub(crate) mod retry;

pub use client::VelixClient;
pub use config::VelixConfig;
pub use error::VelixError;
pub use modules::{
    checkin::{CheckinIdentifyRequest, CheckinIdentifyResponse, Location, LivenessBlock, LivenessSample},
    events::{CreateGuestRequest, GuestResponse},
    lgpd::DeletionRequestResponse,
    me::MeResponse,
    onboarding::{FrameResult, OnboardingRequest, OnboardingResponse},
};

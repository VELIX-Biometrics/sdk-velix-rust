pub mod client;
pub mod config;
pub mod error;
pub mod modules;
pub(crate) mod retry;

pub use client::VelixClient;
pub use config::VelixConfig;
pub use error::VelixError;
pub use modules::{
    checkin::CheckinResult,
    events::VelixEvent,
    persons::Person,
    tenants::Tenant,
};

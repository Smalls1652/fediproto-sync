/// Config utilities for Fediproto Sync.
pub mod config;
/// Crypto operations for Fediproto Sync.
pub mod crypto;
/// Error types for Fediproto Sync.
pub mod error;

/// Version information generated from git metadata.
pub const GIT_VERSION: &str = std::env!("GIT_VERSION");

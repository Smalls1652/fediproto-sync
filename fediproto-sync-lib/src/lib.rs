/// Config utilities for Fediproto Sync.
pub mod config;

/// Crypto operations for Fediproto Sync.
pub mod crypto;

/// Error types for Fediproto Sync.
pub mod error;

/// Generic utilities for FediProto Sync.
pub mod utils;

/// Version information generated from git metadata.
pub const GIT_VERSION: &str = std::env!("GIT_VERSION");

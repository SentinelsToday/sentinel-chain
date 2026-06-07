//! `sentinel-chain` — on-chain instruction + state encoding for the Sentinel
//! Labs attestation anchoring program.
//!
//! The crate is split so it compiles on stock host Rust by default (no
//! Solana SDK). Enable the `solana` feature (or run `cargo build-sbf`) to
//! pull in the on-chain entrypoint.

pub mod instruction;
pub mod state;

#[cfg(feature = "solana")]
pub mod program;

pub const PROGRAM_ID_STR: &str = "Sent1nelChain111111111111111111111111111111";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

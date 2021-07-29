#![deny(missing_docs)]

//! An Uniswap-like program for the Safecoin blockchain.

pub mod constraints;
pub mod curve;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("6RWe1TGwvojnbAynyWrHzm3GgHf7AmX7kLQTJG7vHCfb");

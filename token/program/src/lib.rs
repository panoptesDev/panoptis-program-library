#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! An ERC20-like Token program for the Solcoin blockchain

pub mod error;
pub mod instruction;
pub mod native_mint;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

/// Convert the UI representation of a token amount (using the decimals field defined in its mint)
/// to the raw amount
pub fn ui_amount_to_amount(ui_amount: f64, decimals: u8) -> u64 {
    (ui_amount * 10_usize.pow(decimals as u32) as f64) as u64
}

/// Convert a raw amount to its UI representation (using the decimals field defined in its mint)
pub fn amount_to_ui_amount(amount: u64, decimals: u8) -> f64 {
    amount as f64 / 10_usize.pow(decimals as u32) as f64
}

solana_program::declare_id!("HMGr16f8Ct1Zeb9TGPypt9rPgzCkmhCQB8Not8vwiPW1");

/// Checks that the supplied program ID is the correct one for SPL-token
pub fn check_program_account(spl_token_program_id: &Pubkey) -> ProgramResult {
    if spl_token_program_id != &id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

use anchor_lang::prelude::*;

#[error_code]
pub enum CnctdStudioError {
    #[msg("Invalid input provided.")]
    InvalidInput,

    #[msg("Unauthorized action.")]
    Unauthorized,

    #[msg("Resource not found.")]
    NotFound,

    #[msg("Operation failed due to an unknown error.")]
    OperationFailed,

    #[msg("Insufficient funds.")]
    InsufficientFunds,

    #[msg("Invalid token account owner")]
    InvalidTokenAccountOwner,

    #[msg("Treasury ATA does not match expected address")]
    InvalidTreasuryATA,

    #[msg("Treasury PDA does not match expected address")]
    InvalidTreasuryPDA,

    #[msg("Invalid Mint")]
    InvalidMint,

    #[msg("Invalid Mint Authority")]
    InvalidMintAuthority,
}

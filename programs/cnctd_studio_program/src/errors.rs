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
}

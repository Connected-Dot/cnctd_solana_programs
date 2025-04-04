use anchor_lang::prelude::*;

#[error_code]
pub enum CnctdStudioError {
    #[msg("Not implemented yet.")]
    NotImplemented,

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

    #[msg("Invalid artist PDA")]
    InvalidArtistPDA,

    #[msg("Invalid token account")]
    InvalidTokenAccount,

    #[msg("Invalid token mint")]
    InvalidTokenMint,
    
    #[msg("Splits must add up to 100%")]
    InvalidSplitTotal,

    #[msg("Cannot remove the last admin")]
    CannotRemoveLastAdmin,

    #[msg("Admin already exists")]
    AdminAlreadyExists,

    #[msg("Invalid user")]
    InvalidUser,

    #[msg("Admin not found")]
    AdminNotFound,

    #[msg("Not enough accounts provided")]
    NotEnoughAccounts,

    #[msg("Escrow already fulfilled")]
    EscrowAlreadyFulfilled,

    #[msg("Token account not empty")]
    TokenAccountNotEmpty,

    #[msg("Close account failed")]
    CloseAccountFailed,

    #[msg("Transfer failed")]
    TransferFailed,

    #[msg("Invalid payment recipient")]
    InvalidPaymentReceiver,

    #[msg("Invalid payment splits")]
    InvalidPaymentSplits,

    #[msg("Escrow not fulfilled")]
    EscrowNotFulfilled,

    #[msg("Payments not fulfilled")]
    PaymentsNotFulfilled,

    #[msg("NFT not minted")]
    NFTNotMinted,
}

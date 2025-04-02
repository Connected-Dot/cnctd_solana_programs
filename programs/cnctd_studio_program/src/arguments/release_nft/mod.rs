use anchor_lang::prelude::*;

mod open_escrow_args;
mod fulfill_release_args;
mod complete_release_args;
mod close_escrow_args;

pub use open_escrow_args::*;
pub use fulfill_release_args::*;
pub use complete_release_args::*;
pub use close_escrow_args::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PaymentSplit {
    pub recipient_usdc_ata: Pubkey,  // ATA to receive payment
    pub recipient_cnctd_ata: Pubkey,  // ATA to receive CNCTD reward (if applicable)
    pub amount: u64,        // Pre-calculated amount in USDC lamports
}
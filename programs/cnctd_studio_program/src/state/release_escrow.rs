use anchor_lang::prelude::*;

use crate::arguments::release::PaymentSplit;

#[account]
pub struct ReleaseEscrow {
    pub release_id: String,
    pub buyer_id: String,
    pub treasury_fee: u64,
    pub payment_splits: Vec<PaymentSplit>,
    pub total_amount: u64,
    pub payments_fulfilled: bool,  // Payments have been sent to treasury and artists
    pub nft_minted: bool,          // NFT has been minted to buyer
    pub rewards_paid: bool,        // Rewards have been issued
    pub fulfilled: bool,           // Overall completion flag (escrow can be closed)
    pub purchase_date: i64,
    pub bump: u8,
}

impl ReleaseEscrow {
    // Calculate space needed for the account
    pub fn space() -> usize {
        // Base size for fixed fields
        let size = 8 + // discriminator
                   32 + // release_id (max)
                   32 + // buyer_id (max)
                   8 + // treasury_fee
                   4 + // payment_splits vec length
                   (10 * (32 + 32 + 8)) + // Up to 10 payment splits
                   8 + // total_amount
                   1 + // payments_fulfilled
                   1 + // nft_minted
                   1 + // rewards_paid
                   1 + // fulfilled
                   8 + // purchase_date (i64 timestamp)
                   1; // bump
        size
    }
}
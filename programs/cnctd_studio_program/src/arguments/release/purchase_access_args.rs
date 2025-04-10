use anchor_lang::prelude::*;

use super::PaymentSplit;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PurchaseAccessArgs {
    pub buyer_id: String,
    pub release_id: String,
    pub treasury_fee: u64,
    pub payment_splits: Vec<PaymentSplit>,
    pub created_at: i64,
    pub expiration_date: Option<i64>,
    pub fee_compensation: Option<u64>,
}

use anchor_lang::prelude::*;

use crate::state::release_access::{AccessRights, AccessType};

use super::PaymentSplit;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct OpenEscrowArgs {
    pub buyer_id: String,
    pub release_id: String,
    pub treasury_fee: u64,
    pub payment_splits: Vec<PaymentSplit>,
    pub purchase_date: i64,
    pub expiration_date: Option<i64>,
    pub access_type: AccessType,
    pub access_rights: AccessRights,
    pub fee_compensation: Option<u64>,
}
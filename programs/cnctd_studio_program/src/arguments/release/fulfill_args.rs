use anchor_lang::prelude::*;

use crate::{arguments::metadata::Creator};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct FulfillReleaseArgs {
    // Identifiers to find the right escrow
    pub release_id: String,
    pub buyer_id: String,

    // Purchase Info
    pub created_at: i64,
    pub expiration_date: Option<i64>,
    // pub access_type: AccessType,
    
    // NFT metadata
    pub name: String,
    pub symbol: String,
    pub metadata_uri: String,
    pub is_mutable: bool,
    pub seller_fee_basis_points: u16,
    
    // Creator information
    pub creators: Vec<Creator>,

    pub fee_compensation: Option<u64>,
}

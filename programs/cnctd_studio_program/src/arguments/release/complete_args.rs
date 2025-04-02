use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CompleteReleaseArgs {
    // The unique ID of the release
    pub release_id: String,
    
    // The buyer's unique ID
    pub buyer_id: String,
    
    // Optional fee compensation for admin
    pub fee_compensation: Option<u64>,
}

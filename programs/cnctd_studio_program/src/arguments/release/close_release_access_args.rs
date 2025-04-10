use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CloseReleaseAccessArgs {
    pub buyer_id: String,
    pub release_id: String,
    pub fee_compensation: Option<u64>,
}

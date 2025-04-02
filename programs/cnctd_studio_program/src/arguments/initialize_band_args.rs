use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct InitializeBandArgs {
    pub band_id: String,
    pub fee_compensation: Option<u64>,
}
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CloseEscrowArgs {
    pub buyer_id: String,
    pub release_id: String,
}
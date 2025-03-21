use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct InitializeUserArgs {
    pub user_id: String,
    pub fee_compensation: Option<u64>,
    pub usdc_deposit_amount: Option<u64>,
}
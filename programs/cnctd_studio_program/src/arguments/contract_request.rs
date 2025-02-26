use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Credits {
    pub artist_pda: String,
    pub percentage: u8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ContractRequest {
    pub album_id: String,
    pub price_usdc: f64,
    pub credits: Vec<Credits>
}
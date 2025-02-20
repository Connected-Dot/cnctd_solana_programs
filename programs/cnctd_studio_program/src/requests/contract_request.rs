use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Credits {
    pub artist_id: String,
    pub writing: f64,
    pub performance: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ContractRequest {
    pub album_id: String,
    pub price: f64,
    pub credits: Vec<Credits>
}
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PaymentSplit {
    pub recipient: Pubkey,  // ATA to receive payment
    pub amount: u64,        // Pre-calculated amount in USDC lamports
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PurchaseReleaseArgs {
    // Basic Album Info
    pub release_id: String,
    pub name: String,
    pub description: Option<String>,
    pub image_url: String,
    pub metadata_uri: String,
    pub is_mutable: bool,
    pub seller_fee_basis_points: u16,
    
    // Minting Info
    pub edition_count: u32,
    
    // Payment Information (pre-calculated by server)
    pub treasury_fee: u64,
    pub payment_splits: Vec<PaymentSplit>,
    
    // Optional Extra Metadata
    pub band_id: Option<String>,
    pub release_date: Option<i64>,

    pub fee_compensation: Option<u64>,
    pub cnctd_reward: Option<u64>,
    pub music_reward: Option<u64>,
}
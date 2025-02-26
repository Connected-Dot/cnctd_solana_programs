use anchor_lang::prelude::*;

#[account]
pub struct Split {
    pub artist_pda: Pubkey, // PDA of the artist
    pub percentage: u8, // Percentage of the split (must sum to 100)
}

#[derive(Debug)]
#[account]
pub struct MintAlbumArgs {
    // Basic Album Info
    pub album_id: String,
    pub name: String,
    pub description: Option<String>,
    pub image_url: String,
    pub metadata_uri: String,
    pub is_mutable: bool,
    pub seller_fee_basis_points: u16,

    // Minting Info
    pub edition_count: u32,
    pub price_usdc: u64,

    // Revenue Split Details
    pub is_band_release: bool,
    pub performance_splits: Vec<Split>,
    pub writing_splits: Vec<Split>,
    pub treasury_cut_basis_points: u16,

    // Optional Extra Metadata
    pub band_id: Option<String>,
    pub release_date: Option<i64>,
}

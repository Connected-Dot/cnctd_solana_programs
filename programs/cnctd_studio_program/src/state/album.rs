use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MintAlbum<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,  // Wallet of the person calling this
}

#[account]
pub struct Album {
    pub album_id: String,    // Unique album identifier
    pub creator: Pubkey,     // Wallet that created the album (original owner)
    pub price: u64,          // Price in USDC (amount in lamports)
    pub credits: Vec<CreditSplit>, // Artist payout splits
    // pub resale_allowed: bool, // Whether resale is allowed
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreditSplit {
    pub artist_wallet: Pubkey,  // Artist’s wallet address
    pub percentage: u8,         // Percentage share (0-100)
}


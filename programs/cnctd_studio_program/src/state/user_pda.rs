use anchor_lang::prelude::*;

#[account]
pub struct UserPDA {
    pub admin: Pubkey,        // Always your program's treasury PDA
    pub auth: Option<Pubkey>,        // User's wallet (can change ATAs)
    pub usdc_ata: Pubkey,      // Program-derived ATA (default)
    pub usdc_cust: Option<Pubkey>, // User’s preferred ATA (if set)
    pub cnctd_ata: Pubkey,     // Program-derived ATA (default)
    pub cnctd_cust: Option<Pubkey>, // User’s preferred ATA (if set)
    pub music_ata: Pubkey,     // Program-derived ATA for MUS1C rewards
    pub music_cust: Option<Pubkey>, // User’s preferred ATA for MUS1C rewards
    pub fees_waived: u64, // Fees that have been waived
    pub waived_count: u64, // Number of waived transactions
    pub bump: u8, // PDA bump seed
}
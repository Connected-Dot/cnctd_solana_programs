use anchor_lang::prelude::*;

#[account]
pub struct BandPDA {
    pub admin: Pubkey,        // Always your program's treasury PDA
    pub usdc_ata: Pubkey,      // Program-derived ATA (default)
    pub cnctd_ata: Pubkey,     // Program-derived ATA (default)
    pub fees_waived: u64, // Fees that have been waived
    pub waived_count: u64, // Number of waived transactions
    pub bump: u8, // PDA bump seed
}
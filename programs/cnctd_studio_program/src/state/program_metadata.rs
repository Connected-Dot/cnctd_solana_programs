use anchor_lang::prelude::*;

#[account]
pub struct ProgramMetadata {
    pub owner: Pubkey,       // Who owns the program (can modify settings)
    pub version: u8,         // Program version
    pub treasury_pda: Pubkey, // Treasury account
    pub updated_at: i64,     // Last modification timestamp
    pub bump: u8,            // PDA bump seed
}
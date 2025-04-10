use anchor_lang::prelude::*;

#[account]
pub struct ReleaseAccess {
    pub release_id: String,
    pub buyer_id: String,
    pub created_at: i64,
    pub expiration_date: Option<i64>,
    // pub bump: u8,
}

impl ReleaseAccess {
    // Calculate space needed for the account
    pub fn space() -> usize {
        // Use a more generous allocation approach
        let size = 8 + // discriminator
            32 + // release_id (max)
            32 + // buyer_id (max)
            8 + // created_at (i64 timestamp)
            9 + // expiration_date (Option<i64>): 1 for variant + 8 for value
            // 1 + // bump
            16; // Extra padding for safety
        size
    }
}
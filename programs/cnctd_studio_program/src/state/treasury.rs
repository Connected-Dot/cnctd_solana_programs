use anchor_lang::prelude::*;

#[account]
pub struct Treasury {
    pub admins: Vec<Pubkey>, // Multiple admins
    pub bump: u8, 
    pub version: u8,
}

impl Treasury {
    pub fn is_admin(&self, pubkey: &Pubkey) -> bool {
        self.admins.iter().any(|admin| admin == pubkey)
    }
}
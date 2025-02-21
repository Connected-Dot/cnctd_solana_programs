use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;
pub mod constants;

use instructions::*;

declare_id!("4X4ghmVDL7B29vNns8qizXsMUcsE8TZi1EvPewLsPkrN");

#[program]
pub mod cnctd_solana_program {
    use super::*;

    pub fn initialize_treasury(ctx: Context<InitializeTreasury>, initial_admins: Option<Vec<Pubkey>>) -> Result<()> {
        instructions::initialize_treasury::handler(ctx, initial_admins)
    }
}


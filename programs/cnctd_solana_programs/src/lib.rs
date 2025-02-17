use anchor_lang::prelude::*;

declare_id!("AsE3BweZsNJa2oT6sbvNh1UXmLLJmcfYY1hvGJXL9T8L");

#[program]
pub mod cnctd_solana_programs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
